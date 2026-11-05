# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 5 novembre 2026

- [Announcing rustup 1.29.3](https://blog.rust-lang.org/2026/11/03/Rustup-1.29.3/)  
  <sub>`rust-blog` · officiel · score 105</sub>  
  The rustup team has published a new point release of rustup, 1.29.3. Rustup is the recommended tool to install Rust, a programming language that empowers everyone to build reliable and efficient software. What's new in rustup 1.29.3 This new patch release has fixed some minor regressions over the previous one: Fixed…
- [This Week in Rust 676](https://this-week-in-rust.org/blog/2026/11/04/this-week-in-rust-676/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0434: Vulnerability in ant-releases](https://rustsec.org/advisories/RUSTSEC-2026-0434.html)  
  <sub>`rustsec` · securite · score 95</sub>  
  `ant-releases`: extracting a release archive can write files outside the destination directory (path traversal / zip-slip)
- [RUSTSEC-2026-0427: Vulnerability in sn-releases](https://rustsec.org/advisories/RUSTSEC-2026-0427.html)  
  <sub>`rustsec` · securite · score 95</sub>  
  `sn-releases`: `extract_release_archive` writes archive entries without a path-traversal check (zip-slip)
- [RUSTSEC-2026-0420: Vulnerability in wasmtime-wasi](https://rustsec.org/advisories/RUSTSEC-2026-0420.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  wasip3 UDP `send` can deliver a datagram to an address only allowed for receiving
- [RUSTSEC-2026-0421: Vulnerability in wasmtime-wasi-http](https://rustsec.org/advisories/RUSTSEC-2026-0421.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  wasi-http p2: outgoing content-length can be invalid or not match body
- [RUSTSEC-2026-0424: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0424.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Unchecked memory usage in transferring borrowed resources between components
- [RUSTSEC-2026-0423: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0423.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Cross-engine type confusion with host-created struct-types-with-supertypes
- [RUSTSEC-2026-0422: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0422.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Typed functions with `i31` or `nullref` arguments may panic due to insufficient type-check
- [RUSTSEC-2026-0437: Vulnerability in azure_security_keyvault_certificates](https://rustsec.org/advisories/RUSTSEC-2026-0437.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Rejected authentication challenges can poison the Key Vault scope cache

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-11-05 16:47 UTC
