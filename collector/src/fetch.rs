use crate::config::{Config, Kind, Source};
use crate::item::{clean_text, id_for, truncate, Item};
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use serde::Deserialize;

pub struct Client {
    http: reqwest::Client,
}

pub struct Probe {
    pub status: u16,
    pub reason: String,
    pub content_type: String,
    /// Après redirections : révèle un flux qui a déménagé.
    pub final_url: String,
    pub bytes: usize,
}

impl Client {
    pub fn new(user_agent: &str) -> Result<Self> {
        let http = reqwest::Client::builder()
            .user_agent(user_agent)
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        Ok(Self { http })
    }

    pub async fn collect(&self, cfg: &Config, src: &Source) -> Result<Vec<Item>> {
        let mut items = match src.kind {
            Kind::Feed => self.feed(cfg, src).await?,
            Kind::HnAlgolia => self.hn(cfg, src).await?,
            Kind::CratesIo => self.crates_io(cfg, src).await?,
        };
        items.sort_by(|a, b| b.published.cmp(&a.published));
        if let Some(limit) = src.limit {
            items.truncate(limit);
        }
        Ok(items)
    }

    /// Faits HTTP bruts, pour le diagnostic `--check-source`. Séparé de
    /// `collect()` afin de pouvoir rapporter le statut même quand le parsing
    /// échoue ensuite.
    pub async fn probe(&self, url: &str) -> Result<Probe> {
        let resp = self.http.get(url).send().await?;
        let status = resp.status();
        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("(absent)")
            .to_string();
        let final_url = resp.url().to_string();
        Ok(Probe {
            status: status.as_u16(),
            reason: status.canonical_reason().unwrap_or("").to_string(),
            content_type,
            final_url,
            bytes: resp.bytes().await?.len(),
        })
    }

    async fn bytes(&self, url: &str) -> Result<Vec<u8>> {
        let resp = self.http.get(url).send().await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(anyhow!("HTTP {status}"));
        }
        Ok(resp.bytes().await?.to_vec())
    }

    async fn feed(&self, cfg: &Config, src: &Source) -> Result<Vec<Item>> {
        let url = src.url.as_deref().expect("validé au chargement de la config");
        let body = self.bytes(url).await?;
        let feed = feed_rs::parser::parse(&body[..]).context("flux illisible")?;

        let now = Utc::now();
        let mut out = Vec::new();
        for e in feed.entries {
            let Some(link) = e.links.first().map(|l| l.href.clone()) else {
                continue;
            };
            let title = e
                .title
                .map(|t| clean_text(&t.content))
                .unwrap_or_else(|| "(sans titre)".into());
            let summary = e
                .summary
                .map(|s| clean_text(&s.content))
                .or_else(|| e.content.and_then(|c| c.body).map(|b| clean_text(&b)))
                .filter(|s| !s.is_empty())
                .map(|s| truncate(&s, 320));

            let scored = format!("{title} {}", summary.clone().unwrap_or_default());
            out.push(Item {
                id: id_for(&link),
                url: link,
                score: src.weight * 10 + cfg.boost_for(&scored),
                title,
                source_id: src.id.clone(),
                tags: src.tags.clone(),
                summary,
                published: e.published.or(e.updated),
                first_seen: now,
            });
        }
        Ok(out)
    }

    async fn hn(&self, cfg: &Config, src: &Source) -> Result<Vec<Item>> {
        #[derive(Deserialize)]
        struct Resp {
            hits: Vec<Hit>,
        }
        #[derive(Deserialize)]
        struct Hit {
            #[serde(rename = "objectID")]
            object_id: String,
            title: Option<String>,
            url: Option<String>,
            points: Option<i64>,
            created_at: Option<chrono::DateTime<Utc>>,
        }

        let query = src.query.as_deref().expect("validé au chargement");
        let endpoint = format!(
            "https://hn.algolia.com/api/v1/search_by_date?query={}&tags=story&hitsPerPage=50",
            urlencoding(query)
        );
        let resp: Resp = serde_json::from_slice(&self.bytes(&endpoint).await?)?;

        let floor = src.min_points.unwrap_or(0);
        let now = Utc::now();
        Ok(resp
            .hits
            .into_iter()
            .filter(|h| h.points.unwrap_or(0) >= floor)
            .filter_map(|h| {
                let title = clean_text(&h.title?);
                // Une story sans URL est un "Ask HN" : on pointe vers le fil.
                let url = h.url.unwrap_or_else(|| {
                    format!("https://news.ycombinator.com/item?id={}", h.object_id)
                });
                let points = h.points.unwrap_or(0);
                Some(Item {
                    id: id_for(&url),
                    url,
                    score: src.weight * 10 + cfg.boost_for(&title) + points / 10,
                    summary: Some(format!("{points} points sur Hacker News")),
                    title,
                    source_id: src.id.clone(),
                    tags: src.tags.clone(),
                    published: h.created_at,
                    first_seen: now,
                })
            })
            .collect())
    }

    async fn crates_io(&self, cfg: &Config, src: &Source) -> Result<Vec<Item>> {
        // `#[serde(default)]` partout : crates.io fait évoluer ce payload
        // (`most_recently_updated` est devenu `just_updated`), un champ disparu
        // ne doit pas casser la collecte.
        #[derive(Deserialize)]
        struct Summary {
            #[serde(default)]
            new_crates: Vec<Crate>,
            #[serde(default)]
            just_updated: Vec<Crate>,
        }
        #[derive(Default, Deserialize)]
        struct Crate {
            name: String,
            description: Option<String>,
            max_stable_version: Option<String>,
            max_version: Option<String>,
            downloads: Option<i64>,
            updated_at: Option<chrono::DateTime<Utc>>,
        }

        // crates.io rejette les requêtes sans User-Agent identifiable (403).
        let s: Summary =
            serde_json::from_slice(&self.bytes("https://crates.io/api/v1/summary").await?)?;

        let floor = src.min_downloads.unwrap_or(0);
        let now = Utc::now();
        let mut out = Vec::new();
        for (c, kind) in s
            .new_crates
            .into_iter()
            .map(|c| (c, "nouveau"))
            .chain(s.just_updated.into_iter().map(|c| (c, "mis à jour")))
        {
            if c.downloads.unwrap_or(0) < floor {
                continue;
            }
            let url = format!("https://crates.io/crates/{}", c.name);
            let version = c.max_stable_version.or(c.max_version).unwrap_or_else(|| "?".into());
            let title = format!("{} {version} ({kind})", c.name);
            let desc = c.description.map(|d| truncate(&clean_text(&d), 200));
            let scored = format!("{title} {}", desc.clone().unwrap_or_default());
            // Un crate très téléchargé mérite plus d'attention qu'un crate neuf anonyme.
            let popularity = match c.downloads.unwrap_or(0) {
                d if d > 1_000_000 => 20,
                d if d > 100_000 => 12,
                d if d > 10_000 => 6,
                _ => 0,
            };
            out.push(Item {
                id: id_for(&format!("{url}#{version}")),
                url,
                score: src.weight * 10 + cfg.boost_for(&scored) + popularity,
                title,
                source_id: src.id.clone(),
                tags: src.tags.clone(),
                summary: desc,
                published: c.updated_at,
                first_seen: now,
            });
        }
        Ok(out)
    }
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            c => c
                .to_string()
                .bytes()
                .map(|b| format!("%{b:02X}"))
                .collect::<String>(),
        })
        .collect()
}
