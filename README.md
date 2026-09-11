# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 11 septembre 2026

- [This Week in Rust 668](https://this-week-in-rust.org/blog/2026/09/09/this-week-in-rust-668/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [Rust debugging survey 2026 results](https://blog.rust-lang.org/2026/09/07/rust-debugging-survey-2026-results/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  One of the biggest challenges Rust developers report in our annual surveys is a subpar debugging experience. So, back in February, we ran our first Rust Debugging Survey, in the hopes of identifying how Rust developers are using debuggers and what problems they are facing when doing so. We received over 2,300…
- [RUSTSEC-2026-0282: Vulnerability in aligned_box](https://rustsec.org/advisories/RUSTSEC-2026-0282.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Double free in `AlignedBox<\[T\]>::realloc_with_default` when an element&apos;s `Drop` panics
- [RUSTSEC-2026-0280: greentic-setup-dev contained malicious code](https://rustsec.org/advisories/RUSTSEC-2026-0280.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `greentic-setup-dev` 1.3.34027618345 was removed from crates.io due to containing malicious code
- [RUSTSEC-2026-0281: greentic-setup contained malicious code](https://rustsec.org/advisories/RUSTSEC-2026-0281.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `greentic-setup` 1.3.1-dev.34027618345 was removed from crates.io due to containing malicious code
- [RUSTSEC-2026-0279: Vulnerability in rojo](https://rustsec.org/advisories/RUSTSEC-2026-0279.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Rojo development server vulnerable to DNS rebinding, allowing unauthenticated read/write access and local program execution
- [Welcome Dongpo and Ross to the Cargo team](https://blog.rust-lang.org/inside-rust/2026/09/08/welcome-dongpo-and-ross-to-the-cargo-team/)  
  <sub>`inside-rust` · officiel, compilo · score 80</sub>  
  We are excited to welcome Dongpo Liu and Ross Sullivan to the Cargo team! Dongpo has contributed to the Rust Project for years across different teams. He has been an important bridge between Cargo and other Rust teams and has brought insights and fresh ideas from across the Project. Dongpo also integrated cargo info…
- [Rust Is Tier-1 Language at Microsoft](https://rustfoundation.org/media/guest-post-rust-is-tier-1-language-at-microsoft/)  
  <sub>`lobsters` · communaute · score 60</sub>  
  Comments
- [Introducing CUDA Rust: Two Tracks for Writing GPU Kernels](https://developer.nvidia.com/blog/introducing-cuda-rust-two-tracks-for-writing-gpu-kernels/)  
  <sub>`lobsters` · communaute · score 60</sub>  
  Note that a PDF also exists: https://arxiv.org/pdf/2606.15991 Comments
- [A decade of rustls](https://rustls.dev/blog/2026-09-08-a-decade-of-rustls/)  
  <sub>`lobsters` · communaute · score 60</sub>  
  Context: rustls is the TLS implementation in Rust. Comments

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-09-11 11:26 UTC
