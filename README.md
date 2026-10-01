# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 1 octobre 2026

- [This Week in Rust 671](https://this-week-in-rust.org/blog/2026/09/30/this-week-in-rust-671/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [Announcing Rust 1.99.0](https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  The Rust team is happy to announce a new version of Rust, 1.99.0. Rust is a programming language empowering everyone to build reliable and efficient software. If you have a previous version of Rust installed via rustup, you can get 1.99.0 with: $ rustup update stable If you don't have it already, you can get rustup…
- [Rust 1.99.0](https://github.com/rust-lang/rust/releases/tag/1.99.0)  
  <sub>`rustc-releases` · release · score 96</sub>  
  Language Add allow-by-default raw_borrows_via_references lint that checks for references that decay immediately into raw borrows Extend unconditional_panic lint to function calls that panic when the chunks/windows size is zero Stabilize C-variadic function definitions Stabilize the ability to use #\[unsafe(naked)\]…
- [RUSTSEC-2026-0313: Vulnerability in wasmtime-wasi-http](https://rustsec.org/advisories/RUSTSEC-2026-0313.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Outgoing HTTP body write allows guest-driven host memory exhaustion
- [RUSTSEC-2026-0314: Vulnerability in wasmtime-wasi](https://rustsec.org/advisories/RUSTSEC-2026-0314.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Guest can panic host through filesystem datetime overflow
- [RUSTSEC-2026-0316: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0316.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Dynamic record lifting can allocate beyond the hostcall fuel limit
- [RUSTSEC-2026-0315: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0315.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  `call_ref` and exception `catch` can drop some fuel accounting, leading to exponential fuel amplification
- [RUSTSEC-2026-0317: Vulnerability in sheets-diff](https://rustsec.org/advisories/RUSTSEC-2026-0317.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  A 512-byte workbook can provoke a multi-gigabyte allocation and abort the process
- [RUSTSEC-2026-0312: Vulnerability in x509-validator](https://rustsec.org/advisories/RUSTSEC-2026-0312.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Excluded iPAddress name constraints with an all-zero mask are not applied
- [RUSTSEC-2026-0311: Vulnerability in latex-rust](https://rustsec.org/advisories/RUSTSEC-2026-0311.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Stack overflow on deeply nested LaTeX input

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-10-01 13:24 UTC
