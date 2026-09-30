# 🎯 jevkray-captcha-v3

**🌐 语言:** [English](README.md) · [Русский](README.ru.md) · **中文** · [日本語](README.ja.md) · [Français](README.fr.md) · [Español](README.es.md)

**🚀 在线试用：** <https://jevkray.github.io/jevkray-captcha-v3/>

> 一种视觉验证码：4 位数字由**与背景噪点完全相同的点**构成——没有单独的文字图层。

![jevkray-captcha-v3](docs/screenshot.png)

---

## ✨ 这是什么

在 160×160 的场地上，约 9200 个彩色点组成密集马赛克并不断移动。数字只表现为**一组点**：它们汇成一个像“活果冻”一样运动的块——拉伸、呼吸、尺寸脉动，其上还有全局波纹与漩涡（“液态镜头”）。

单帧几乎无法与背景区分：数字笔画与背景中的点密度、寿命和出现频率都被刻意对齐——即 *zero knowledge per frame* 原则。在网页版中，画面以实时流传输：没有可下载的文件，未悬停时整个场地完全隐藏。

## ⚙️ 工作原理

| # | 过程 |
|---|---|
| 1 | 密集均匀的点阵：网格步长 2 px，点 1–2 px，6 种协调配色。 |
| 2 | 第一秒：数字不可见——数字点随背景一起运动。 |
| 3 | 随后 1.5 秒：数字点从随机位置飞入各自的格子（组装）。 |
| 4 | 之后文字像“果冻”一样存在：平移 + ±12° 旋转 + ±2.5 px“呼吸” + 笔画变形、格子撕裂/重新粘合以及 ±25% 尺寸脉动。 |
| 5 | 漂入字符格子的点会被“捕获”，开始随文字运动（不隐藏、无生灭跳变）。 |
| 6 | 其上叠加“液态镜头”（全局波纹与漩涡）；裂口与空斑在背景中漂移。 |
| 7 | 画面以 20 fps 流式传输，无法重复下载；答案在服务器端用加盐 SHA-256 校验；验证码有效期 30 秒。 |

## 🚀 快速开始

**Web 应用**（.NET 10）：`dotnet run --urls http://localhost:5199`
配置见 `appsettings.json` → `CapGen:Path`、`CapGen:Digits`、`CapGen:Fps`、`CapGen:Seconds`、`CapGen:Mode`（`classic` 或 `jelly`）。

**生成器**（Rust）：
```bash
cd generator
cargo build --release
./target/release/capgen.exe 4821 out.gif 20 30 jelly   # 代码, 文件, fps, 秒数, 模式（classic|jelly）；.bin 输出原始帧用于流式传输
```

## 🧪 如何测试（模型与人类）

1. 生成带标签的集合：`./generator/eval.ps1 -N 100`
2. 把 GIF（或 `samples/` 中的拼图）交给模型，并给出提示：

> 分析这段动画。其中隐藏着 4 位数字：它是唯一一组作为整体运动的点。请编写算法（Python + Pillow/PyAV），从 GIF 文件中读出 4 位数字，并在提供的样例上测量准确率。

3. 统计结果：`./metrics.ps1 -Csv answers.csv`

## 🏆 公开挑战

- **目标：** 单张 GIF 读出 4 位数字的求解器 —— 在隐藏的 100 张 GIF 上达到 ≥ **50%** 完全匹配。
- **允许：** 任何 CV/ML 方法，用自己的数据训练（生成器公开）。
- **禁止：** 从元数据读取答案、使用隐藏集标签、人工众包。
- 完整规则：[`CONTEST.md`](CONTEST.md)

## 🧠 求解器与攻击史

| 求解器 | 完全匹配 | 逐位准确率 |
|---|---|---|
| 参考 CV 攻击（`generator/src/bin/solve.rs`） | **0 / 50** | ~10–13%（随机） |
| `solve2.rs` — 背景均值 + EM（我们） | v8 上 50/66 · **当前 (v10) 0/10** | 87.5% → 25% |
| `solve3.rs` — flow 变体（已修复） | 当前 (v10) **0 / 10** | 25% |
| `solve2.rs` v2.1 — 尺度适配（WIP） | 0 / 10 | 25% |
| 社区（Z.ai、Dmitrii）— CV 尝试（v8 防护） | **4 / 100** | ~52% |

📖 完整解析：[`docs/ATTACK-HISTORY.md`](docs/ATTACK-HISTORY.md) — 求解器如何构建、得出什么结论、验证码如何加固，以及为何当前版本无法攻破。

## 📁 仓库结构

```
generator/         Rust：capgen（GIF/raw 生成器，classic/jelly 模式）+ solve.rs、solve2.rs、solve3.rs（求解器）
Pages/, wwwroot/   ASP.NET Core Razor Pages 网站（深色 UI，6 种语言）
samples/           10 个带标签的 GIF + 拼图（labels.csv 仅用于评分）
docs/              截图 + 攻击史（ATTACK-HISTORY.md）
eval.ps1           生成 N 个 GIF 并评估参考求解器
metrics.ps1        准确率 + “N 次尝试内成功”表
CONTEST.md         公开挑战规则
NOTICE.md          作者、日期、源码 SHA-256 指纹
```

## ⚖️ 许可与归属

© 2026 **jevkray**。保留所有权利。允许在注明出处的前提下进行研究、本地运行、测试与公开评测；商业使用需书面许可。详见 [`LICENSE.txt`](LICENSE.txt) 与 [`NOTICE.md`](NOTICE.md)。

## 🔗 链接

- GitHub: [github.com/Jevkray](https://github.com/Jevkray)
- 挑战包: [github.com/Jevkray/jevkray-captcha-v3](https://github.com/Jevkray/jevkray-captcha-v3)

## 📬 联系方式

- ✉️ Email: [krasovskyworks@gmail.com](mailto:krasovskyworks@gmail.com)
- ✈️ Telegram: [@eugenekray](https://t.me/eugenekray)
- ☕ Boosty: [boosty.to/jevkray](https://boosty.to/jevkray)
