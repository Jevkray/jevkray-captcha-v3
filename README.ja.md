# 🎯 jevkray-captcha-v3

**🌐 言語:** [English](README.md) · [Русский](README.ru.md) · [中文](README.zh.md) · **日本語** · [Français](README.fr.md) · [Español](README.es.md)

> 4 桁のコードが**背景ノイズとまったく同じ点**で構成される視覚 CAPTCHA です。文字のレイヤーはありません。

![jevkray-captcha-v3](docs/screenshot.png)

---

## ✨ これは何か

160×160 の領域を約 9200 個の色付きの点が密なモザイクとして動き続けます。コードは**ひとつの剛体として動く点の集まり**としてのみ存在し、並進し、ゆっくり回転し、「呼吸」します。

1 フレームだけでは背景とほぼ区別できません。数字のストロークと背景で、点の密度・寿命・出現頻度が意図的に揃えられています（*zero knowledge per frame* の原則）。

## ⚙️ 仕組み

| # | 動作 |
|---|---|
| 1 | 密で均一な点の場：グリッド間隔 2 px、点 1–2 px、調和の取れた 6 色。 |
| 2 | 最初の 1 秒：コードは不可視 — 数字の点は背景と同じ流れで動きます。 |
| 3 | 続く 1.5 秒：数字の点がランダムな位置から各セルへ飛び込みます（組み立て）。 |
| 4 | その後、テキストは唯一の剛体：並進 + ±12° 回転 + ±2.5 px の「呼吸」。 |
| 5 | 文字のセルに入った点は「捕獲」され、テキストと一緒に動き始めます（隠蔽なし、生成/消滅のアーティファクトなし）。 |
| 6 | 回答はサーバー側でソルト付き SHA-256 により照合され、コードの寿命は 30 秒です。 |

## 🚀 クイックスタート

**Web アプリ**（.NET 10）：`dotnet run --urls http://localhost:5199`
設定：`appsettings.json` → `CapGen:Path` / `CapGen:Digits` / `CapGen:Fps` / `CapGen:Seconds`

**生成器**（Rust）：
```bash
cd generator
cargo build --release
./target/release/capgen.exe 4821 out.gif 20 30   # コード, 出力, fps, 秒
```

## 🧪 テスト方法（モデルと人間向け）

1. ラベル付きセットを作成：`./generator/eval.ps1 -N 100`
2. GIF（または `samples/` のスプライトシート）をモデルに渡し、次のプロンプトを与えます：

> このアニメーションを解析してください。内部に 4 桁のコードが隠されています。それは全体として動く唯一の点の集まりです。GIF ファイルから 4 桁を読み取るアルゴリズム（Python + Pillow/PyAV）を書き、提供されたサンプルで精度を測定してください。

3. 結果を集計：`./metrics.ps1 -Csv answers.csv`

## 🏆 公開チャレンジ

- **目標：** 1 つの GIF から 4 桁を読むソルバー — 非公開の 100 GIF で **30%** 以上の完全一致。
- **許可：** 任意の CV/ML 手法、自分のデータでの学習（生成器は公開）。
- **禁止：** メタデータからの答え読み取り、非公開セットのラベル使用、人力ファーム。
- 詳細：[`CONTEST.md`](CONTEST.md)

## 📊 ベースライン（実測）

| ソルバー | 完全一致 | 桁ごと |
|---|---|---|
| 参照 CV 攻撃（`generator/src/bin/solve.rs`） | **0 / 50** | ~10–13%（偶然） |
| 強化前の同じ攻撃（初期版） | 91 / 100 | 97.5% |

## 📁 リポジトリ構成

```
generator/         Rust：capgen（GIF 生成器）+ solve.rs（参照攻撃）
Pages/, wwwroot/   ASP.NET Core Razor Pages アプリ（ダーク UI、6 言語）
samples/           ラベル付き GIF 6 個 + スプライトシート（labels.csv は採点用）
docs/              スクリーンショット
eval.ps1           N 個の GIF を生成し参照ソルバーを評価
metrics.ps1        精度 + 「N 回の試行での成功率」表
CONTEST.md         公開チャレンジのルール
NOTICE.md          著作者、日付、ソースの SHA-256 フィンガープリント
```

## ⚖️ ライセンスと権利

© 2026 **jevkray**. All rights reserved. 出典を明記すれば研究・ローカル実行・テスト・公開レビューは可能です。商用利用には書面での許可が必要です。[`LICENSE.txt`](LICENSE.txt)、[`NOTICE.md`](NOTICE.md) を参照。

## 🔗 リンク

- GitHub: [github.com/Jevkray](https://github.com/Jevkray)
- チャレンジバンドル: [github.com/Jevkray/jevkray-captcha-v3](https://github.com/Jevkray/jevkray-captcha-v3)