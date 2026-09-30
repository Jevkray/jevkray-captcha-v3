---
title: jevkray-captcha-v3
emoji: 🎯
colorFrom: gray
colorTo: indigo
sdk: docker
app_port: 7860
pinned: true
---

# jevkray-captcha-v3 — live captcha with Rust generation

This is the full service, not a demo with pre-rendered files: every challenge is generated
**on the server** by the `capgen` tool (Rust), and the frames are **streamed live** (no
downloadable file; the field reveals on hover). The answer is verified against a salted hash
in the session cache.

- Generator: `generator/src/main.rs` (Rust, no dependencies besides `gif`).
- Reference attack: `generator/src/bin/solve.rs` — 0 exact matches on 50 GIFs.
- Strong in-house attack: `generator/src/bin/solve2.rs` — 66–83% on v8, 0% on the current v10.
- Public challenge goal: **50%+** exact matches.
- Full attack history: [docs/ATTACK-HISTORY.en.md](../../docs/ATTACK-HISTORY.en.md).

Repository with sources, rules and samples:
[github.com/Jevkray/jevkray-captcha-v3](https://github.com/Jevkray/jevkray-captcha-v3)

Contacts: krasovskyworks@gmail.com, Telegram [@eugenekray](https://t.me/eugenekray).

---

# jevkray-captcha-v3 — живая капча с Rust-генерацией

Это полная версия сервиса, а не демка с предрендеренными файлами: каждый challenge
генерируется **на сервере** утилитой `capgen` (Rust), кадры транслируются **потоком**
(готовый файл не отдаётся; поле открывается при наведении). Ответ проверяется по хэшу
в сессионном кэше.

- Генератор: `generator/src/main.rs` (Rust, без внешних зависимостей кроме `gif`).
- Эталонная атака: `generator/src/bin/solve.rs` — 0 точных совпадений на 50 GIF.
- Сильная внутренняя атака: `generator/src/bin/solve2.rs` — 66–83% на v8, 0% на текущей v10.
- Цель публичного вызова: **50%+** точных совпадений.
- Полная хроника атак: [docs/ATTACK-HISTORY.md](../../docs/ATTACK-HISTORY.md) · [EN](../../docs/ATTACK-HISTORY.en.md).

Репозиторий с исходниками, правилами и примерами:
[github.com/Jevkray/jevkray-captcha-v3](https://github.com/Jevkray/jevkray-captcha-v3)

Контакты: krasovskyworks@gmail.com, Telegram [@eugenekray](https://t.me/eugenekray).
