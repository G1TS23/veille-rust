# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 14 octobre 2026

- [This Week in Rust 672](https://this-week-in-rust.org/blog/2026/10/07/this-week-in-rust-672/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2024-0449: unsafe-libyaml is unmaintained](https://rustsec.org/advisories/RUSTSEC-2024-0449.html)  
  <sub>`rustsec` · securite · score 96</sub>  
  unsafe-libyaml is unmaintained
- [RUSTSEC-2026-0347: Vulnerability in rustls](https://rustsec.org/advisories/RUSTSEC-2026-0347.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Buffered application data sent to outer server after ECH rejection
- [RUSTSEC-2026-0348: Vulnerability in rustls](https://rustsec.org/advisories/RUSTSEC-2026-0348.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Brotli certificate decompression can allocate up to 1 GiB before authentication
- [RUSTSEC-2026-0349: Vulnerability in rustls](https://rustsec.org/advisories/RUSTSEC-2026-0349.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Unauthenticated ServerHello can downgrade later connections to classical key exchange
- [RUSTSEC-2026-0346: Vulnerability in rustls](https://rustsec.org/advisories/RUSTSEC-2026-0346.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `Debug` impls for client session values expose TLS secrets
- [RUSTSEC-2021-0158: Vulnerability in ammonia](https://rustsec.org/advisories/RUSTSEC-2021-0158.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  XSS in ammonia via SVG `animateMotion`, `animateTransform`, and/or `animateColor` tags
- [RUSTSEC-2026-0345: Vulnerability in kuva](https://rustsec.org/advisories/RUSTSEC-2026-0345.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Untrusted labels can inject SVG attributes, CSS values, and terminal escape sequences
- [RUSTSEC-2026-0344: Vulnerability in ruzstd](https://rustsec.org/advisories/RUSTSEC-2026-0344.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Decompression bomb in `ruzstd`: a tiny input can exhaust memory (OOM)
- [RUSTSEC-2021-0157: Vulnerability in ammonia](https://rustsec.org/advisories/RUSTSEC-2021-0157.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Incorrect escaping for `url("")` functions in CSS style attributes

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-10-14 14:00 UTC
