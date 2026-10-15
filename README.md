# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 15 octobre 2026

- [RUSTSEC-2026-0362: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0362.html)  
  <sub>`rustsec` · securite · score 102</sub>  
  Wasmtime may not properly enforce async-entrypoint requirement
- [RUSTSEC-2026-0353: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0353.html)  
  <sub>`rustsec` · securite · score 102</sub>  
  Panic when invoking an async-lifted function via `call`
- [RUSTSEC-2026-0354: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0354.html)  
  <sub>`rustsec` · securite · score 102</sub>  
  Host panic when component `call_async` is cancelled under some conditions
- [RUSTSEC-2024-0449: unsafe-libyaml is unmaintained](https://rustsec.org/advisories/RUSTSEC-2024-0449.html)  
  <sub>`rustsec` · securite · score 96</sub>  
  unsafe-libyaml is unmaintained
- [RUSTSEC-2026-0351: Vulnerability in wasmtime-wasi-http](https://rustsec.org/advisories/RUSTSEC-2026-0351.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  `wasi-http`: Request with long url before query parameter crashes host
- [RUSTSEC-2026-0350: Vulnerability in wasmtime-wasi-http](https://rustsec.org/advisories/RUSTSEC-2026-0350.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Guests can send absolute-form HTTP requests to different hosts
- [RUSTSEC-2026-0352: Vulnerability in wasmtime-wasi](https://rustsec.org/advisories/RUSTSEC-2026-0352.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Filesystem sandbox escape when renaming symlinks with trailing slashes
- [RUSTSEC-2026-0358: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0358.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Yielding in a component start function can lead to a host panic
- [RUSTSEC-2026-0361: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0361.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Panic when pending calls are run after a trap
- [RUSTSEC-2026-0364: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0364.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Exception tags can cause host data structures to be unaligned

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-10-15 13:40 UTC
