# Public challenge: break the captcha made of moving dots

> 🌐 **Русская версия:** [CONTEST.md](CONTEST.md)

## What you get

- Full generator source (`generator/`) — parameters, font, timings and modes (`classic` / `jelly`) are open.
- A set of 10 labelled GIFs (`samples/`) — for local debugging.
- A hidden set (100 GIFs) for the final measurement — on request from the author.
- The web application (.NET + Rust) — live frame streaming: no finished file is served, the field reveals on hover.

## Goal

A solver that takes ONE GIF file and outputs the 4 digits.
- Allowed: any algorithms, CV, ML, training on your own generated data
  (the generator is open — generate as many examples as you like), any languages.
- Forbidden: reading the answer from the file/metadata, using the hidden set's
  labels, human farms (except the separate "human + tool" category).
- Time limit per GIF: 30 seconds (that is how long a challenge lives).

## Metric

Share of exact 4-digit matches on the hidden set of 100 GIFs.
References: the reference CV solver = 0/50; a strong CV solver (background mean + EM) = 66–83% on the
original build and 0% on the current one; the best honest community result = 4/100. The "captcha is
broken" threshold is 50%. Full history: `docs/ATTACK-HISTORY.en.md`.

## How to participate

1. Fork the repository / copy the generator.
2. Write a solver, run it on your own data.
3. Submit: the solver code, the run command and the result on the hidden set
   (the author will run it himself to rule out tuning to the labels).

## What happens with the results

- Public credit from the author (name/nick in the README).
- If the captcha is broken — we record it as a robustness result and note which
  parameters had to be changed to close it.

## Contacts

- ✉️ Email: [krasovskyworks@gmail.com](mailto:krasovskyworks@gmail.com)
- ✈️ Telegram: [@eugenekray](https://t.me/eugenekray)
- ☕ Boosty: [boosty.to/jevkray](https://boosty.to/jevkray)
