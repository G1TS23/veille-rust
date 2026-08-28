# Notes

Le dossier qui fait la différence entre ce repo et un lecteur RSS.

Le collecteur remplit `content/digests/` tout seul. **Ici, tu écris à la main** :
ce que tu as retenu d'un article, pourquoi une RFC compte, un bout de code testé,
une conclusion après avoir essayé un crate.

Convention suggérée : un fichier par sujet, pas par date.

```
notes/
├── async-runtimes.md      # ce que je comprends de tokio vs smol vs glommio
├── borrow-checker.md      # notes au fil des évolutions de NLL / Polonius
└── crates-testes.md       # crates essayés, verdict, date
```

Rien n'est automatisé ici, et c'est volontaire.
