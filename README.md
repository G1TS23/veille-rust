# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 19 septembre 2026

- [Be alert: targeted attacks on prominent Rustaceans](https://blog.rust-lang.org/2026/09/17/targeted-attacks/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  We believe that there is an ongoing campaign targeting rust-lang members and owners of popular crates that is attempting to compromise devices and accounts in order to use them to publish malware. What we've seen A video call is set up for something positive — maybe for a job, maybe for a project, maybe for a contract…
- [This Week in Rust 669](https://this-week-in-rust.org/blog/2026/09/16/this-week-in-rust-669/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0288: Vulnerability in cosmian_kyber](https://rustsec.org/advisories/RUSTSEC-2026-0288.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  cosmian_kyber: AVX2 backend skips Fujisaki-Okamoto implicit rejection, enabling chosen-ciphertext key recovery
- [RUSTSEC-2026-0287: cosmian_kyber is unmaintained](https://rustsec.org/advisories/RUSTSEC-2026-0287.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  cosmian_kyber is unmaintained
- [RUSTSEC-2026-0290: Vulnerability in pqc_kyber](https://rustsec.org/advisories/RUSTSEC-2026-0290.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  pqc_kyber: AVX2 backend skips Fujisaki-Okamoto implicit rejection, enabling chosen-ciphertext key recovery
- [RUSTSEC-2026-0289: pqc_kyber is unmaintained](https://rustsec.org/advisories/RUSTSEC-2026-0289.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  pqc_kyber is unmaintained
- [RUSTSEC-2026-0286: Vulnerability in cryptoki](https://rustsec.org/advisories/RUSTSEC-2026-0286.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Out-of-bounds read when decoding CKA_ALLOWED_MECHANISMS
- [RUSTSEC-2026-0285: Vulnerability in rustls](https://rustsec.org/advisories/RUSTSEC-2026-0285.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  TLS 1.3 handshake messages incorrectly accepted across encryption level boundaries
- [RUSTSEC-2026-0284: Unsoundness in lockfree](https://rustsec.org/advisories/RUSTSEC-2026-0284.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Double free in `Map::into_iter` and an uninitialized `Arc` in `SharedIncin::clear`
- [RUSTSEC-2026-0283: clear_on_drop is unmaintained](https://rustsec.org/advisories/RUSTSEC-2026-0283.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  clear_on_drop is unmaintained

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-09-19 11:04 UTC
