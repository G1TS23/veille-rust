# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 6 novembre 2026

- [Announcing rustup 1.29.3](https://blog.rust-lang.org/2026/11/03/Rustup-1.29.3/)  
  <sub>`rust-blog` · officiel · score 105</sub>  
  The rustup team has published a new point release of rustup, 1.29.3. Rustup is the recommended tool to install Rust, a programming language that empowers everyone to build reliable and efficient software. What's new in rustup 1.29.3 This new patch release has fixed some minor regressions over the previous one: Fixed…
- [This Week in Rust 676](https://this-week-in-rust.org/blog/2026/11/04/this-week-in-rust-676/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0434: Vulnerability in ant-releases](https://rustsec.org/advisories/RUSTSEC-2026-0434.html)  
  <sub>`rustsec` · securite · score 95</sub>  
  `ant-releases`: extracting a release archive can write files outside the destination directory (path traversal / zip-slip)
- [RUSTSEC-2026-0437: Vulnerability in azure_security_keyvault_certificates](https://rustsec.org/advisories/RUSTSEC-2026-0437.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Rejected authentication challenges can poison the Key Vault scope cache
- [RUSTSEC-2026-0438: Vulnerability in azure_security_keyvault_secrets](https://rustsec.org/advisories/RUSTSEC-2026-0438.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Rejected authentication challenges can poison the Key Vault scope cache
- [RUSTSEC-2026-0439: Vulnerability in azure_security_keyvault_keys](https://rustsec.org/advisories/RUSTSEC-2026-0439.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Rejected authentication challenges can poison the Key Vault scope cache
- [RUSTSEC-2026-0440: astral-tokio-tar is unmaintained](https://rustsec.org/advisories/RUSTSEC-2026-0440.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `astral-tokio-tar` is unmaintained
- [RUSTSEC-2026-0441: pin-utils is unmaintained](https://rustsec.org/advisories/RUSTSEC-2026-0441.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `pin-utils` is unmaintained
- [RUSTSEC-2026-0442: Unsoundness in arrow-data](https://rustsec.org/advisories/RUSTSEC-2026-0442.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Apache Arrow Rust: Improper validation of RunEndEncoded arrays allows out of bounds reads
- [RUSTSEC-2026-0436: Vulnerability in calamine](https://rustsec.org/advisories/RUSTSEC-2026-0436.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Panic and unbounded allocation (OOM) in the CFB/OLE2 parser on malformed input

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-11-06 14:21 UTC
