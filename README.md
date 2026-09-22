# 🦀 Veille Rust

Veille automatisée sur l'écosystème Rust : 13 sources agrégées chaque jour par un collecteur écrit en Rust, tournant sur GitHub Actions.

**[→ Consulter le site](https://g1ts23.github.io/veille-rust/)** · [flux RSS](https://g1ts23.github.io/veille-rust/atom.xml) · [archives](content/digests/) · [mes notes](notes/)

---

## Top de la semaine — 22 septembre 2026

- [GitHub Actions leaking secrets when Miri output is cached](https://blog.rust-lang.org/2026/09/21/github-actions-leaking-secrets-when-miri-output-is-cached/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  The Rust Security Response Team was notified that Miri stores all environment variables to target/, allowing secrets to persist in caches. While not necessary a vulnerability in and of itself, when paired with GitHub Actions caching behavior, it is possible for this to expose secrets to PRs. Overview GitHub Actions…
- [Be alert: targeted attacks on prominent Rustaceans](https://blog.rust-lang.org/2026/09/17/targeted-attacks/)  
  <sub>`rust-blog` · officiel · score 100</sub>  
  We believe that there is an ongoing campaign targeting rust-lang members and owners of popular crates that is attempting to compromise devices and accounts in order to use them to publish malware. What we've seen A video call is set up for something positive — maybe for a job, maybe for a project, maybe for a contract…
- [This Week in Rust 669](https://this-week-in-rust.org/blog/2026/09/16/this-week-in-rust-669/)  
  <sub>`twir` · newsletter, must-read · score 100</sub>  
  Hello and welcome to another issue of This Week in Rust! Rust is a programming language empowering everyone to build reliable and efficient software. This is a weekly summary of its progress and community. Want something mentioned? Tag us at @thisweekinrust.bsky.social on Bluesky or @ThisWeekinRust on mastodon.social,…
- [RUSTSEC-2026-0298: Vulnerability in unicycle](https://rustsec.org/advisories/RUSTSEC-2026-0298.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Use-after-free when a future&apos;s `Drop` panics while the container is dropped
- [RUSTSEC-2026-0299: owned-alloc is unmaintained](https://rustsec.org/advisories/RUSTSEC-2026-0299.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `owned-alloc` is unmaintained
- [RUSTSEC-2026-0300: Vulnerability in skiplist](https://rustsec.org/advisories/RUSTSEC-2026-0300.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Use-after-free in `clear` and `retain` when an element&apos;s `Drop` panics
- [RUSTSEC-2026-0296: unzip is unmaintained](https://rustsec.org/advisories/RUSTSEC-2026-0296.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `unzip` is unmaintained
- [RUSTSEC-2026-0297: Vulnerability in unzip](https://rustsec.org/advisories/RUSTSEC-2026-0297.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  `unzip`: archive extraction is vulnerable to path traversal (zip-slip)
- [RUSTSEC-2026-0293: Vulnerability in ringbuf](https://rustsec.org/advisories/RUSTSEC-2026-0293.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Double free / use-after-free in `Consumer::skip` and `Consumer::clear` when an element&apos;s `Drop` panics
- [RUSTSEC-2026-0291: Vulnerability in owned-alloc](https://rustsec.org/advisories/RUSTSEC-2026-0291.html)  
  <sub>`rustsec` · securite · score 90</sub>  
  Double free in `OwnedAlloc::drop_in_place` when the contained value&apos;s `Drop` panics

---

## Fonctionnement

- `sources.toml` — la liste des sources et le scoring. **C'est le fichier à faire vivre.**
- [`SETUP.md`](SETUP.md) — installation, ajout d'une source, réglage du bruit, pièges connus.
- `collector/` — le collecteur (Rust) : fetch, dédup, scoring, rendu.
- `data/seen.jsonl` — index de dédup. `data/items/` — archive brute par mois.
- `content/digests/` — un digest par jour, publié via Zola sur GitHub Pages.
- `notes/` — les notes écrites à la main. C'est ce qui distingue ce repo d'un lecteur RSS.

Collecte quotidienne à 06:17 et 08:43 UTC · dernière trouvaille 2026-09-22 13:39 UTC
