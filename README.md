# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 11 novembre 2026

- [Google Summer of Code 2026 results](https://blog.rust-lang.org/2026/11/11/gsoc-2026-results/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  The Rust Project participated in Google Summer of Code (GSoC) for the third time this year. This time we had 13 projects, and our dear contributors have been working very hard on them for several months. As usual, the projects had various durations, so while some of them have ended in August, the last one has only…
- [This Week in Rust 676](https://this-week-in-rust.org/blog/2026/11/04/this-week-in-rust-676/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0480: Vulnerability in tdx_workload_attestation](https://rustsec.org/advisories/RUSTSEC-2026-0480.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Panic on a GCP launch endorsement without TDX measurements
- [RUSTSEC-2026-0481: Vulnerability in tdx_workload_attestation](https://rustsec.org/advisories/RUSTSEC-2026-0481.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `verify_x509_cert` does not check the certificate validity period
- [RUSTSEC-2026-0482: Vulnerability in cap-primitives](https://rustsec.org/advisories/RUSTSEC-2026-0482.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Paths or symlinks ending in a slash escape the sandbox
- [RUSTSEC-2026-0483: lwd-uri-util contained malicious code](https://rustsec.org/advisories/RUSTSEC-2026-0483.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `lwd-uri-util` was removed from crates.io for malicious code
- [RUSTSEC-2026-0490: Vulnerability in hickory-proto](https://rustsec.org/advisories/RUSTSEC-2026-0490.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Zone file lexer panics on a token of 4096 or more characters
- [RUSTSEC-2026-0491: Vulnerability in hickory-proto](https://rustsec.org/advisories/RUSTSEC-2026-0491.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Memory amplification from record counts in the DNS message header
- [RUSTSEC-2026-0489: Vulnerability in hickory-proto](https://rustsec.org/advisories/RUSTSEC-2026-0489.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Panic when an SVCB or HTTPS parameter value is a single quote character
- [RUSTSEC-2026-0487: Vulnerability in hickory-proto](https://rustsec.org/advisories/RUSTSEC-2026-0487.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Panic on a malformed `alpn` value when parsing SVCB or HTTPS records from text

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-11-11 14:28 UTC
