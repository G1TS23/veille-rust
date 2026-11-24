# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 24 novembre 2026

- [RUSTSEC-2026-0566: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0566.html)  
  <sub>`rustsec` · securite · score 102</sub>  
  wasmtime-wasi: Unbounded host memory growth via empty writes to `AsyncWriteStream`
- [This Week in Rust 678](https://this-week-in-rust.org/blog/2026/11/18/this-week-in-rust-678/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0587: Vulnerability in wasmtime-wasi](https://rustsec.org/advisories/RUSTSEC-2026-0587.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  WASI filesystem sandbox escape on Windows through a drive-prefixed path component
- [RUSTSEC-2026-0567: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0567.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Abort in Wasmtime builds without the `threads` cargo feature
- [RUSTSEC-2026-0569: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0569.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Panic in `Caller::get_module_export` when a host function isn&apos;t called from WebAssembly
- [RUSTSEC-2026-0570: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0570.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Abort in Wasmtime builds without the `gc` cargo feature
- [RUSTSEC-2026-0572: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0572.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Panic when a component cancels a stream or future copy on the wrong end
- [RUSTSEC-2026-0571: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0571.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  wasmtime-wasi-http: Empty Body Writes can result in Unbound Heap Usage with HTTP/2
- [RUSTSEC-2026-0568: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0568.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Stale GC heap base pointer in compiled code lets a Wasm guest abort the host process
- [RUSTSEC-2026-0573: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0573.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Process abort when a GC heap&apos;s reservation isn&apos;t a multiple of the page size

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-11-24 19:41 UTC
