use crate::item::Item;
use crate::store::write_atomic;
use anyhow::{Context, Result};
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

    let last_find = recent.iter().map(|i| i.first_seen).max();
    out.push_str(&format!(
        "\n---\n\n## Fonctionnement\n\n\
        - `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**\n\
        - `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.\n\
        - `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.\n\
        - `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.\n\
        - `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.\n\n\
        Collecte quotidienne à 06:17 UTC · {}\n",
        // Surtout pas `Utc::now()` : un horodatage de run rendrait le README
        // différent à chaque passage, donc un commit par jour même sans contenu.
        match last_find {
            Some(t) => format!("dernière trouvaille {}", t.format("%Y-%m-%d %H:%M UTC")),
            None => "rien de neuf sur les 7 derniers jours".to_string(),
        }
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

/// Corps de l'issue hebdomadaire, rendu depuis `collector/templates/weekly.md`.
///
/// Le template est lu à l'exécution (et non `include_str!`) : tu peux retoucher
/// le texte de l'issue sans recompiler le collecteur.
pub fn weekly(path: &Path, tpl_path: &Path, recent: &[Item]) -> Result<()> {
    const TOP: usize = 15;

    let raw = std::fs::read_to_string(tpl_path)
        .with_context(|| format!("template introuvable : {}", tpl_path.display()))?;

    // Les lignes de commentaire disparaissent avant tout le reste, sinon leurs
    // exemples de placeholders seraient eux-mêmes substitués.
    let mut tpl: String = raw
        .lines()
        .filter(|l| !l.trim_start().starts_with("{{!"))
        .collect::<Vec<_>>()
        .join("\n");

    // Les régions `item` / `rest_item` ne sont pas du contenu : ce sont les
    // formats d'une ligne. On les retire du corps avant de le rendre.
    let item_tpl = take_region(&mut tpl, "item")
        .unwrap_or_else(|| "- [ ] [{{title}}]({{url}}) — `{{source}}`".into());
    let rest_tpl = take_region(&mut tpl, "rest_item").unwrap_or_else(|| item_tpl.clone());

    let top: Vec<&Item> = recent.iter().take(TOP).collect();
    let rest: Vec<&Item> = recent.iter().skip(TOP).collect();

    keep_region(&mut tpl, "has_items", !recent.is_empty());
    keep_region(&mut tpl, "empty", recent.is_empty());
    keep_region(&mut tpl, "has_rest", !rest.is_empty());

    let lines = |items: &[&Item], line: &str| {
        items.iter().map(|it| fill_item(line, it)).collect::<Vec<_>>().join("\n")
    };

    let out = tpl
        .replace("{{date}}", &date_fr(Utc::now().date_naive()))
        .replace("{{count}}", &recent.len().to_string())
        .replace("{{top_count}}", &top.len().to_string())
        .replace("{{rest_count}}", &rest.len().to_string())
        .replace("{{items}}", &lines(&top, &item_tpl))
        .replace("{{rest}}", &lines(&rest, &rest_tpl));

    write_atomic(path, squeeze(&out).as_bytes())
}

fn fill_item(line: &str, it: &Item) -> String {
    line.replace("{{title}}", &md(&it.title))
        .replace("{{url}}", &it.url)
        .replace("{{source}}", &it.source_id)
        .replace("{{score}}", &it.score.to_string())
        .replace("{{tags}}", &it.tags.join(", "))
        .replace("{{summary}}", &md(it.summary.as_deref().unwrap_or("")))
}

/// Retire `[[#nom]]…[[/nom]]` du template et renvoie son contenu.
fn take_region(tpl: &mut String, name: &str) -> Option<String> {
    let (start, end, inner) = locate_region(tpl, name)?;
    tpl.replace_range(start..end, "");
    Some(inner)
}

/// Garde le contenu de la région (marqueurs retirés) ou supprime le tout.
fn keep_region(tpl: &mut String, name: &str, keep: bool) {
    while let Some((start, end, inner)) = locate_region(tpl, name) {
        tpl.replace_range(start..end, if keep { inner.trim_matches('\n') } else { "" });
    }
}

/// Bornes de la région et contenu entre les marqueurs.
fn locate_region(tpl: &str, name: &str) -> Option<(usize, usize, String)> {
    let open = format!("[[#{name}]]");
    let close = format!("[[/{name}]]");
    let start = tpl.find(&open)?;
    let inner_start = start + open.len();
    let inner_end = tpl[inner_start..].find(&close)? + inner_start;
    Some((start, inner_end + close.len(), tpl[inner_start..inner_end].to_string()))
}

/// Les régions supprimées laissent des trous : on ramène les lignes vides
/// consécutives à une seule.
fn squeeze(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut blanks = 0;
    for line in s.lines() {
        if line.trim().is_empty() {
            blanks += 1;
            if blanks > 1 {
                continue;
            }
        } else {
            blanks = 0;
        }
        out.push_str(line);
        out.push('\n');
    }
    format!("{}\n", out.trim_end())
}
