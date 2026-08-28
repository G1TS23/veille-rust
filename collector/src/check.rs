use crate::config::{Config, Kind, Source};
use crate::fetch::Client;
use crate::store::Store;
use anyhow::Result;
use std::collections::HashSet;
use std::path::Path;

/// Sonde un flux avant de l'ajouter à `sources.toml` : répond-il, se parse-t-il,
/// que produirait-il concrètement ?
///
/// Volontairement en deux requêtes : la première rapporte les faits HTTP bruts
/// (utile quand le parsing échoue), la seconde emprunte le chemin de production
/// exact, donc ce qui s'affiche ici est ce que la collecte produira.
pub async fn check_source(root: &Path, cfg: &Config, url: &str, weight: i64) -> Result<()> {
    println!("Sonde de {url}\n");

    let client = Client::new(&cfg.user_agent)?;

    let probe = match client.probe(url).await {
        Ok(p) => p,
        Err(e) => {
            println!("  ✗ requête échouée : {e}");
            println!("\n  Causes fréquentes : URL invalide, DNS, TLS, ou un site");
            println!("  qui rejette les requêtes non navigateur.");
            return Ok(());
        }
    };

    let ok = (200..300).contains(&probe.status);
    println!("  HTTP          {} {} {}", probe.status, probe.reason, if ok { "✓" } else { "✗" });
    println!("  Content-Type  {}", probe.content_type);
    println!("  Taille        {}", human_size(probe.bytes));
    if probe.final_url.trim_end_matches('/') != url.trim_end_matches('/') {
        println!("  ⚠ Redirigé vers {}", probe.final_url);
        println!("    Utilise plutôt cette URL dans sources.toml.");
    }
    if !ok {
        println!("\n  Le serveur n'a pas renvoyé de succès : rien à parser.");
        return Ok(());
    }

    // Source synthétique passée au vrai collecteur.
    let src = Source {
        id: "check".into(),
        kind: Kind::Feed,
        url: Some(url.to_string()),
        query: None,
        tags: vec![],
        weight,
        limit: None,
        min_points: None,
        min_downloads: None,
    };

    let items = match client.collect(cfg, &src).await {
        Ok(i) => i,
        Err(e) => {
            println!("\n  Parsing       ✗ {e}");
            println!("\n  feed-rs n'a pas reconnu ce contenu. Vérifie que l'URL pointe");
            println!("  bien vers le flux et non vers la page HTML qui l'annonce.");
            return Ok(());
        }
    };

    println!("\n  Parsing       ✓");
    println!("  Entrées       {}", items.len());

    if items.is_empty() {
        println!("\n  Le flux se parse mais ne contient aucune entrée exploitable.");
        println!("  Les entrées sans lien sont ignorées par le collecteur.");
        return Ok(());
    }

    if let Some(d) = items.iter().filter_map(|i| i.published).max() {
        let age = (chrono::Utc::now() - d).num_days();
        println!("  Plus récente  {} ({age} j)", d.format("%Y-%m-%d"));
        if age > 180 {
            println!("  ⚠ Flux probablement abandonné.");
        }
    } else {
        println!("  ⚠ Aucune date : les entrées ne pourront pas être triées.");
    }

    // Combien seraient déjà connus ? Révèle qu'une source en double une autre.
    let known: HashSet<String> = Store::new(root.join("data"))
        .load_seen()?
        .into_iter()
        .map(|(id, _)| id)
        .collect();
    let dups = items.iter().filter(|i| known.contains(&i.id)).count();
    if dups > 0 {
        println!(
            "  ⚠ {dups}/{} déjà collectés ailleurs — recoupe une source existante.",
            items.len()
        );
    }

    let retenus = items.iter().filter(|i| i.score >= cfg.min_score).count();
    println!(
        "  Score         {} (weight {weight} × 10 + bonus) — {retenus}/{} au-dessus de min_score = {}",
        items.first().map(|i| i.score).unwrap_or(0),
        items.len(),
        cfg.min_score
    );
    if retenus == 0 {
        println!("  ⚠ Rien ne passerait le filtre. Monte `weight` ou baisse `min_score`.");
    }

    println!("\n  Échantillon :");
    for it in items.iter().take(3) {
        println!("    [{:>4}] {}", it.score, it.title);
        println!("           {}", it.url);
    }

    println!("\n  Bloc à coller dans sources.toml :\n");
    println!("[[source]]");
    println!("id     = \"{}\"", suggest_id(url));
    println!("kind   = \"feed\"");
    println!("url    = \"{url}\"");
    println!("tags   = [\"à-remplir\"]");
    println!("weight = {weight}");
    if items.len() > 20 {
        println!("limit  = 15   # flux bavard : {} entrées par appel", items.len());
    }

    Ok(())
}

/// Identifiant lisible dérivé du domaine : blog.rust-lang.org → "blog-rust-lang".
fn suggest_id(url: &str) -> String {
    let host = url::Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(str::to_lowercase))
        .unwrap_or_default();
    let host = host.strip_prefix("www.").unwrap_or(&host);
    let mut parts: Vec<&str> = host.split('.').collect();
    if parts.len() > 1 {
        parts.pop(); // le TLD n'apporte rien
    }
    let id = parts.join("-");
    if id.is_empty() { "ma-source".into() } else { id }
}

fn human_size(n: usize) -> String {
    match n {
        n if n >= 1_048_576 => format!("{:.1} Mo", n as f64 / 1_048_576.0),
        n if n >= 1024 => format!("{:.1} Ko", n as f64 / 1024.0),
        n => format!("{n} o"),
    }
}
