# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 29 octobre 2026

- [This Week in Rust 675](https://this-week-in-rust.org/blog/2026/10/28/this-week-in-rust-675/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [How to fund Rust contributors](https://blog.rust-lang.org/2026/10/26/how-to-fund-rust-contributors/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  TLDR: To fund general maintenance, donate to the Rust Foundation Maintainers Fund or sponsor the RustNL Maintainers Team To fund specific features and improvements, sponsor fundable Rust Project Goals To fund individual contributors, donate to their GitHub Sponsors accounts There is always so much to do in the Rust…
- [RUSTSEC-2026-0386: Unsoundness in tokio](https://rustsec.org/advisories/RUSTSEC-2026-0386.html)  
  <sub>`rustsec` · securite · score 98</sub>  
  `AsyncFd::new` and related constructors violate I/O safety
- [RUSTSEC-2026-0415: Vulnerability in juniper_hyper](https://rustsec.org/advisories/RUSTSEC-2026-0415.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Unbounded memory use on large request bodies
- [RUSTSEC-2026-0417: Vulnerability in juniper](https://rustsec.org/advisories/RUSTSEC-2026-0417.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Excessive CPU and memory use on crafted GraphQL documents
- [RUSTSEC-2026-0416: Vulnerability in juniper](https://rustsec.org/advisories/RUSTSEC-2026-0416.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Stack overflow on deeply nested GraphQL documents
- [RUSTSEC-2026-0418: Vulnerability in libz-sys](https://rustsec.org/advisories/RUSTSEC-2026-0418.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  libz-sys build script leaks environment variables in the build directory
- [RUSTSEC-2026-0419: Vulnerability in juniper_warp](https://rustsec.org/advisories/RUSTSEC-2026-0419.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Unbounded memory use on large request bodies
- [RUSTSEC-2026-0414: Vulnerability in pulldown-cmark](https://rustsec.org/advisories/RUSTSEC-2026-0414.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Exponential event growth with `ENABLE_WIKILINKS`
- [RUSTSEC-2026-0413: Vulnerability in rustls-ffi](https://rustsec.org/advisories/RUSTSEC-2026-0413.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Use-after-free of caller-owned server name in client connection constructors

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-10-29 15:31 UTC
