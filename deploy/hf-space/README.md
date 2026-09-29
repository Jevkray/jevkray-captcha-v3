---
title: jevkray-captcha-v3
emoji: 🎯
colorFrom: gray
colorTo: indigo
sdk: docker
app_port: 7860
pinned: true
---

# jevkray-captcha-v3 — живая капча с Rust-генерацией

Это полная версия сервиса, а не демка с предрендеренными файлами:
каждый challenge генерируется **на сервере** утилитой `capgen` (Rust) в GIF
(160×160, 20 fps, 30 секунд), ответ проверяется по хэшу в сессионном кэше.

- Генератор: `generator/src/main.rs` (Rust, без внешних зависимостей кроме `gif`).
- Эталонная атака: `generator/src/bin/solve.rs` — 0 точных совпадений на 50 GIF.
- Цель публичного вызова: **50%+** точных совпадений.

Репозиторий с исходниками, правилами и примерами:
[github.com/Jevkray/jevkray-captcha-v3](https://github.com/Jevkray/jevkray-captcha-v3)

Контакты: krasovskyworks@gmail.com, Telegram [@eugenekray](https://t.me/eugenekray).