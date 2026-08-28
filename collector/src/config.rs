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
