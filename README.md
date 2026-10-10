# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 10 octobre 2026

- [This Week in Rust 672](https://this-week-in-rust.org/blog/2026/10/07/this-week-in-rust-672/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0334: Vulnerability in bip322](https://rustsec.org/advisories/RUSTSEC-2026-0334.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  BIP-322 address ownership verification bypass for P2WPKH and P2SH-P2WPKH addresses
- [RUSTSEC-2026-0333: Vulnerability in noyalib](https://rustsec.org/advisories/RUSTSEC-2026-0333.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Resource budgets not enforced on the typed deserialization path
- [RUSTSEC-2026-0332: Unsoundness in wasapi](https://rustsec.org/advisories/RUSTSEC-2026-0332.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `WaveFormat::parse` reads past the end of a `&WAVEFORMATEX`
- [RUSTSEC-2026-0329: Vulnerability in libcrux-hmac-drbg](https://rustsec.org/advisories/RUSTSEC-2026-0329.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Auto-Reseeding HMAC-DRBG could panic for some output lengths
- [RUSTSEC-2026-0330: Vulnerability in libcrux-kem](https://rustsec.org/advisories/RUSTSEC-2026-0330.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Hybrid Encapsulation from Seed Panics on Short Seed
- [RUSTSEC-2026-0331: Vulnerability in libcrux-kem](https://rustsec.org/advisories/RUSTSEC-2026-0331.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Panic on Decoding Short Hybrid Keys
- [Async Rust: Where does the scheduler live?](https://herecomesthemoon.net/2026/10/async-rust-where-does-the-scheduler-live/)  
  <sub>`lobsters` · communaute · score 68</sub>  
  Comments
- [The Performance Cost of RwLock in Our Read-Heavy Workload](https://pranitha.dev/posts/rwlock-vs-lockfree/)  
  <sub>`lobsters` · communaute · score 66</sub>  
  Comments
- [Mold 3.0.0 Released](https://github.com/rui314/mold/releases/tag/v3.0.0)  
  <sub>`lobsters` · communaute · score 65</sub>  
  Comments

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-10-10 12:37 UTC
