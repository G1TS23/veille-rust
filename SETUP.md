# Mise en route

## 1. Créer le repo

```bash
cd "/Users/olivierfalahi/Documents/Perso/veille rust"
gh repo create veille-rust --public --source=. --remote=origin --push
```

Le compte et le nom du repo sont codés en dur à quatre endroits. Si tu changes
l'un des deux, mets-les à jour partout — sinon **toute la navigation du site
casse silencieusement** (le build réussit, les liens renvoient 404) :

| Fichier | Quoi |
|---|---|
| `config.toml` | `base_url` — hostname Pages, **en minuscules** |
| `templates/base.html` | les deux liens vers le repo |
| `collector/src/render.rs` | les URLs du README généré |
| `sources.toml` | le `user_agent` (crates.io exige un contact valide) |

Vérification rapide après coup : `curl -s <url-du-site> | grep -o 'href=\"[^\"]*github.io[^\"]*\"' | sort -u`

## 2. Activer GitHub Pages

**Settings → Pages → Source : `GitHub Actions`** (surtout pas « Deploy from a branch »).
Sans ça, `deploy-pages` échoue avec une erreur peu explicite.

Le site sera sur `https://<ton-user>.github.io/veille-rust/`.

## 3. Autoriser le bot à pousser

**Settings → Actions → General → Workflow permissions → « Read and write permissions »**.
Sinon le `git push` du workflow renvoie 403.

## 4. Premier run

Onglet **Actions → veille → Run workflow**. Coche `dry_run` pour un essai à blanc.

## 5. Optionnel — notification Discord

Serveur Discord → Paramètres du salon → Intégrations → Webhooks → Nouveau webhook → copier l'URL.

**Settings → Secrets and variables → Actions → New repository secret**
- Nom : `DISCORD_WEBHOOK_URL`
- Valeur : l'URL du webhook

Le step est automatiquement sauté tant que ce secret n'existe pas.

> Pour **Slack** à la place : le payload JSON `{"content": …}` devient `{"text": …}`
> dans `.github/workflows/veille.yml`, et le secret s'appelle `SLACK_WEBHOOK_URL`.
> Slack n'accepte pas la syntaxe `[titre](url)` : il faut `<url|titre>`.

## 6. Optionnel — label pour l'issue hebdo

```bash
gh label create veille --color 0E8A16 --description "Récap hebdomadaire"
```

Le workflow retombe sur une création sans label si celui-ci n'existe pas.

---

# Usage quotidien

```bash
# Essai à blanc : collecte et affiche, n'écrit rien
cargo run --release --manifest-path collector/Cargo.toml -- --dry-run

# Run complet en local (écrit data/, content/, README.md)
cargo run --release --manifest-path collector/Cargo.toml

# Récap hebdo dans data/weekly.md
cargo run --release --manifest-path collector/Cargo.toml -- --weekly

# Prévisualiser le site (zola doit être installé : brew install zola)
zola serve
```

## Ajouter une source

**Sonde-la d'abord.** La commande n'écrit rien et te dit tout ce qu'il faut
savoir avant de toucher à `sources.toml` :

```bash
cargo run --release --manifest-path collector/Cargo.toml -- \
  --check-source https://exemple.com/feed.xml [--weight 5]
```

Elle rapporte le code HTTP, les redirections (le flux a peut-être déménagé),
si `feed-rs` avale le contenu, le nombre d'entrées, la fraîcheur de la plus
récente, un échantillon avec les scores réels — et te sort le bloc `[[source]]`
prêt à coller.

Elle signale aussi trois pièges que l'œil ne voit pas :

| Avertissement | Ce que ça veut dire |
|---|---|
| `⚠ Flux probablement abandonné` | rien publié depuis plus de 180 jours |
| `⚠ n/n déjà collectés ailleurs` | la source recoupe une source existante |
| `⚠ Rien ne passerait le filtre` | le `weight` est trop bas pour ton `min_score` |

> `--check-source` ne sonde que les flux RSS/Atom (`kind = "feed"`).
> `hn_algolia` et `crates_io` n'ont pas d'URL à tester : ce sont des points
> d'API fixes, déjà configurés.

Puis le bloc dans `sources.toml` :

```toml
[[source]]
id     = "mon-blog"
kind   = "feed"        # feed | hn_algolia | crates_io
url    = "https://exemple.com/feed.xml"
tags   = ["blog"]
weight = 5             # score de base = weight × 10
limit  = 10            # optionnel, plafonne les flux bavards
```

### Référence complète des champs

| Champ | Requis | S'applique à | Rôle |
|---|---|---|---|
| `id` | ✅ | tous | identifiant court, affiché dans les digests |
| `kind` | ✅ | tous | `feed` \| `hn_algolia` \| `crates_io` |
| `url` | ✅ pour `feed` | `feed` | l'URL du flux RSS/Atom |
| `query` | ✅ pour `hn_algolia` | `hn_algolia` | le terme cherché sur Hacker News |
| `tags` | — | tous | thèmes ; le premier sert à grouper le digest |
| `weight` | — | tous | **score de base = `weight` × 10** |
| `limit` | — | tous | plafonne le nombre d'items retenus par run |
| `min_points` | — | `hn_algolia` | seuil de points HN sous lequel on ignore |
| `min_downloads` | — | `crates_io` | seuil de téléchargements sous lequel on ignore |

> ⚠️ **Le piège du `weight` omis.** Il n'a pas de valeur par défaut utile : s'il
> manque, il vaut `0`, donc le score de base vaut `0`, donc tout passe sous
> `min_score` — la source est collectée puis intégralement filtrée. Une source
> qui « ne remonte jamais rien » vient presque toujours de là.
>
> **Le collecteur le signale désormais au démarrage**, à chaque run :
>
> ```
> ⚠ `ma-source` : `weight` absent ou nul → score de base 0, sous min_score 30.
>   Cette source sera collectée puis entièrement filtrée.
> ⚠ `autre` : score de base 10 < min_score 30 — ne remontera que les items
>   cumulant des bonus de mots-clés.
> ```
>
> L'avertissement est silencieux quand la configuration est saine. Il ignore
> les sources `hn_algolia`, dont le bonus de points n'est pas plafonné.

Exemples pour les deux autres types :

```toml
[[source]]
id         = "hn-async"
kind       = "hn_algolia"
query      = "rust async"
tags       = ["communaute", "async"]
weight     = 6
min_points = 40          # ignore les stories sous 40 points

[[source]]
id            = "crates-new"
kind          = "crates_io"
tags          = ["crates"]
weight        = 3
min_downloads = 20000    # ignore les crates confidentiels
```

## Modifier le texte de l'issue hebdomadaire

Le corps de l'issue vit dans **`collector/templates/weekly.md`**. Il est lu à
l'exécution : tu l'édites, tu relances, **sans recompiler**.

```
Valeurs   {{date}} {{count}} {{top_count}} {{rest_count}} {{items}} {{rest}}
Blocs     [[#has_items]]…[[/has_items]]   affiché s'il y a des items
          [[#empty]]…[[/empty]]           affiché si la semaine est vide
          [[#has_rest]]…[[/has_rest]]     affiché s'il y a un reste replié
Lignes    [[#item]] et [[#rest_item]] définissent le format d'UNE ligne,
          avec {{title}} {{url}} {{source}} {{score}} {{tags}} {{summary}}
Commentaires  les lignes commençant par {{! ne sont jamais rendues
```

Le **titre** de l'issue (`Veille Rust — S35 2026`) et son **label** restent dans
`.github/workflows/digest.yml`, pas dans le template.

Pour prévisualiser sans ouvrir d'issue :

```bash
cargo run --release --manifest-path collector/Cargo.toml -- --weekly
cat data/weekly.md
```

## Le cas particulier de Hacker News

`hn_algolia` interroge la recherche plein texte d'Algolia sur `query = "rust"`.
Elle ne distingue pas le langage du mot courant, ni le titre du corps : des
articles sans rapport remontent régulièrement.

**`min_points` ne corrige pas ça.** Le seuil mesure la popularité sur HN, pas la
pertinence Rust — et les articles les plus votés sont justement les sujets
généralistes. Sur les 6 premiers items collectés :

| Points | Sujet | Rust ? |
|---|---|---|
| 294 | Just the rumour of a bug is enough to find an exploit | ambigu |
| 179 | Show HN: We built open OpenRouter… | ❌ |
| 135 | Meta Paid $17B – Gets to Write Safety Rules… | ❌ |
| 71 | **TurboKV: Insanely fast Rust key-value store** | ✅ |
| 70 | Some conservationists… Africa's wild dogs | ❌ |
| 51 | Tell HN: Man, AI is killing my brain | ❌ |

Monter le seuil à 80 écarte le **seul item clairement pertinent** et conserve
les deux plus hors-sujet. La précision ne s'améliore pas, seul le volume baisse.

Si le hors-sujet devient gênant, la vraie parade est de **retirer la source** :
`lobsters` interroge le tag `rust`, donc précis par construction, et couvre en
grande partie le même terrain.

## Régler le bruit

- **Trop d'items ?** monte `min_score` dans `sources.toml`, ou baisse le `weight` des sources bavardes.
- **Une source noie le reste ?** mets-lui un `limit`.
- **Rien ne remonte ?** `min_score` est trop haut : un item vaut `weight × 10` + bonus.

## Repartir de zéro

```bash
rm data/seen.jsonl              # tout sera re-collecté au prochain run
rm -rf data/items content/digests/*.md
```

---

# Points de vigilance

| Sujet | À savoir |
|---|---|
| **Cron désactivé** | GitHub coupe les `schedule` après 60 jours sans activité sur le repo. Les commits du bot suffisent à le garder actif — sauf si la veille ne trouve jamais rien. |
| **Retard du cron** | `17 6 * * *` et pas `0 6 * * *` : les heures rondes sont saturées, les jobs partent en retard ou sont abandonnés. |
| **Cron abandonné** | GitHub livre `schedule` « au mieux » : une exécution peut ne jamais avoir lieu, sans trace ni file d'attente — c'est arrivé le 2026-08-29, à la première occurrence. D'où **deux crons quotidiens** (06:17 et 08:43 UTC), espacés pour tomber dans des fenêtres de charge distinctes. Un second passage ne coûte rien : la dédup empêche tout doublon et un run sans nouveauté ne commite pas. |
| **Pas de chaînage** | Un push fait avec `GITHUB_TOKEN` ne déclenche aucun autre workflow. C'est pourquoi collecte, build et deploy sont dans le **même** workflow. |
| **Reddit en CI** | Reddit renvoie souvent 403 aux IP de datacenter. Chaque source est isolée : un échec est loggé, le run continue. |
| **Discord** | Cloudflare, devant Discord, renvoie **403 (code 1010)** sur le User-Agent par défaut de `urllib`. Le step envoie donc un UA explicite. Avec un vrai UA, un webhook invalide répond 404 `Unknown Webhook` — c'est ainsi qu'on distingue les deux causes. |
| **Notification non bloquante** | Le step Discord est en `continue-on-error`. Sans ça son échec fait échouer `collect`, et `deploy` est sauté : le site ne serait pas publié à cause d'un message raté. |
| **crates.io** | Exige un User-Agent identifiable (défini dans `sources.toml`) et fait évoluer son schéma JSON — `most_recently_updated` est devenu `just_updated`. |
| **Deux runs le même jour** | Le digest du jour est reconstruit depuis l'archive, pas depuis les seules nouveautés du run. |
| **Commits** | Un run qui ne trouve rien ne commite rien. `data/latest.json` et `data/weekly.md` sont des artefacts transitoires, volontairement gitignorés : suivis en git, leurs horodatages provoqueraient un commit quotidien vide. |
| **« dernière trouvaille »** | La date affichée en pied de README est celle du dernier item trouvé, pas celle du dernier passage du cron. C'est ce qui rend les runs à vide sans effet. |
