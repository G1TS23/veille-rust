# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 12 octobre 2026

- [This Week in Rust 672](https://this-week-in-rust.org/blog/2026/10/07/this-week-in-rust-672/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2024-0449: unsafe-libyaml is unmaintained](https://rustsec.org/advisories/RUSTSEC-2024-0449.html)  
  <sub>`rustsec` · securite · score 96</sub>  
  unsafe-libyaml is unmaintained
- [RUSTSEC-2021-0157: Vulnerability in ammonia](https://rustsec.org/advisories/RUSTSEC-2021-0157.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Incorrect escaping for `url("")` functions in CSS style attributes
- [RUSTSEC-2026-0337: Vulnerability in ic-cipher](https://rustsec.org/advisories/RUSTSEC-2026-0337.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Poly1305 panics or computes a wrong tag on attacker-chosen messages of 64 bytes or more
- [RUSTSEC-2026-0341: Vulnerability in quinn-proto](https://rustsec.org/advisories/RUSTSEC-2026-0341.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Stream reassembly memory-exhaustion guard in quinn-proto can be bypassed
- [RUSTSEC-2026-0343: Vulnerability in quinn-proto](https://rustsec.org/advisories/RUSTSEC-2026-0343.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Outgoing datagram buffer accounting can panic the sender in quinn-proto
- [RUSTSEC-2026-0339: Vulnerability in quinn-proto](https://rustsec.org/advisories/RUSTSEC-2026-0339.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Unbounded growth of pending connection ID retirements in quinn-proto
- [RUSTSEC-2026-0342: Vulnerability in quinn-proto](https://rustsec.org/advisories/RUSTSEC-2026-0342.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Zero-length DATAGRAM frames bypass the receive buffer limit in quinn-proto
- [RUSTSEC-2026-0340: Vulnerability in quinn-proto](https://rustsec.org/advisories/RUSTSEC-2026-0340.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Peer minimum ACK delay can panic quinn-proto when ACK_FREQUENCY transmission is enabled
- [RUSTSEC-2026-0338: Vulnerability in quinn-proto](https://rustsec.org/advisories/RUSTSEC-2026-0338.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Remote peer can panic quinn-proto with zero-length STREAM frames during unordered reads

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-10-12 18:11 UTC
