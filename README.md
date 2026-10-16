# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 16 octobre 2026

- [Announcing Rust 1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/)  
  <sub>`rust-blog` · officiel · score 105</sub>  
  The Rust team has published a new point release of Rust, 1.99.1. Rust is a programming language that is empowering everyone to build reliable and efficient software. If you have a previous version of Rust installed via rustup, getting Rust 1.99.1 is as easy as: rustup update stable If you don't have it already, you…
- [RUSTSEC-2026-0362: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0362.html)  
  <sub>`rustsec` · securite · score 102</sub>  
  Wasmtime may not properly enforce async-entrypoint requirement
- [RUSTSEC-2026-0353: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0353.html)  
  <sub>`rustsec` · securite · score 102</sub>  
  Panic when invoking an async-lifted function via `call`
- [RUSTSEC-2026-0354: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0354.html)  
  <sub>`rustsec` · securite · score 102</sub>  
  Host panic when component `call_async` is cancelled under some conditions
- [This Week in Rust 673](https://this-week-in-rust.org/blog/2026/10/14/this-week-in-rust-673/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2024-0449: unsafe-libyaml is unmaintained](https://rustsec.org/advisories/RUSTSEC-2024-0449.html)  
  <sub>`rustsec` · securite · score 96</sub>  
  unsafe-libyaml is unmaintained
- [Rust 1.99.1](https://github.com/rust-lang/rust/releases/tag/1.99.1)  
  <sub>`rustc-releases` · release · score 94</sub>  
  library: fix missing symbol wcslen on UEFI and embedded targets rustc: fix miscompilation of FFI bool return values on x86_64 rust-lld: add the rpath for libLLVM on macOS
- [RUSTSEC-2026-0351: Vulnerability in wasmtime-wasi-http](https://rustsec.org/advisories/RUSTSEC-2026-0351.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  `wasi-http`: Request with long url before query parameter crashes host
- [RUSTSEC-2026-0350: Vulnerability in wasmtime-wasi-http](https://rustsec.org/advisories/RUSTSEC-2026-0350.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Guests can send absolute-form HTTP requests to different hosts
- [RUSTSEC-2026-0352: Vulnerability in wasmtime-wasi](https://rustsec.org/advisories/RUSTSEC-2026-0352.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Filesystem sandbox escape when renaming symlinks with trailing slashes

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-10-16 11:30 UTC
