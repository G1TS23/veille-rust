# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 13 novembre 2026

- [This Week in Rust 677](https://this-week-in-rust.org/blog/2026/11/11/this-week-in-rust-677/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [Announcing Rust 1.100.0](https://blog.rust-lang.org/2026/11/12/Rust-1.100.0/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  The Rust team is happy to announce a new version of Rust, 1.100.0. Rust is a programming language empowering everyone to build reliable and efficient software. If you have a previous version of Rust installed via rustup, you can get 1.100.0 with: $ rustup update stable If you don't have it already, you can get rustup…
- [Google Summer of Code 2026 results](https://blog.rust-lang.org/2026/11/11/gsoc-2026-results/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  The Rust Project participated in Google Summer of Code (GSoC) for the third time this year. This time we had 13 projects, and our dear contributors have been working very hard on them for several months. As usual, the projects had various durations, so while some of them have ended in August, the last one has only…
- [RUSTSEC-2026-0521: Vulnerability in hickory-recursor](https://rustsec.org/advisories/RUSTSEC-2026-0521.html)  
  <sub>`rustsec` · securite · score 95</sub>  
  Recursor followed every glueless name server in a referral (no fixed hickory-recursor release)
- [RUSTSEC-2026-0522: Vulnerability in hickory-recursor](https://rustsec.org/advisories/RUSTSEC-2026-0522.html)  
  <sub>`rustsec` · securite · score 95</sub>  
  Recursor kept querying cyclic glueless sibling delegations (no fixed hickory-recursor release)
- [RUSTSEC-2026-0533: Vulnerability in ammonia](https://rustsec.org/advisories/RUSTSEC-2026-0533.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Incorrect handling of embedded SVG and MathML leads to mutation XSS
- [RUSTSEC-2026-0524: Vulnerability in quinn-proto](https://rustsec.org/advisories/RUSTSEC-2026-0524.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Long Retry token can panic or exhaust memory in a QUIC client
- [RUSTSEC-2026-0523: Vulnerability in quinn-proto](https://rustsec.org/advisories/RUSTSEC-2026-0523.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  MAX_STREAM_DATA frame can open streams beyond the stream limit
- [RUSTSEC-2026-0525: Vulnerability in libp2p-quic](https://rustsec.org/advisories/RUSTSEC-2026-0525.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Remote panic when a peer certificate expires during the QUIC handshake
- [RUSTSEC-2026-0526: Vulnerability in libp2p-rendezvous](https://rustsec.org/advisories/RUSTSEC-2026-0526.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Rendezvous server accepted unlimited registrations per peer

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-11-13 14:40 UTC
