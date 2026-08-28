use crate::item::Item;
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashSet;
use std::io::Write;
use std::path::{Path, PathBuf};

/// L'état vit en JSONL et non en SQLite : les diffs git restent lisibles et
/// réparables à la main, ce qui n'est pas le cas d'un binaire.
pub struct Store {
    root: PathBuf,
}

/// Au-delà, un item sort de l'index de dédup — sinon `seen.jsonl` gonfle sans fin.
const RETENTION_DAYS: i64 = 400;

impl Store {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn seen_path(&self) -> PathBuf {
        self.root.join("seen.jsonl")
    }

    /// Index de dédup : (id, first_seen) pour tout ce qu'on a déjà publié.
    pub fn load_seen(&self) -> Result<Vec<(String, DateTime<Utc>)>> {
        let path = self.seen_path();
        if !path.exists() {
            return Ok(Vec::new());
        }
        let raw = std::fs::read_to_string(&path)?;
        Ok(raw
            .lines()
            .filter(|l| !l.trim().is_empty())
            .filter_map(|l| serde_json::from_str::<SeenRow>(l).ok())
            .map(|r| (r.id, r.first_seen))
            .collect())
    }

    pub fn save_seen(&self, rows: &[(String, DateTime<Utc>)]) -> Result<()> {
        let cutoff = Utc::now() - Duration::days(RETENTION_DAYS);
        let mut buf = String::new();
        for (id, first_seen) in rows.iter().filter(|(_, t)| *t >= cutoff) {
            buf.push_str(&serde_json::to_string(&SeenRow {
                id: id.clone(),
                first_seen: *first_seen,
            })?);
            buf.push('\n');
        }
        write_atomic(&self.seen_path(), buf.as_bytes())
    }

    /// Archive brute, un fichier par mois : `data/items/2026-08.jsonl`.
    pub fn append_archive(&self, items: &[Item]) -> Result<()> {
        if items.is_empty() {
            return Ok(());
        }
        let dir = self.root.join("items");
        std::fs::create_dir_all(&dir)?;
        let path = dir.join(format!("{}.jsonl", Utc::now().format("%Y-%m")));
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .with_context(|| format!("ouverture de {}", path.display()))?;
        for it in items {
            writeln!(f, "{}", serde_json::to_string(it)?)?;
        }
        Ok(())
    }

    /// Tous les items vus aujourd'hui, relus depuis l'archive. Indispensable :
    /// si le collecteur tourne deux fois dans la journée, le second run ne doit
    /// pas écraser le digest avec ses seules nouveautés.
    pub fn today(&self) -> Result<Vec<Item>> {
        let today = Utc::now().date_naive();
        Ok(self
            .recent(2)?
            .into_iter()
            .filter(|i| i.first_seen.date_naive() == today)
            .collect())
    }

    /// Relit les archives pour reconstruire une fenêtre glissante (README hebdo).
    pub fn recent(&self, days: i64) -> Result<Vec<Item>> {
        let dir = self.root.join("items");
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let cutoff = Utc::now() - Duration::days(days);
        let mut seen_ids = HashSet::new();
        let mut out = Vec::new();

        let mut files: Vec<_> = std::fs::read_dir(&dir)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|e| e == "jsonl"))
            .collect();
        files.sort();
        // Deux mois suffisent à couvrir n'importe quelle fenêtre raisonnable.
        for path in files.iter().rev().take(2) {
            for line in std::fs::read_to_string(path)?.lines() {
                let Ok(it) = serde_json::from_str::<Item>(line) else {
                    continue;
                };
                if it.first_seen >= cutoff && seen_ids.insert(it.id.clone()) {
                    out.push(it);
                }
            }
        }
        out.sort_by(|a, b| b.score.cmp(&a.score).then(b.first_seen.cmp(&a.first_seen)));
        Ok(out)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SeenRow {
    id: String,
    first_seen: DateTime<Utc>,
}

/// Écrit via un fichier temporaire puis renomme : un run interrompu ne laisse
/// jamais un `seen.jsonl` tronqué (ce qui rejouerait toute la veille).
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, bytes)?;
    std::fs::rename(&tmp, path).with_context(|| format!("écriture de {}", path.display()))?;
    Ok(())
}
