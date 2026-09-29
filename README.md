# 🎯 jevkray-captcha-v3

**🌐 Languages:** **English** · [Русский](README.ru.md) · [中文](README.zh.md) · [日本語](README.ja.md) · [Français](README.fr.md) · [Español](README.es.md)

**🚀 Try it live:** <https://jevkray.github.io/jevkray-captcha-v3/>

> A visual CAPTCHA where the 4-digit code is assembled from **the very same dots as the background noise** — there is no separate text layer.

![jevkray-captcha-v3](docs/screenshot.png)

---

## ✨ What is this

Over a 160×160 field, ~9200 colored dots drift in a dense mosaic. The code exists only as a **group of dots that moves as a single rigid body**: it translates, slowly rotates and “breathes”.

A single frame is almost indistinguishable from the background: dot density, lifetime and appearance rate inside the glyph strokes and in the background are deliberately equalized — the *zero knowledge per frame* principle.

## ⚙️ How it works

| # | What happens |
|---|---|
| 1 | Dense uniform dot field: grid step 2 px, dots 1–2 px, palette of 6 matching colors. |
| 2 | First second: the code is invisible — glyph dots just follow the background flow. |
| 3 | Next 1.5 s: glyph dots fly from random places into their cells (the assembly). |
| 4 | After that the text is the only rigid body: translation + ±12° rotation + ±2.5 px “breathing”. |
| 5 | A dot drifting into a glyph cell is captured and starts moving with the text (no hiding, no spawn/death artifacts). |
| 6 | The answer is verified against a salted SHA-256 hash on the server; the code lives 30 s. |

## 🚀 Quick start

**Web app** (.NET 10):

```bash
dotnet run --urls http://localhost:5199
```

`appsettings.json` → `CapGen:Path` (generator binary), `CapGen:Digits`, `CapGen:Fps`, `CapGen:Seconds`.

**Generator** (Rust):

```bash
cd generator
cargo build --release
./target/release/capgen.exe 4821 out.gif 20 30   # code, output, fps, seconds
```

## 🧪 Testing it (models & humans)

1. Make a labelled set: `./generator/eval.ps1 -N 100`
2. Give the GIF (or the sprite sheets in `samples/`) to a model with this prompt:

> Analyse the animation. A 4-digit code is hidden inside: it is the only group of dots moving as a single body. Write an algorithm (Python + Pillow/PyAV) that reads the 4 digits from the GIF and measure its accuracy on the provided samples.

3. Score the answers: `./metrics.ps1 -Csv answers.csv`

## 🏆 Public challenge

- **Goal:** a solver that reads 4 digits from a single GIF — ≥ **30%** exact matches on a hidden 100-GIF set.
- **Allowed:** any CV/ML approach, training on your own generated data (the generator is open).
- **Forbidden:** reading answers from metadata, using the hidden set’s labels, human farms.
- Full rules: [`CONTEST.md`](CONTEST.md)

## 📊 Baseline (measured)

| Solver | Exact matches | Per digit |
|---|---|---|
| Reference CV attack (`generator/src/bin/solve.rs`) | **0 / 50** | ~10–13% (chance) |
| The same attack before hardening (early design) | 91 / 100 | 97.5% |

## 📁 Repository layout

```
generator/         Rust: capgen (GIF generator) + solve.rs (reference attacker)
Pages/, wwwroot/   ASP.NET Core Razor Pages web app (dark UI, 6 languages)
samples/           6 labelled GIFs + sprite sheets (labels.csv is for scoring only)
docs/              screenshot
eval.ps1           generate N GIFs and score the reference solver
sprite.ps1         GIF -> sprite sheet PNG (with frame range)
metrics.ps1        accuracy + “success in N attempts” table
CONTEST.md         public challenge rules
NOTICE.md          authorship, date, SHA-256 fingerprints of sources
```

## ⚖️ License & ownership

© 2026 **jevkray**. All rights reserved. Study, local runs, testing and public reviews are allowed with attribution; commercial use requires written permission. See [`LICENSE.txt`](LICENSE.txt) and [`NOTICE.md`](NOTICE.md) (dated fingerprints for priority).

## 🔗 Links

- GitHub: [github.com/Jevkray](https://github.com/Jevkray)
- Challenge bundle: [github.com/Jevkray/jevkray-captcha-v3](https://github.com/Jevkray/jevkray-captcha-v3)

## 📬 Contact

- ✉️ Email: [krasovskyworks@gmail.com](mailto:krasovskyworks@gmail.com)
- ✈️ Telegram: [@eugenekray](https://t.me/eugenekray)
- ☕ Boosty: [boosty.to/jevkray](https://boosty.to/jevkray)
