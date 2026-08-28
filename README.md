# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 14 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 28 août 2026

- [Enabling the next-generation trait solver on nightly](https://blog.rust-lang.org/2026/08/21/enabling-next-solver-on-nightly/)  
  <sub>`rust-blog` · officiel · score 110</sub>  
  After nearly 4 years of active development, the next-generation trait solver is close to stabilization. We are enabling it by default on nightly to surface any remaining issues and plan to stabilize it in the next months. This is the largest single change to the Rust compiler since its initial release. It completely…
- [Announcing Rust 1.97.1](https://blog.rust-lang.org/2026/07/16/Rust-1.97.1/)  
  <sub>`rust-blog` · officiel · score 105</sub>  
  The Rust team has published a new point release of Rust, 1.97.1. Rust is a programming language that is empowering everyone to build reliable and efficient software. If you have a previous version of Rust installed via rustup, getting Rust 1.97.1 is as easy as: rustup update stable If you don't have it already, you…
- [Announcing Rust 1.96.1](https://blog.rust-lang.org/2026/06/30/Rust-1.96.1/)  
  <sub>`rust-blog` · officiel · score 105</sub>  
  The Rust team has published a new point release of Rust, 1.96.1. Rust is a programming language that is empowering everyone to build reliable and efficient software. If you have a previous version of Rust installed via rustup, getting Rust 1.96.1 is as easy as: rustup update stable If you don't have it already, you…
- [This Week in Rust 666](https://this-week-in-rust.org/blog/2026/08/26/this-week-in-rust-666/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [This Week in Rust 665](https://this-week-in-rust.org/blog/2026/08/19/this-week-in-rust-665/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [This Week in Rust 664](https://this-week-in-rust.org/blog/2026/08/12/this-week-in-rust-664/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [This Week in Rust 663](https://this-week-in-rust.org/blog/2026/08/05/this-week-in-rust-663/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [Announcing our first Maintainers in Residence](https://blog.rust-lang.org/2026/08/26/announcing-our-first-maintainers-in-residence/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  We are very happy to announce the Rust Project's first round of Maintainers in Residence: Gen Li (@rami3l), Chris Denton (@ChrisDenton), Alejandra González (@blyxyas), León Liehr (@fmease), and Maintainer Grant recipients: Jason Newcomb (@Jarcho) and Jonas Böttiger (@joboet). These contributors will be funded for…
- [Supply chain attack on arrayref](https://blog.rust-lang.org/2026/08/20/supply-chain-attack-on-arrayref/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  What happened On 2026-08-20 at 7:15 UTC we got a report that the proc-macro1 crate was malicious. The Rust Security Response Team verified this to be the case: the crate had a build script that was downloading a malicious payload. This crate proc-macro1 and others like it (proc-macro-en, aovine, arone, aronenao,…
- [Announcing Rust 1.98.0](https://blog.rust-lang.org/2026/08/20/Rust-1.98.0/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  The Rust team is happy to announce a new version of Rust, 1.98.0. Rust is a programming language empowering everyone to build reliable and efficient software. If you have a previous version of Rust installed via rustup, you can get 1.98.0 with: $ rustup update stable If you don't have it already, you can get rustup…

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 UTC · dernière trouvaille 2026-08-28 11:07 UTC
