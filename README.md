# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 8 novembre 2026

- [Announcing rustup 1.29.3](https://blog.rust-lang.org/2026/11/03/Rustup-1.29.3/)  
  <sub>`rust-blog` · officiel · score 105</sub>  
  The rustup team has published a new point release of rustup, 1.29.3. Rustup is the recommended tool to install Rust, a programming language that empowers everyone to build reliable and efficient software. What's new in rustup 1.29.3 This new patch release has fixed some minor regressions over the previous one: Fixed…
- [This Week in Rust 676](https://this-week-in-rust.org/blog/2026/11/04/this-week-in-rust-676/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0434: Vulnerability in ant-releases](https://rustsec.org/advisories/RUSTSEC-2026-0434.html)  
  <sub>`rustsec` · securite · score 95</sub>  
  `ant-releases`: extracting a release archive can write files outside the destination directory (path traversal / zip-slip)
- [RUSTSEC-2026-0443: Vulnerability in actix-files](https://rustsec.org/advisories/RUSTSEC-2026-0443.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Hidden file disclosure through Windows short names
- [RUSTSEC-2026-0444: astral-tl is unmaintained](https://rustsec.org/advisories/RUSTSEC-2026-0444.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `astral-tl` is unmaintained
- [RUSTSEC-2026-0447: Vulnerability in actix-web](https://rustsec.org/advisories/RUSTSEC-2026-0447.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Compressed request bodies can exhaust memory before extractor limits apply
- [RUSTSEC-2026-0445: Vulnerability in vodozemac](https://rustsec.org/advisories/RUSTSEC-2026-0445.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Degraded secret zeroization capabilities
- [RUSTSEC-2026-0451: Vulnerability in vodozemac](https://rustsec.org/advisories/RUSTSEC-2026-0451.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Degraded secret zeroization capabilities
- [RUSTSEC-2026-0452: Vulnerability in surrealkv](https://rustsec.org/advisories/RUSTSEC-2026-0452.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Out-of-bounds read when decoding internal keys shorter than 16 bytes
- [RUSTSEC-2026-0453: Vulnerability in surrealdb](https://rustsec.org/advisories/RUSTSEC-2026-0453.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Use-after-free in KV backends via leaked `affinitypool::spawn_local` future

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-11-08 13:30 UTC
