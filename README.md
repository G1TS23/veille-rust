# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 16 septembre 2026

- [This Week in Rust 668](https://this-week-in-rust.org/blog/2026/09/09/this-week-in-rust-668/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0285: Vulnerability in rustls](https://rustsec.org/advisories/RUSTSEC-2026-0285.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  TLS 1.3 handshake messages incorrectly accepted across encryption level boundaries
- [RUSTSEC-2026-0284: Unsoundness in lockfree](https://rustsec.org/advisories/RUSTSEC-2026-0284.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Double free in `Map::into_iter` and an uninitialized `Arc` in `SharedIncin::clear`
- [RUSTSEC-2026-0283: clear_on_drop is unmaintained](https://rustsec.org/advisories/RUSTSEC-2026-0283.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  clear_on_drop is unmaintained
- [What is Rust’s equivalent of compiling C++ with -O3?](https://www.reddit.com/r/rust/comments/1werfhi/what_is_rusts_equivalent_of_compiling_c_with_o3/)  
  <sub>`reddit` · communaute · score 61</sub>  
  Hello everyone, I’m building an Axum API and want to optimize the release binary for runtime performance. In C++, I would use flags such as -O3, and possibly -march=native. I know cargo build --release enables optimizations, but are there additional Cargo profile settings or rustc flags worth using? What settings do…
- [Some things Veloren does differently](https://blog.jsbarretto.com/post/veloren)  
  <sub>`lobsters` · communaute · score 60</sub>  
  Comments
- [Ubuntu 26.10 completes transition to Rust-based coreutils](https://www.omgubuntu.co.uk/2026/09/ubuntu-2610-rust-coreutils-complete)  
  <sub>`lobsters` · communaute · score 60</sub>  
  Comments
- [Unsizing unsized values](https://hackmd.io/@WorldSEnder/Hkyqni6Ofl)  
  <sub>`lobsters` · communaute · score 60</sub>  
  Comments
- [txcript: Switching coding agents mid-conversation](https://github.com/skillsynchq/txcript)  
  <sub>`lobsters` · communaute · score 60</sub>  
  Comments
- [Trying to Make a Loop Auto-Vectorize](https://jsgroth.dev/blog/posts/trying-to-make-a-loop-auto-vectorize/)  
  <sub>`lobsters` · communaute · score 60</sub>  
  Comments

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-09-16 11:42 UTC
