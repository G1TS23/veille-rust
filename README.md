# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 9 novembre 2026

- [Announcing rustup 1.29.3](https://blog.rust-lang.org/2026/11/03/Rustup-1.29.3/)  
  <sub>`rust-blog` · officiel · score 105</sub>  
  The rustup team has published a new point release of rustup, 1.29.3. Rustup is the recommended tool to install Rust, a programming language that empowers everyone to build reliable and efficient software. What's new in rustup 1.29.3 This new patch release has fixed some minor regressions over the previous one: Fixed…
- [This Week in Rust 676](https://this-week-in-rust.org/blog/2026/11/04/this-week-in-rust-676/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0463: Vulnerability in openssl](https://rustsec.org/advisories/RUSTSEC-2026-0463.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Out-of-bounds write in `aes::unwrap_key`
- [RUSTSEC-2026-0456: Vulnerability in heifer-hevc-dec](https://rustsec.org/advisories/RUSTSEC-2026-0456.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Panic on an HEVC tile layout that does not match the active SPS
- [RUSTSEC-2026-0460: Vulnerability in heifer-hevc-dec](https://rustsec.org/advisories/RUSTSEC-2026-0460.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Panic on chroma QP offsets enabled with an empty offset list
- [RUSTSEC-2026-0461: Vulnerability in rmcp](https://rustsec.org/advisories/RUSTSEC-2026-0461.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Unbounded tool schema cache in Streamable HTTP server transport
- [RUSTSEC-2026-0457: Vulnerability in rmcp](https://rustsec.org/advisories/RUSTSEC-2026-0457.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  OAuth client SSRF filter checks host strings, not resolved addresses
- [RUSTSEC-2026-0462: Vulnerability in rmcp](https://rustsec.org/advisories/RUSTSEC-2026-0462.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Unbounded response body buffering in Streamable HTTP clients
- [RUSTSEC-2026-0458: Vulnerability in heifer-isobmff](https://rustsec.org/advisories/RUSTSEC-2026-0458.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Memory exhaustion when parsing a crafted `iloc` box
- [RUSTSEC-2026-0459: Vulnerability in libsqlite3-sys](https://rustsec.org/advisories/RUSTSEC-2026-0459.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `libsqlite3-sys` via C SQLite CVE-2026-119375

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-11-09 19:05 UTC
