# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 23 octobre 2026

- [Announcing rustup 1.29.2](https://blog.rust-lang.org/2026/10/20/Rustup-1.29.2/)  
  <sub>`rust-blog` · officiel · score 105</sub>  
  The rustup team is happy to announce the release of rustup version 1.29.2. Rustup is the recommended tool to install Rust, a programming language that empowers everyone to build reliable and efficient software. What's new in rustup 1.29.2 This is a small release which only updates the rustls-webpki and…
- [This Week in Rust 674](https://this-week-in-rust.org/blog/2026/10/21/this-week-in-rust-674/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0380: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0380.html)  
  <sub>`rustsec` · securite · score 99</sub>  
  Bit-packing in Cranelift compiler causes silent overflow and miscompilation
- [RUSTSEC-2026-0377: Vulnerability in wasmtime-wasi-http](https://rustsec.org/advisories/RUSTSEC-2026-0377.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  `wasi-http` p3: forwarded trailers aren&apos;t checked against `is_forbidden_header` hook
- [RUSTSEC-2026-0375: Vulnerability in wasmtime-wasi-http](https://rustsec.org/advisories/RUSTSEC-2026-0375.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  `wasi-http` p3: Guests can set incorrect content-length for spliced response bodies
- [RUSTSEC-2026-0376: Vulnerability in wasmtime-wasi-http](https://rustsec.org/advisories/RUSTSEC-2026-0376.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  `wasi-http`: Guests can add conflicting content lengths to messages
- [RUSTSEC-2026-0379: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0379.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Cranelift backend on aarch64 can jump to incorrect target on large Wasm `br_table` instruction
- [RUSTSEC-2026-0378: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0378.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Panic lowering a host-created component model stream into a guest that expects a different payload type
- [RUSTSEC-2026-0385: Vulnerability in ctutils](https://rustsec.org/advisories/RUSTSEC-2026-0385.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Non-constant-time mask generation and `u128` comparisons
- [RUSTSEC-2026-0384: Vulnerability in tokio](https://rustsec.org/advisories/RUSTSEC-2026-0384.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Race condition in `tokio::time::Interval` on Tokio v1.54.0

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-10-23 13:47 UTC
