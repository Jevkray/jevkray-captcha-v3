# 🎯 jevkray-captcha-v3

**🌐 Idiomas:** [English](README.md) · [Русский](README.ru.md) · [中文](README.zh.md) · [日本語](README.ja.md) · [Français](README.fr.md) · **Español**

> Un CAPTCHA visual en el que el código de 4 dígitos se compone de **los mismos puntos que el ruido de fondo** — sin capa de texto aparte.

![jevkray-captcha-v3](docs/screenshot.png)

---

## ✨ Qué es esto

Sobre un campo de 160×160, ~9200 puntos de color forman un mosaico denso en movimiento. El código solo existe como **un grupo de puntos que se mueve como un único cuerpo rígido**: se traslada, gira lentamente y «respira».

Un solo fotograma es casi indistinguible del fondo: la densidad de puntos, la vida útil y la frecuencia de aparición en los trazos de los dígitos y en el fondo están igualadas a propósito — principio *zero knowledge per frame*.

## ⚙️ Cómo funciona

| # | Qué ocurre |
|---|---|
| 1 | Campo denso y uniforme: paso de rejilla 2 px, puntos 1–2 px, paleta de 6 colores armonizados. |
| 2 | Primer segundo: el código es invisible — los puntos de los dígitos siguen el flujo del fondo. |
| 3 | Los siguientes 1,5 s: los puntos vuelan desde posiciones aleatorias a sus celdas (ensamblaje). |
| 4 | Después el texto es el único cuerpo rígido: traslación + rotación ±12° + «respiración» ±2,5 px. |
| 5 | Un punto que entra en una celda del glifo es «capturado» y empieza a moverse con el texto (sin ocultación ni artefactos de aparición/desaparición). |
| 6 | La respuesta se verifica en el servidor con un hash SHA-256 con sal; el código vive 30 s. |

## 🚀 Inicio rápido

**Aplicación web** (.NET 10): `dotnet run --urls http://localhost:5199`
Configuración: `appsettings.json` → `CapGen:Path`, `CapGen:Digits`, `CapGen:Fps`, `CapGen:Seconds`.

**Generador** (Rust):

```bash
cd generator
cargo build --release
./target/release/capgen.exe 4821 out.gif 20 30   # código, archivo, fps, segundos
```

## 🧪 Cómo probarlo (modelos y humanos)

1. Crear un conjunto etiquetado: `./generator/eval.ps1 -N 100`
2. Dar el GIF (o las hojas de sprites de `samples/`) a un modelo con este prompt:

> Analiza la animación. Dentro hay un código de 4 dígitos: es el único grupo de puntos que se mueve como un solo cuerpo. Escribe un algoritmo (Python + Pillow/PyAV) que lea los 4 dígitos del archivo GIF y mide su precisión con los ejemplos proporcionados.

3. Calcular los resultados: `./metrics.ps1 -Csv answers.csv`

## 🏆 Reto público

- **Objetivo:** un solucionador que lea 4 dígitos de un solo GIF — ≥ **30 %** de coincidencias exactas en un conjunto oculto de 100 GIF.
- **Permitido:** cualquier enfoque CV/ML, entrenamiento con tus propios datos (el generador es abierto).
- **Prohibido:** leer la respuesta de los metadatos, usar las etiquetas del conjunto oculto, granjas humanas.
- Reglas completas: [`CONTEST.md`](CONTEST.md)

## 📊 Línea base (medida)

| Solucionador | Coincidencias exactas | Por dígito |
|---|---|---|
| Ataque CV de referencia (`generator/src/bin/solve.rs`) | **0 / 50** | ~10–13 % (azar) |
| El mismo ataque antes del endurecimiento (versión inicial) | 91 / 100 | 97,5 % |

## 📁 Estructura del repositorio

```
generator/         Rust: capgen (generador de GIF) + solve.rs (ataque de referencia)
Pages/, wwwroot/   aplicación ASP.NET Core Razor Pages (UI oscura, 6 idiomas)
samples/           6 GIF etiquetados + hojas de sprites (labels.csv solo para puntuar)
docs/              captura de pantalla
eval.ps1           genera N GIF y evalúa el solucionador de referencia
metrics.ps1        precisión + tabla de «éxito en N intentos»
CONTEST.md         reglas del reto público
NOTICE.md          autoría, fecha, huellas SHA-256 de las fuentes
```

## ⚖️ Licencia y propiedad

© 2026 **jevkray**. Todos los derechos reservados. Se permite estudiar, ejecutar localmente, probar y reseñar públicamente con atribución; el uso comercial requiere permiso escrito. Ver [`LICENSE.txt`](LICENSE.txt) y [`NOTICE.md`](NOTICE.md).

## 🔗 Enlaces

- GitHub: [github.com/Jevkray](https://github.com/Jevkray)
- Paquete del reto: [github.com/Jevkray/jevkray-captcha-v3](https://github.com/Jevkray/jevkray-captcha-v3)