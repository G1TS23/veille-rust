# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 27 octobre 2026

- [Announcing rustup 1.29.2](https://blog.rust-lang.org/2026/10/20/Rustup-1.29.2/)  
  <sub>`rust-blog` · officiel · score 105</sub>  
  The rustup team is happy to announce the release of rustup version 1.29.2. Rustup is the recommended tool to install Rust, a programming language that empowers everyone to build reliable and efficient software. What's new in rustup 1.29.2 This is a small release which only updates the rustls-webpki and…
- [How to fund Rust contributors](https://blog.rust-lang.org/2026/10/26/how-to-fund-rust-contributors/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  TLDR: To fund general maintenance, donate to the Rust Foundation Maintainers Fund or sponsor the RustNL Maintainers Team To fund specific features and improvements, sponsor fundable Rust Project Goals To fund individual contributors, donate to their GitHub Sponsors accounts There is always so much to do in the Rust…
- [This Week in Rust 674](https://this-week-in-rust.org/blog/2026/10/21/this-week-in-rust-674/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0386: Unsoundness in tokio](https://rustsec.org/advisories/RUSTSEC-2026-0386.html)  
  <sub>`rustsec` · securite · score 98</sub>  
  `AsyncFd::new` and related constructors violate I/O safety
- [RUSTSEC-2026-0413: Vulnerability in rustls-ffi](https://rustsec.org/advisories/RUSTSEC-2026-0413.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Use-after-free of caller-owned server name in client connection constructors
- [RUSTSEC-2026-0412: Vulnerability in cargo-nextest](https://rustsec.org/advisories/RUSTSEC-2026-0412.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Arbitrary code execution when rerunning tests from a portable recording
- [RUSTSEC-2026-0393: Unsoundness in zstd](https://rustsec.org/advisories/RUSTSEC-2026-0393.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Out-of-bounds read when training a dictionary from inconsistent sample sizes
- [RUSTSEC-2026-0392: Vulnerability in zstd](https://rustsec.org/advisories/RUSTSEC-2026-0392.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Uninitialized memory exposed by `write::Decoder` on a corrupt legacy frame
- [RUSTSEC-2026-0391: Vulnerability in zstd](https://rustsec.org/advisories/RUSTSEC-2026-0391.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Data from an abandoned stream leaks into the next stream on a reused context
- [RUSTSEC-2026-0394: Vulnerability in rustpython-vm](https://rustsec.org/advisories/RUSTSEC-2026-0394.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Build environment compiled into the library and readable at runtime

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-10-27 14:54 UTC
