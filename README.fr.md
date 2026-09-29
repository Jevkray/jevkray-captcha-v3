# 🎯 jevkray-captcha-v3

**🌐 Langues :** [English](README.md) · [Русский](README.ru.md) · [中文](README.zh.md) · [日本語](README.ja.md) · **Français** · [Español](README.es.md)

> Un CAPTCHA visuel où le code à 4 chiffres est composé **des mêmes points que le bruit de fond** — aucun calque de texte séparé.

![jevkray-captcha-v3](docs/screenshot.png)

---

## ✨ De quoi s'agit-il

Sur un champ de 160×160, ~9200 points colorés forment une mosaïque dense en mouvement. Le code n'existe que comme **un groupe de points se déplaçant comme un seul corps rigide** : translation, rotation lente et « respiration ».

Une image seule est presque indiscernable du fond : densité des points, durée de vie et fréquence d'apparition dans les traits des chiffres et dans le fond sont volontairement égalisées — principe *zero knowledge per frame*.

## ⚙️ Comment ça marche

| # | Étape |
|---|---|
| 1 | Champ dense et uniforme : pas de grille 2 px, points 1–2 px, palette de 6 couleurs harmonieuses. |
| 2 | Première seconde : le code est invisible — les points des chiffres suivent simplement le flux du fond. |
| 3 | Les 1,5 s suivantes : les points s'envolent depuis des positions aléatoires vers leurs cellules (assemblage). |
| 4 | Ensuite le texte est le seul corps rigide : translation + rotation ±12° + « respiration » ±2,5 px. |
| 5 | Un point qui dérive dans une cellule est « capturé » et se met à bouger avec le texte (aucun masquage, aucun artefact d'apparition/disparition). |
| 6 | La réponse est vérifiée côté serveur par un hachage SHA-256 salé ; le code vit 30 s. |

## 🚀 Démarrage rapide

**Application web** (.NET 10) : `dotnet run --urls http://localhost:5199`
Configuration : `appsettings.json` → `CapGen:Path`, `CapGen:Digits`, `CapGen:Fps`, `CapGen:Seconds`.

**Générateur** (Rust) :

```bash
cd generator
cargo build --release
./target/release/capgen.exe 4821 out.gif 20 30   # code, fichier, fps, secondes
```

## 🧪 Comment tester (modèles et humains)

1. Créer un jeu étiqueté : `./generator/eval.ps1 -N 100`
2. Donner le GIF (ou les planches de sprites de `samples/`) à un modèle avec ce prompt :

> Analyse cette animation. Un code à 4 chiffres y est caché : c'est le seul groupe de points se déplaçant comme un corps unique. Écris un algorithme (Python + Pillow/PyAV) qui lit les 4 chiffres du fichier GIF et mesure sa précision sur les exemples fournis.

3. Calculer les résultats : `./metrics.ps1 -Csv answers.csv`

## 🏆 Défi public

- **Objectif :** un solveur qui lit 4 chiffres dans un seul GIF — ≥ **30 %** de correspondances exactes sur un jeu caché de 100 GIF.
- **Autorisé :** toute approche CV/ML, entraînement sur vos propres données (le générateur est ouvert).
- **Interdit :** lire la réponse dans les métadonnées, utiliser les étiquettes du jeu caché, fermes humaines.
- Règles complètes : [`CONTEST.md`](CONTEST.md)

## 📊 Référence (mesurée)

| Solveur | Correspondances exactes | Par chiffre |
|---|---|---|
| Attaque CV de référence (`generator/src/bin/solve.rs`) | **0 / 50** | ~10–13 % (hasard) |
| La même attaque avant durcissement (version initiale) | 91 / 100 | 97,5 % |

## 📁 Structure du dépôt

```
generator/         Rust : capgen (générateur de GIF) + solve.rs (attaque de référence)
Pages/, wwwroot/   application ASP.NET Core Razor Pages (UI sombre, 6 langues)
samples/           6 GIF étiquetés + planches de sprites (labels.csv sert à la notation)
docs/              capture d'écran
eval.ps1           génère N GIF et évalue le solveur de référence
metrics.ps1        précision + tableau « succès en N tentatives »
CONTEST.md         règles du défi public
NOTICE.md          paternité, date, empreintes SHA-256 des sources
```

## ⚖️ Licence et propriété

© 2026 **jevkray**. Tous droits réservés. Étude, exécution locale, tests et revues publiques autorisés avec attribution ; usage commercial sur autorisation écrite. Voir [`LICENSE.txt`](LICENSE.txt) et [`NOTICE.md`](NOTICE.md).

## 🔗 Liens

- GitHub : [github.com/Jevkray](https://github.com/Jevkray)
- Paquet du défi : [github.com/Jevkray/jevkray-captcha-v3](https://github.com/Jevkray/jevkray-captcha-v3)