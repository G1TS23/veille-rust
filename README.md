# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 4 septembre 2026

- [Announcing Rust 1.98.1](https://blog.rust-lang.org/2026/09/03/Rust-1.98.1/)  
  <sub>`rust-blog` · officiel · score 105</sub>  
  The Rust team has published a new point release of Rust, 1.98.1. Rust is a programming language that is empowering everyone to build reliable and efficient software. If you have a previous version of Rust installed via rustup, getting Rust 1.98.1 is as easy as: rustup update stable If you don't have it already, you…
- [Announcing rustup 1.29.1](https://blog.rust-lang.org/2026/09/01/Rustup-1.29.1/)  
  <sub>`rust-blog` · officiel · score 105</sub>  
  The rustup team is happy to announce the release of rustup version 1.29.1. Rustup is the recommended tool to install Rust, a programming language that empowers everyone to build reliable and efficient software. What's new in rustup 1.29.1 The headlines of this release are: Concurrency in certain rustup operations has…
- [This Week in Rust 667](https://this-week-in-rust.org/blog/2026/09/02/this-week-in-rust-667/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0269: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0269.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Filesystem sandbox escape when paths or symlinks contain trailing slashes
- [RUSTSEC-2026-0268: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0268.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Guest controlled-size host heap allocation through WASIp3 streams
- [Rust 1.98.1](https://github.com/rust-lang/rust/releases/tag/1.98.1)  
  <sub>`rustc-releases` · release · score 90</sub>  
  rustc: fix miscompilation in generating vtables
- [RUSTSEC-2026-0278: Vulnerability in zbus_polkit](https://rustsec.org/advisories/RUSTSEC-2026-0278.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `zbus_polkit`: authorization bypass via PID reuse
- [RUSTSEC-2026-0276: Vulnerability in apimock](https://rustsec.org/advisories/RUSTSEC-2026-0276.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Path traversal in apimock&apos;s file-serving fallback
- [RUSTSEC-2026-0277: Vulnerability in apimock-server](https://rustsec.org/advisories/RUSTSEC-2026-0277.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Path traversal in apimock-server&apos;s file-serving fallback
- [RUSTSEC-2026-0275: Vulnerability in azure_core](https://rustsec.org/advisories/RUSTSEC-2026-0275.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Legacy `azure_core` writes the `authorization` header value to logs

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-09-04 11:23 UTC
