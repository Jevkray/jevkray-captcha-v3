# 🎯 jevkray-captcha-v3

**🌐 Idiomas:** [English](README.md) · [Русский](README.ru.md) · [中文](README.zh.md) · [日本語](README.ja.md) · [Français](README.fr.md) · **Español**

**🚀 Pruébalo en línea:** <https://jevkray.github.io/jevkray-captcha-v3/>

> Un CAPTCHA visual en el que el código de 4 dígitos se compone de **los mismos puntos que el ruido de fondo** — sin capa de texto aparte.

![jevkray-captcha-v3](docs/screenshot.png)

---

## ✨ Qué es esto

Sobre un campo de 160×160, ~9200 puntos de color forman un mosaico denso en movimiento. El código solo existe como **un grupo de puntos**: se ensambla en un bloque móvil que se comporta como una gelatina viva — se estira, respira, pulsa de tamaño, mientras por encima pasan ondas y vórtices globales (una «lente líquida»).

Un solo fotograma es casi indistinguible del fondo: la densidad de puntos, la vida útil y la frecuencia de aparición en los trazos de los dígitos y en el fondo están igualadas a propósito — principio *zero knowledge per frame*. En la versión web los fotogramas llegan en flujo en tiempo real: no hay archivo descargable, y sin pasar el ratón el campo queda totalmente oculto.

## ⚙️ Cómo funciona

| # | Qué ocurre |
|---|---|
| 1 | Campo denso y uniforme: paso de rejilla 2 px, puntos 1–2 px, paleta de 6 colores armonizados. |
| 2 | Primer segundo: el código es invisible — los puntos de los dígitos siguen el flujo del fondo. |
| 3 | Los siguientes 1,5 s: los puntos vuelan desde posiciones aleatorias a sus celdas (ensamblaje). |
| 4 | Después el texto vive como una gelatina: traslación + rotación ±12° + «respiración» ±2,5 px + deformación de trazos, desgarros/reglues de celdas y pulsación de tamaño ±25 %. |
| 5 | Un punto que entra en una celda del glifo es «capturado» y empieza a moverse con el texto (sin ocultación ni artefactos de aparición/desaparición). |
| 6 | Encima — una «lente líquida» (ondas globales y vórtices); desgarros y huecos vagan por el fondo. |
| 7 | Los fotogramas se transmiten en flujo a 20 fps y no se pueden volver a descargar; la respuesta se verifica con un hash SHA-256 con sal en el servidor; el código vive 30 s. |

## 🚀 Inicio rápido

**Aplicación web** (.NET 10): `dotnet run --urls http://localhost:5199`
Configuración: `appsettings.json` → `CapGen:Path`, `CapGen:Digits`, `CapGen:Fps`, `CapGen:Seconds`, `CapGen:Mode` (`classic` o `jelly`).

**Generador** (Rust):

```bash
cd generator
cargo build --release
./target/release/capgen.exe 4821 out.gif 20 30 jelly   # código, archivo, fps, segundos, modo (classic|jelly); la extensión .bin escribe fotogramas crudos para el streaming
```

## 🧪 Cómo probarlo (modelos y humanos)

1. Crear un conjunto etiquetado: `./generator/eval.ps1 -N 100`
2. Dar el GIF (o las hojas de sprites de `samples/`) a un modelo con este prompt:

> Analiza la animación. Dentro hay un código de 4 dígitos: es el único grupo de puntos que se mueve como un solo cuerpo. Escribe un algoritmo (Python + Pillow/PyAV) que lea los 4 dígitos del archivo GIF y mide su precisión con los ejemplos proporcionados.

3. Calcular los resultados: `./metrics.ps1 -Csv answers.csv`

## 🏆 Reto público

- **Objetivo:** un solucionador que lea 4 dígitos de un solo GIF — ≥ **50 %** de coincidencias exactas en un conjunto oculto de 100 GIF.
- **Permitido:** cualquier enfoque CV/ML, entrenamiento con tus propios datos (el generador es abierto).
- **Prohibido:** leer la respuesta de los metadatos, usar las etiquetas del conjunto oculto, granjas humanas.
- Reglas completas: [`CONTEST.en.md`](CONTEST.en.md)

## 🧠 Solucionadores e historial de ataques

| Solucionador | Coincidencias exactas | Por dígito |
|---|---|---|
| Ataque CV de referencia (`generator/src/bin/solve.rs`) | **0 / 50** | ~10–13 % (azar) |
| `solve2.rs` — media + EM (nuestro) | 50/66 en v8 · **0/10 en la actual (v10)** | 87,5 % → 25 % |
| `solve3.rs` — variante flow (corregida) | **0 / 10** en la actual (v10) | 25 % |
| `solve2.rs` v2.1 — adaptación de escala (WIP) | 0 / 10 | 25 % |
| Comunidad (Z.ai, Dmitrii) — intentos CV (protección v8) | **4 / 100** | ~52 % |

📖 Análisis completo: [`docs/ATTACK-HISTORY.en.md`](docs/ATTACK-HISTORY.en.md) — cómo se construyeron los solucionadores, conclusiones, endurecimiento de la captcha y por qué la versión actual resiste.

## 📁 Estructura del repositorio

```
generator/         Rust: capgen (GIF/raw, modos classic/jelly) + solve.rs, solve2.rs, solve3.rs (solucionadores)
Pages/, wwwroot/   aplicación ASP.NET Core Razor Pages (UI oscura, 6 idiomas)
samples/           10 GIF etiquetados + hojas de sprites (labels.csv solo para puntuar)
docs/              captura de pantalla + historial de ataques (ATTACK-HISTORY.en.md)
eval.ps1           genera N GIF y evalúa el solucionador de referencia
metrics.ps1        precisión + tabla de «éxito en N intentos»
CONTEST.md         reglas del reto público
NOTICE.md          autoría, fecha, huellas SHA-256 de las fuentes
```

## ⚖️ Licencia y propiedad

© 2026 **jevkray**. Todos los derechos reservados. Se permite estudiar, ejecutar localmente, probar y reseñar públicamente con atribución; el uso comercial requiere permiso escrito. Ver [`LICENSE.en.txt`](LICENSE.en.txt) y [`NOTICE.en.md`](NOTICE.en.md).

## 🔗 Enlaces

- GitHub: [github.com/Jevkray](https://github.com/Jevkray)
- Paquete del reto: [github.com/Jevkray/jevkray-captcha-v3](https://github.com/Jevkray/jevkray-captcha-v3)

## 📬 Contacto

- ✉️ Email: [krasovskyworks@gmail.com](mailto:krasovskyworks@gmail.com)
- ✈️ Telegram: [@eugenekray](https://t.me/eugenekray)
- ☕ Boosty: [boosty.to/jevkray](https://boosty.to/jevkray)
