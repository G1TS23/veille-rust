# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 8 octobre 2026

- [RUSTSEC-2026-0327: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0327.html)  
  <sub>`rustsec` · securite · score 102</sub>  
  Wasmtime component async-lifted callback result count is unvalidated, causing a native stack buffer overflow
- [This Week in Rust 672](https://this-week-in-rust.org/blog/2026/10/07/this-week-in-rust-672/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [Demoting i686 Windows targets to std-only](https://blog.rust-lang.org/2026/10/02/demoting-i686-windows-targets-to-std-only/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  With Rust 1.100.0, the following changes to 32-bit Windows targets will happen: i686-pc-windows-msvc Tier 1 with host tools target will be demoted to Tier 1 without host tools. i686-pc-windows-gnu Tier 2 with host tools target will be demoted to Tier 2 without host tools. Builds of the standard library will continue…
- [RUSTSEC-2026-0320: Vulnerability in wasmtime-wasi-http](https://rustsec.org/advisories/RUSTSEC-2026-0320.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Wasmtime wasi:http implementation panics with a zero timeout supplied
- [RUSTSEC-2026-0323: Vulnerability in wasmtime-wasi](https://rustsec.org/advisories/RUSTSEC-2026-0323.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  fd_readdir copies uninitialized struct padding into guest memory
- [RUSTSEC-2026-0322: Vulnerability in wasmtime-wasi](https://rustsec.org/advisories/RUSTSEC-2026-0322.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Excessive allocated memory on the host when guests don&apos;t have stdio
- [RUSTSEC-2026-0324: Vulnerability in wasmtime-wasi](https://rustsec.org/advisories/RUSTSEC-2026-0324.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Guest can panic host through filesystem timestamp before the epoch on wasip3
- [RUSTSEC-2026-0321: Vulnerability in wasmtime-wasi](https://rustsec.org/advisories/RUSTSEC-2026-0321.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  WASI preview 0 implementation of `poll_oneoff` circumvents fuel consumption
- [RUSTSEC-2026-0326: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0326.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Rooting for GC values live across `try_call` may be missing, causing GC heap corruption
- [RUSTSEC-2026-0325: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0325.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Mis-typed WebAssembly tag imports can lead to GC heap corruption

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-10-08 15:50 UTC
