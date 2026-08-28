use crate::item::Item;
use crate::store::write_atomic;
use anyhow::Result;
use chrono::{Datelike, NaiveDate, Utc};
use std::collections::BTreeSet;
use std::path::Path;

const MOIS: [&str; 12] = [
    "janvier", "février", "mars", "avril", "mai", "juin", "juillet", "août",
    "septembre", "octobre", "novembre", "décembre",
];

pub fn date_fr(d: NaiveDate) -> String {
    format!("{} {} {}", d.day(), MOIS[(d.month() - 1) as usize], d.year())
}

/// Échappe ce qui casserait un lien Markdown ou un front-matter TOML.
fn md(s: &str) -> String {
    s.replace('[', "\\[").replace(']', "\\]")
}

fn toml_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn bullet(it: &Item) -> String {
    let mut line = format!("- [{}]({})", md(&it.title), it.url);
    line.push_str(&format!("  \n  <sub>`{}`", it.source_id));
    if !it.tags.is_empty() {
        line.push_str(&format!(" · {}", it.tags.join(", ")));
    }
    line.push_str(&format!(" · score {}</sub>", it.score));
    if let Some(s) = &it.summary {
        line.push_str(&format!("  \n  {}", md(s)));
    }
    line
}

/// Page Zola du jour : `content/digests/YYYY-MM-DD.md`.
pub fn digest(content_dir: &Path, items: &[Item]) -> Result<Option<String>> {
    if items.is_empty() {
        return Ok(None);
    }
    let today = Utc::now().date_naive();
    let tags: BTreeSet<&str> = items.iter().flat_map(|i| i.tags.iter()).map(String::as_str).collect();

    let mut out = String::new();
    out.push_str("+++\n");
    out.push_str(&format!("title = \"Veille Rust — {}\"\n", toml_str(&date_fr(today))));
    out.push_str(&format!("date = {today}\n"));
    out.push_str("[taxonomies]\n");
    out.push_str(&format!(
        "tags = [{}]\n",
        tags.iter().map(|t| format!("\"{}\"", toml_str(t))).collect::<Vec<_>>().join(", ")
    ));
    out.push_str("+++\n\n");
    out.push_str(&format!(
        "{} item{} collecté{} ce jour.\n\n",
        items.len(),
        if items.len() > 1 { "s" } else { "" },
        if items.len() > 1 { "s" } else { "" }
    ));

    // Regroupé par tag principal : plus lisible qu'une liste plate de 40 liens.
    let mut groups: std::collections::BTreeMap<&str, Vec<&Item>> = Default::default();
    for it in items {
        groups.entry(it.tags.first().map(String::as_str).unwrap_or("divers")).or_default().push(it);
    }
    for (tag, mut group) in groups {
        group.sort_by(|a, b| b.score.cmp(&a.score));
        out.push_str(&format!("## {tag}\n\n"));
        for it in group {
            out.push_str(&bullet(it));
            out.push_str("\n\n");
        }
    }

    let path = content_dir.join("digests").join(format!("{today}.md"));
    write_atomic(&path, out.as_bytes())?;
    Ok(Some(path.display().to_string()))
}

/// README = vitrine du repo : le top de la semaine, régénéré à chaque run.
pub fn readme(path: &Path, recent: &[Item], total_sources: usize) -> Result<()> {
    let today = Utc::now().date_naive();
    let mut out = String::new();
    out.push_str("# 🦀 Veille Rust\n\n");
    out.push_str(&format!(
        "Veille automatisée sur l'écosystème Rust : {total_sources} sources agrégées chaque jour \
         par un collecteur écrit en Rust, tournant sur GitHub Actions.\n\n\
         **[→ Consulter le site](https://olivierfalahi.github.io/veille-rust/)** · \
         [flux RSS](https://olivierfalahi.github.io/veille-rust/atom.xml) · \
         [archives](content/digests/) · [mes notes](notes/)\n\n---\n\n"
    ));
    out.push_str(&format!("## Top de la semaine — {}\n\n", date_fr(today)));

    if recent.is_empty() {
        out.push_str("_Rien de neuf cette semaine._\n");
    } else {
        for it in recent.iter().take(10) {
            out.push_str(&bullet(it));
            out.push('\n');
        }
    }

    out.push_str(&format!(
        "\n---\n\n## Fonctionnement\n\n\
        - `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**\n\
        - `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.\n\
        - `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.\n\
        - `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.\n\
        - `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.\n\n\
        Collecte quotidienne à 06:17 UTC · dernière mise à jour {}\n",
        Utc::now().format("%Y-%m-%d %H:%M UTC")
    ));

    write_atomic(path, out.as_bytes())
}

/// Payload consommé par les steps de notification du workflow.
pub fn latest_json(path: &Path, items: &[Item]) -> Result<()> {
    let top: Vec<_> = {
        let mut v: Vec<&Item> = items.iter().collect();
        v.sort_by(|a, b| b.score.cmp(&a.score));
        v.into_iter().take(10).collect()
    };
    let payload = serde_json::json!({
        "generated_at": Utc::now(),
        "count": items.len(),
        "top": top,
    });
    write_atomic(path, serde_json::to_string_pretty(&payload)?.as_bytes())
}

/// Corps de l'issue hebdomadaire.
pub fn weekly(path: &Path, recent: &[Item]) -> Result<()> {
    let today = Utc::now().date_naive();
    let mut out = format!(
        "Récap de la semaine écoulée, arrêté au {}.\n\n",
        date_fr(today)
    );

    if recent.is_empty() {
        out.push_str("Rien de neuf cette semaine.\n");
    } else {
        out.push_str(&format!("**{} items** collectés. Top 15 :\n\n", recent.len()));
        // Cases à cocher : l'issue devient une liste de lecture.
        for it in recent.iter().take(15) {
            out.push_str(&format!(
                "- [ ] [{}]({}) — `{}` · score {}\n",
                md(&it.title),
                it.url,
                it.source_id,
                it.score
            ));
        }
        out.push_str(&format!(
            "\n<details><summary>Les {} autres</summary>\n\n",
            recent.len().saturating_sub(15)
        ));
        for it in recent.iter().skip(15) {
            out.push_str(&format!("- [{}]({}) — `{}`\n", md(&it.title), it.url, it.source_id));
        }
        out.push_str("\n</details>\n");
    }

    out.push_str("\n---\n_Généré automatiquement. Ferme l'issue quand tu as fait le tri._\n");
    write_atomic(path, out.as_bytes())
}
