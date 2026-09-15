# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 15 septembre 2026

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
- [RUSTSEC-2026-0282: Vulnerability in aligned_box](https://rustsec.org/advisories/RUSTSEC-2026-0282.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Double free in `AlignedBox<\[T\]>::realloc_with_default` when an element&apos;s `Drop` panics
- [Welcome Dongpo and Ross to the Cargo team](https://blog.rust-lang.org/inside-rust/2026/09/08/welcome-dongpo-and-ross-to-the-cargo-team/)  
  <sub>`inside-rust` · officiel, compilo · score 80</sub>  
  We are excited to welcome Dongpo Liu and Ross Sullivan to the Cargo team! Dongpo has contributed to the Rust Project for years across different teams. He has been an important bridge between Cargo and other Rust teams and has brought insights and fresh ideas from across the Project. Dongpo also integrated cargo info…
- [What is Rust’s equivalent of compiling C++ with -O3?](https://www.reddit.com/r/rust/comments/1werfhi/what_is_rusts_equivalent_of_compiling_c_with_o3/)  
  <sub>`reddit` · communaute · score 61</sub>  
  Hello everyone, I’m building an Axum API and want to optimize the release binary for runtime performance. In C++, I would use flags such as -O3, and possibly -march=native. I know cargo build --release enables optimizations, but are there additional Cargo profile settings or rustc flags worth using? What settings do…
- [Speeding up gearhash on ARM64 (2× faster)](https://sam.dev/blog/gearhash-on-arm64)  
  <sub>`lobsters` · communaute · score 60</sub>  
  Comments
- [Stabilizing Rust's never type](https://lwn.net/SubscriberLink/1091015/d9e48318ed242b41/)  
  <sub>`lobsters` · communaute · score 60</sub>  
  Comments
- [Optimizing a single Rust Clippy lint by 3133X](https://blog.goose.love/posts/making-a-clippy-lint-faster-by-3133x/)  
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

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-09-15 13:51 UTC
