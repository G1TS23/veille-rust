use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// SHA-256 (16 octets hex) de l'URL canonisée. C'est la clé de dédup.
    pub id: String,
    pub url: String,
    pub title: String,
    pub source_id: String,
    pub tags: Vec<String>,
    pub score: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<DateTime<Utc>>,
    pub first_seen: DateTime<Utc>,
}

/// Paramètres de tracking à jeter avant de hasher : sans ça, le même article
/// vu via deux sources différentes compte comme deux items.
const JUNK_PARAMS: &[&str] = &[
    "utm_source",
    "utm_medium",
    "utm_campaign",
    "utm_term",
    "utm_content",
    "ref",
    "ref_src",
    "fbclid",
    "gclid",
    "mc_cid",
    "mc_eid",
];

/// Canonise une URL pour la dédup : host en minuscules, paramètres de tracking
/// supprimés, fragment supprimé, slash final supprimé.
pub fn canonicalize(raw: &str) -> String {
    let Ok(mut u) = Url::parse(raw.trim()) else {
        return raw.trim().to_lowercase();
    };

    u.set_fragment(None);

    if let Some(host) = u.host_str() {
        let lowered = host.to_lowercase();
        let stripped = lowered.strip_prefix("www.").unwrap_or(&lowered).to_string();
        let _ = u.set_host(Some(&stripped));
    }

    let kept: Vec<(String, String)> = u
        .query_pairs()
        .filter(|(k, _)| !JUNK_PARAMS.contains(&k.as_ref()))
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();
    if kept.is_empty() {
        u.set_query(None);
    } else {
        let mut qs = u.query_pairs_mut();
        qs.clear();
        for (k, v) in &kept {
            qs.append_pair(k, v);
        }
        drop(qs);
    }

    let s = u.to_string();
    s.strip_suffix('/').unwrap_or(&s).to_string()
}

pub fn id_for(raw_url: &str) -> String {
    let mut h = Sha256::new();
    h.update(canonicalize(raw_url).as_bytes());
    // sha2 0.11 renvoie un `Array` sans impl LowerHex : on formate à la main.
    h.finalize().iter().take(8).map(|b| format!("{b:02x}")).collect()
}

/// Nettoie un titre/résumé venu d'un flux : balises HTML grossièrement retirées,
/// entités courantes décodées, espaces normalisés.
pub fn clean_text(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_tag = false;
    for c in input.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    let out = out
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ");
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let cut: String = s.chars().take(max).collect();
    // Coupe sur le dernier espace pour ne pas trancher un mot en deux.
    match cut.rfind(' ') {
        Some(i) if i > max / 2 => format!("{}…", &cut[..i]),
        _ => format!("{cut}…"),
    }
}
