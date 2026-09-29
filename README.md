# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 29 septembre 2026

- [This Week in Rust 670](https://this-week-in-rust.org/blog/2026/09/23/this-week-in-rust-670/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [Announcing a Maintainer in Residence: Scott Schafer for the Cargo team](https://blog.rust-lang.org/2026/09/22/announcing-a-maintainer-in-residence-scott-schafer-for-the-cargo-team/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  At the end of August, we announced our first Maintainers in Residence, Rust Project contributors who are funded for their upstream contributions and maintenance work from the Rust Foundation Maintainers Fund (RFMF). Since then, the Rust Leadership Council has dedicated more funds from its Project Priorities budget to…
- [RUSTSEC-2026-0313: Vulnerability in wasmtime-wasi-http](https://rustsec.org/advisories/RUSTSEC-2026-0313.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Outgoing HTTP body write allows guest-driven host memory exhaustion
- [RUSTSEC-2026-0314: Vulnerability in wasmtime-wasi](https://rustsec.org/advisories/RUSTSEC-2026-0314.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Guest can panic host through filesystem datetime overflow
- [RUSTSEC-2026-0316: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0316.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Dynamic record lifting can allocate beyond the hostcall fuel limit
- [RUSTSEC-2026-0315: Vulnerability in wasmtime](https://rustsec.org/advisories/RUSTSEC-2026-0315.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  `call_ref` and exception `catch` can drop some fuel accounting, leading to exponential fuel amplification
- [RUSTSEC-2026-0312: Vulnerability in x509-validator](https://rustsec.org/advisories/RUSTSEC-2026-0312.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Excluded iPAddress name constraints with an all-zero mask are not applied
- [RUSTSEC-2026-0311: Vulnerability in latex-rust](https://rustsec.org/advisories/RUSTSEC-2026-0311.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Stack overflow on deeply nested LaTeX input
- [RUSTSEC-2026-0309: Unsoundness in bun_collections](https://rustsec.org/advisories/RUSTSEC-2026-0309.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `SinglyLinkedList::remove` dereferences a null link
- [RUSTSEC-2026-0310: Vulnerability in domain](https://rustsec.org/advisories/RUSTSEC-2026-0310.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Various panics, soundness and resource exhaustion issues

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-09-29 12:58 UTC
