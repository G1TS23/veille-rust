# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 26 novembre 2026

- [RUSTSEC-2026-0587: Vulnerability in wasmtime-wasi](https://rustsec.org/advisories/RUSTSEC-2026-0587.html)  
  <sub>`rustsec` · securite · score 94</sub>  
  WASI filesystem sandbox escape on Windows through a drive-prefixed path component
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
- [RUSTSEC-2026-0596: Vulnerability in etherparse](https://rustsec.org/advisories/RUSTSEC-2026-0596.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Out-of-bounds read when iterating over IPv6 extension headers of laxly parsed packets
- [RUSTSEC-2026-0595: Unsoundness in parquet](https://rustsec.org/advisories/RUSTSEC-2026-0595.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Apache Arrow Rust: Bloom filters written as plaintext when writing encrypted Parquet
- [RUSTSEC-2026-0594: Vulnerability in cookie](https://rustsec.org/advisories/RUSTSEC-2026-0594.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Private cookie nonces can repeat in processes that fork
- [RUSTSEC-2026-0593: Vulnerability in pqcrypto-dilithium](https://rustsec.org/advisories/RUSTSEC-2026-0593.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Dilithium verification accepts non-conforming signatures with the `avx2` feature
- [RUSTSEC-2026-0592: Vulnerability in url](https://rustsec.org/advisories/RUSTSEC-2026-0592.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Stack overflow in `Url::origin()` on nested `blob:` URLs

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-11-26 16:58 UTC
