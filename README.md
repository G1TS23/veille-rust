# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 12 novembre 2026

- [Google Summer of Code 2026 results](https://blog.rust-lang.org/2026/11/11/gsoc-2026-results/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  The Rust Project participated in Google Summer of Code (GSoC) for the third time this year. This time we had 13 projects, and our dear contributors have been working very hard on them for several months. As usual, the projects had various durations, so while some of them have ended in August, the last one has only…
- [RUSTSEC-2026-0521: Vulnerability in hickory-recursor](https://rustsec.org/advisories/RUSTSEC-2026-0521.html)  
  <sub>`rustsec` · securite · score 95</sub>  
  Recursor followed every glueless name server in a referral (no fixed hickory-recursor release)
- [RUSTSEC-2026-0522: Vulnerability in hickory-recursor](https://rustsec.org/advisories/RUSTSEC-2026-0522.html)  
  <sub>`rustsec` · securite · score 95</sub>  
  Recursor kept querying cyclic glueless sibling delegations (no fixed hickory-recursor release)
- [RUSTSEC-2024-0450: Vulnerability in russh](https://rustsec.org/advisories/RUSTSEC-2024-0450.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Memory exhaustion from an unchecked SSH packet length
- [RUSTSEC-2026-0494: Vulnerability in russh](https://rustsec.org/advisories/RUSTSEC-2026-0494.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  SSH message fields allocated before their size is checked
- [RUSTSEC-2026-0493: Vulnerability in russh](https://rustsec.org/advisories/RUSTSEC-2026-0493.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  No limit on the size of a decompressed SSH packet
- [RUSTSEC-2026-0496: Vulnerability in russh](https://rustsec.org/advisories/RUSTSEC-2026-0496.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Pre-authentication allocation from the keyboard-interactive response count
- [RUSTSEC-2026-0495: Vulnerability in russh](https://rustsec.org/advisories/RUSTSEC-2026-0495.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Agent server signs with confirm-constrained keys without calling `Agent::confirm`
- [RUSTSEC-2026-0492: Vulnerability in russh](https://rustsec.org/advisories/RUSTSEC-2026-0492.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Client `Handler` callbacks run for channels the client never opened
- [RUSTSEC-2026-0510: Vulnerability in vodozemac](https://rustsec.org/advisories/RUSTSEC-2026-0510.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Panic when decoding malformed Ed25519 keys from Base64

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-11-12 17:02 UTC
