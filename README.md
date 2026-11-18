# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 18 novembre 2026

- [Enabling the parallel frontend on nightly](https://blog.rust-lang.org/2026/11/16/enabling-parallel-frontend-on-nightly/)  
  <sub>`rust-blog` · officiel · score 105</sub>  
  TL;DR: We are enabling the parallel frontend of the Rust compiler on nightly by default, to find remaining issues in preparation for its stabilization. Context The Rust compiler has been using multiple threads to compile code in its "backend" part (using LLVM) for a long time. However, its "frontend" part (which…
- [This Week in Rust 677](https://this-week-in-rust.org/blog/2026/11/11/this-week-in-rust-677/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [Announcing Rust 1.100.0](https://blog.rust-lang.org/2026/11/12/Rust-1.100.0/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  The Rust team is happy to announce a new version of Rust, 1.100.0. Rust is a programming language empowering everyone to build reliable and efficient software. If you have a previous version of Rust installed via rustup, you can get 1.100.0 with: $ rustup update stable If you don't have it already, you can get rustup…
- [RUSTSEC-2026-0521: Vulnerability in hickory-recursor](https://rustsec.org/advisories/RUSTSEC-2026-0521.html)  
  <sub>`rustsec` · securite · score 95</sub>  
  Recursor followed every glueless name server in a referral (no fixed hickory-recursor release)
- [RUSTSEC-2026-0522: Vulnerability in hickory-recursor](https://rustsec.org/advisories/RUSTSEC-2026-0522.html)  
  <sub>`rustsec` · securite · score 95</sub>  
  Recursor kept querying cyclic glueless sibling delegations (no fixed hickory-recursor release)
- [RUSTSEC-2026-0533: Vulnerability in ammonia](https://rustsec.org/advisories/RUSTSEC-2026-0533.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Incorrect handling of embedded SVG and MathML leads to mutation XSS
- [RUSTSEC-2026-0564: Vulnerability in lettre](https://rustsec.org/advisories/RUSTSEC-2026-0564.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Panic on a mailbox display name containing CR or LF allows denial of service
- [RUSTSEC-2026-0562: Vulnerability in lettre](https://rustsec.org/advisories/RUSTSEC-2026-0562.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Panic on an `EHLO` reply line of only white space allows denial of service
- [RUSTSEC-2026-0561: Vulnerability in lettre](https://rustsec.org/advisories/RUSTSEC-2026-0561.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Missing dot-stuffing after a bare LF or CR allows SMTP command injection
- [RUSTSEC-2026-0563: Vulnerability in lettre](https://rustsec.org/advisories/RUSTSEC-2026-0563.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Messages for an address with a quoted local part are sent to a different address

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-11-18 17:30 UTC
