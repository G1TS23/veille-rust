use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub user_agent: String,
    #[serde(default = "default_min_score")]
    pub min_score: i64,
    #[serde(default)]
    pub boost: Vec<Boost>,
    pub source: Vec<Source>,
}

fn default_min_score() -> i64 {
    0
}

#[derive(Debug, Deserialize)]
pub struct Boost {
    pub pattern: String,
    pub points: i64,
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub id: String,
    pub kind: Kind,
    /// Requis pour `kind = "feed"`.
    pub url: Option<String>,
    /// Requis pour `kind = "hn_algolia"`.
    pub query: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub weight: i64,
    /// Nombre max d'items retenus par run (les flux "bavards" en ont besoin).
    pub limit: Option<usize>,
    /// Seuil de points HN en dessous duquel l'item est ignoré.
    pub min_points: Option<i64>,
    /// Seuil de téléchargements crates.io. Sans lui, `just_updated` déverse
    /// des dizaines de crates anonymes par run.
    pub min_downloads: Option<i64>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    /// RSS 0.9x/1.0/2.0, Atom ou JSON Feed — feed-rs les distingue tout seul.
    Feed,
    HnAlgolia,
    CratesIo,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let raw = std::fs::read_to_string(path)
            .with_context(|| format!("lecture de {}", path.display()))?;
        let cfg: Config =
            toml::from_str(&raw).with_context(|| format!("parsing de {}", path.display()))?;

        for s in &cfg.source {
            match s.kind {
                Kind::Feed if s.url.is_none() => {
                    anyhow::bail!("source `{}` : `url` est requis pour kind = \"feed\"", s.id)
                }
                Kind::HnAlgolia if s.query.is_none() => anyhow::bail!(
                    "source `{}` : `query` est requis pour kind = \"hn_algolia\"",
                    s.id
                ),
                _ => {}
            }
        }
        Ok(cfg)
    }

    /// Avertit sur les sources dont le score de base ne franchit pas `min_score`.
    ///
    /// On compare le **score de base** (`weight` × 10) et non un plafond
    /// théorique incluant tous les bonus : ce plafond dépasse presque toujours
    /// `min_score`, ce qui ne signalerait jamais rien. Un item peut encore
    /// passer en cumulant des bonus de mots-clés, d'où « ne remontera que ».
    ///
    /// Le cas courant est un `weight` oublié : serde lui donne `0`, et la
    /// source affiche un `✓` vert avec 0 item sans qu'aucune erreur ne sorte.
    pub fn warn_unreachable(&self) -> usize {
        let mut n = 0;
        for s in &self.source {
            // HN ajoute points/10, sans plafond : impossible d'affirmer quoi
            // que ce soit sur son score final.
            if s.kind == Kind::HnAlgolia {
                continue;
            }
            let base = s.weight * 10;
            if base >= self.min_score {
                continue;
            }
            n += 1;
            if s.weight == 0 {
                eprintln!(
                    "  ⚠ `{}` : `weight` absent ou nul → score de base 0, sous min_score {}. \
                     Cette source sera collectée puis entièrement filtrée.",
                    s.id, self.min_score
                );
            } else {
                eprintln!(
                    "  ⚠ `{}` : score de base {base} < min_score {} — ne remontera que \
                     les items cumulant des bonus de mots-clés.",
                    s.id, self.min_score
                );
            }
        }
        n
    }

    /// Bonus de score cumulé pour tous les motifs présents dans le texte.
    pub fn boost_for(&self, text: &str) -> i64 {
        let haystack = text.to_lowercase();
        self.boost
            .iter()
            .filter(|b| haystack.contains(&b.pattern.to_lowercase()))
            .map(|b| b.points)
            .sum()
    }
}
