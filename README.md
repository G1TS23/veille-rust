# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 24 septembre 2026

- [This Week in Rust 670](https://this-week-in-rust.org/blog/2026/09/23/this-week-in-rust-670/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [Announcing a Maintainer in Residence: Scott Schafer for the Cargo team](https://blog.rust-lang.org/2026/09/22/announcing-a-maintainer-in-residence-scott-schafer-for-the-cargo-team/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  At the end of August, we announced our first Maintainers in Residence, Rust Project contributors who are funded for their upstream contributions and maintenance work from the Rust Foundation Maintainers Fund (RFMF). Since then, the Rust Leadership Council has dedicated more funds from its Project Priorities budget to…
- [GitHub Actions leaking secrets when Miri output is cached](https://blog.rust-lang.org/2026/09/21/github-actions-leaking-secrets-when-miri-output-is-cached/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  The Rust Security Response Team was notified that Miri stores all environment variables to target/, allowing secrets to persist in caches. While not necessary a vulnerability in and of itself, when paired with GitHub Actions caching behavior, it is possible for this to expose secrets to PRs. Overview GitHub Actions…
- [Be alert: targeted attacks on prominent Rustaceans](https://blog.rust-lang.org/2026/09/17/targeted-attacks/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  We believe that there is an ongoing campaign targeting rust-lang members and owners of popular crates that is attempting to compromise devices and accounts in order to use them to publish malware. What we've seen A video call is set up for something positive — maybe for a job, maybe for a project, maybe for a contract…
- [RUSTSEC-2026-0307: Vulnerability in uncbv](https://rustsec.org/advisories/RUSTSEC-2026-0307.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `uncbv`: archive extraction is vulnerable to path traversal (zip-slip)
- [RUSTSEC-2026-0306: Unsoundness in faster-hex](https://rustsec.org/advisories/RUSTSEC-2026-0306.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `hex_decode_unchecked` AVX2 path reads past `src`
- [RUSTSEC-2026-0304: Vulnerability in connectrpc](https://rustsec.org/advisories/RUSTSEC-2026-0304.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Finished streaming calls keep reading a stalled request body indefinitely
- [RUSTSEC-2026-0305: Vulnerability in librsvg](https://rustsec.org/advisories/RUSTSEC-2026-0305.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Use-after-free when XML includes have duplicated entities
- [RUSTSEC-2026-0303: stack-graphs is unmaintained](https://rustsec.org/advisories/RUSTSEC-2026-0303.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `stack-graphs` is archived and unmaintained
- [RUSTSEC-2026-0302: Unsoundness in stack-graphs](https://rustsec.org/advisories/RUSTSEC-2026-0302.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `stack-graphs` C API exports are safe `extern "C"` functions

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-09-24 13:46 UTC
