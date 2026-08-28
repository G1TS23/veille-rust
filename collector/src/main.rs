mod check;
mod config;
mod fetch;
mod item;
mod render;
mod store;

use anyhow::Result;
use chrono::Utc;
use config::Config;
use item::Item;
use std::collections::HashSet;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // Le collecteur tourne depuis la racine du repo (workflow) ou depuis
    // collector/ (dev local) : on remonte jusqu'à trouver sources.toml.
    let root = repo_root()?;
    let cfg = Config::load(root.join("sources.toml"))?;
    let args: Vec<String> = std::env::args().collect();
    let dry_run = args.iter().any(|a| a == "--dry-run");

    // Sonde d'un flux candidat : ne touche à rien, n'écrit rien.
    if let Some(i) = args.iter().position(|a| a == "--check-source") {
        let Some(url) = args.get(i + 1) else {
            anyhow::bail!("usage : --check-source <url> [--weight N]");
        };
        let weight = args
            .iter()
            .position(|a| a == "--weight")
            .and_then(|w| args.get(w + 1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);
        return check::check_source(&root, &cfg, url, weight).await;
    }

    // Mode récap hebdo : on ne collecte rien, on relit l'archive.
    if args.iter().any(|a| a == "--weekly") {
        let store = store::Store::new(root.join("data"));
        let path = root.join("data").join("weekly.md");
        render::weekly(
            &path,
            &root.join("collector").join("templates").join("weekly.md"),
            &store.recent(7)?,
        )?;
        println!("récap hebdo écrit : {}", path.display());
        return Ok(());
    }

    let client = fetch::Client::new(&cfg.user_agent)?;
    let store = store::Store::new(root.join("data"));

    let mut seen = store.load_seen()?;
    let known: HashSet<String> = seen.iter().map(|(id, _)| id.clone()).collect();

    let mut fresh: Vec<Item> = Vec::new();
    let mut batch_ids: HashSet<String> = HashSet::new();
    let mut failures = 0usize;

    for src in &cfg.source {
        // Chaque source est isolée : une source morte ne doit jamais faire
        // échouer le run entier (Reddit renvoie régulièrement 403 en CI).
        match client.collect(&cfg, src).await {
            Ok(items) => {
                let before = fresh.len();
                for it in items {
                    if it.score < cfg.min_score {
                        continue;
                    }
                    // `batch_ids` attrape le même article publié par deux sources
                    // dans le même run — `known` attrape ceux des runs passés.
                    if known.contains(&it.id) || !batch_ids.insert(it.id.clone()) {
                        continue;
                    }
                    fresh.push(it);
                }
                eprintln!("  ✓ {:<16} {} nouveau(x)", src.id, fresh.len() - before);
            }
            Err(e) => {
                failures += 1;
                eprintln!("  ✗ {:<16} {e}", src.id);
            }
        }
    }

    fresh.sort_by(|a, b| b.score.cmp(&a.score).then(b.published.cmp(&a.published)));

    println!(
        "\n{} item(s) retenu(s) sur {} sources ({} en échec)",
        fresh.len(),
        cfg.source.len(),
        failures
    );

    if dry_run {
        for it in fresh.iter().take(20) {
            println!("  [{:>4}] {} — {}", it.score, it.title, it.source_id);
        }
        println!("\n--dry-run : rien n'a été écrit sur le disque.");
        return Ok(());
    }

    let now = Utc::now();
    seen.extend(fresh.iter().map(|i| (i.id.clone(), now)));
    store.save_seen(&seen)?;
    store.append_archive(&fresh)?;

    // `fresh` = les nouveautés de CE run ; `today` = tout ce qui a été vu
    // aujourd'hui, runs précédents compris. Le digest doit refléter le second.
    if let Some(path) = render::digest(&root.join("content"), &store.today()?)? {
        println!("digest écrit : {path}");
    }
    render::latest_json(&root.join("data").join("latest.json"), &fresh)?;
    render::readme(&root.join("README.md"), &store.recent(7)?, cfg.source.len())?;

    // Toutes les sources en échec = problème réel (réseau, DNS), pas une source morte.
    if failures == cfg.source.len() && !cfg.source.is_empty() {
        anyhow::bail!("toutes les sources ont échoué");
    }
    Ok(())
}

fn repo_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join("sources.toml").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            anyhow::bail!("sources.toml introuvable depuis le répertoire courant");
        }
    }
}
