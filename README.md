# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 10 novembre 2026

- [This Week in Rust 676](https://this-week-in-rust.org/blog/2026/11/04/this-week-in-rust-676/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0478: Vulnerability in openssl](https://rustsec.org/advisories/RUSTSEC-2026-0478.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Unchecked callback lengths in PSK and cookie callbacks
- [RUSTSEC-2026-0474: Vulnerability in openssl](https://rustsec.org/advisories/RUSTSEC-2026-0474.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `X509Ref::ocsp_responders` can produce a `&str` that is not valid UTF-8
- [RUSTSEC-2026-0479: Vulnerability in openssl](https://rustsec.org/advisories/RUSTSEC-2026-0479.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Out-of-bounds read when a PEM password callback returns an oversized length
- [RUSTSEC-2026-0477: Vulnerability in openssl](https://rustsec.org/advisories/RUSTSEC-2026-0477.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Buffer overflow with AES key wrap with padding in `cipher_update_inplace`
- [RUSTSEC-2026-0475: Vulnerability in openssl](https://rustsec.org/advisories/RUSTSEC-2026-0475.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Buffer overflow in `MdCtxRef::digest_final`
- [RUSTSEC-2026-0476: Vulnerability in openssl](https://rustsec.org/advisories/RUSTSEC-2026-0476.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Buffer overflow with AES key wrap with padding in `cipher_update`
- [RUSTSEC-2026-0473: Vulnerability in openssl](https://rustsec.org/advisories/RUSTSEC-2026-0473.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Buffer overflow in `Deriver::derive` and `PkeyCtxRef::derive` with OpenSSL 1.1.x
- [RUSTSEC-2026-0464: webhook_commands contained malicious code](https://rustsec.org/advisories/RUSTSEC-2026-0464.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `webhook_commands` was removed from crates.io for malicious code
- [RUSTSEC-2026-0465: electrumd contained malicious code](https://rustsec.org/advisories/RUSTSEC-2026-0465.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `electrumd` was removed from crates.io for malicious code

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-11-10 14:57 UTC
