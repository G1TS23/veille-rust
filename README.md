# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 28 novembre 2026

- [Announcing Rust 1.100.1](https://blog.rust-lang.org/2026/11/27/Rust-1.100.1/)  
  <sub>`rust-blog` · officiel · score 105</sub>  
  The Rust team has published a new point release of Rust, 1.100.1. Rust is a programming language that is empowering everyone to build reliable and efficient software. If you have a previous version of Rust installed via rustup, getting Rust 1.100.1 is as easy as: $ rustup update stable If you don't have it already,…
- [This Week in Rust 679](https://this-week-in-rust.org/blog/2026/11/25/this-week-in-rust-679/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0602: Vulnerability in dryoc](https://rustsec.org/advisories/RUSTSEC-2026-0602.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  Ed25519 signing key pairs with an inconsistent embedded public key are accepted
- [Rust 1.100.1](https://github.com/rust-lang/rust/releases/tag/1.100.1)  
  <sub>`rustc-releases` · release · score 90</sub>  
  library: fix regression where some types were not inferred as having Iterator impls This backports a change to the unstable const version of Iterator, which unintentionally caused existing code to not be seen as implementing Iterator when a valid implementation was present.
- [RUSTSEC-2026-0603: Vulnerability in dryoc](https://rustsec.org/advisories/RUSTSEC-2026-0603.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  X25519 accepts low-order public keys, yielding session keys computable without a private key
- [RUSTSEC-2026-0601: Vulnerability in wsts](https://rustsec.org/advisories/RUSTSEC-2026-0601.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  FROST coordinator accepts unvalidated DKG polynomial commitments
- [RUSTSEC-2026-0600: Vulnerability in kube-client](https://rustsec.org/advisories/RUSTSEC-2026-0600.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `openssl-tls` trusts OpenSSL&apos;s default CA store in addition to the configured cluster CA
- [RUSTSEC-2026-0599: Vulnerability in xmloxide](https://rustsec.org/advisories/RUSTSEC-2026-0599.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Stack overflow from unbounded recursion in DTD content-model parsing
- [RUSTSEC-2026-0597: Unsoundness in hugepage-rs](https://rustsec.org/advisories/RUSTSEC-2026-0597.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `hugepage_rs::Box::new` may cause undefined behavior when initializing memory
- [RUSTSEC-2026-0598: Unsoundness in hugepage-rs](https://rustsec.org/advisories/RUSTSEC-2026-0598.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `hugepage_rs::alloc` may cause undefined behavior with zero-sized layouts from safe code

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-11-28 15:37 UTC
