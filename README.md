# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 21 septembre 2026

- [Be alert: targeted attacks on prominent Rustaceans](https://blog.rust-lang.org/2026/09/17/targeted-attacks/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  We believe that there is an ongoing campaign targeting rust-lang members and owners of popular crates that is attempting to compromise devices and accounts in order to use them to publish malware. What we've seen A video call is set up for something positive — maybe for a job, maybe for a project, maybe for a contract…
- [This Week in Rust 669](https://this-week-in-rust.org/blog/2026/09/16/this-week-in-rust-669/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0293: Vulnerability in ringbuf](https://rustsec.org/advisories/RUSTSEC-2026-0293.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Double free / use-after-free in `Consumer::skip` and `Consumer::clear` when an element&apos;s `Drop` panics
- [RUSTSEC-2026-0291: Vulnerability in owned-alloc](https://rustsec.org/advisories/RUSTSEC-2026-0291.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Double free in `OwnedAlloc::drop_in_place` when the contained value&apos;s `Drop` panics
- [RUSTSEC-2026-0292: Vulnerability in imbl-sized-chunks](https://rustsec.org/advisories/RUSTSEC-2026-0292.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Double free / use-after-free in `Chunk` and `InlineArray` removal methods when an element&apos;s `Drop` panics
- [RUSTSEC-2026-0294: Unsoundness in iceoryx2-bb-container](https://rustsec.org/advisories/RUSTSEC-2026-0294.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Unsoundness in UTF-8 &apos;String&apos; trait
- [RUSTSEC-2026-0295: Vulnerability in z3](https://rustsec.org/advisories/RUSTSEC-2026-0295.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Memory corruption bug on `ApplyResult` type
- [RUSTSEC-2026-0288: Vulnerability in cosmian_kyber](https://rustsec.org/advisories/RUSTSEC-2026-0288.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  cosmian_kyber: AVX2 backend skips Fujisaki-Okamoto implicit rejection, enabling chosen-ciphertext key recovery
- [RUSTSEC-2026-0287: cosmian_kyber is unmaintained](https://rustsec.org/advisories/RUSTSEC-2026-0287.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  cosmian_kyber is unmaintained
- [RUSTSEC-2026-0290: Vulnerability in pqc_kyber](https://rustsec.org/advisories/RUSTSEC-2026-0290.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  pqc_kyber: AVX2 backend skips Fujisaki-Okamoto implicit rejection, enabling chosen-ciphertext key recovery

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-09-21 12:54 UTC
