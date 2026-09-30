# 🕵️ Attack history: jevkray-captcha-v3

> 🌐 **Русская версия:** [ATTACK-HISTORY.md](ATTACK-HISTORY.md)

> **Full breakdown:** how the captcha is built, how it was attacked (in-house R&D and the community),
> what came out of it, and why no known solver passes the 50% threshold on the current build.

**Date:** September 30, 2026 · **Captcha author:** [jevkray](https://github.com/Jevkray) · **Status:** 🟢 holding

---

## 📌 In short

| # | Who | Approach | Result |
|---|---|---|---|
| 1 | `solve.rs` — author's reference | CV: blobs + per-pixel matching | **0 / 50** |
| 2 | `solve2.rs` — in-house, v2.0 | temporal mean + matched filter + EM | **66–83%** on v8 · **0%** on the current v10 |
| 3 | `solve3.rs` — flow variant | inverted statistics for the flow mode | **0%** on the current v10 |
| 4 | `solve2.rs` v2.1 (WIP) | + scale search (the ±25% pulsation) | work in progress, currently **0%** |
| 5 | Community: Z.ai "Super Z" (v8) | 4 CV iterations + a `labels.csv` leak | honestly **4/100** (4%) |
| 6 | Community: Dmitrii (v8) | 14–20 solvers, including a CNN | **4/100** (4%) |

> **Protection versions:** v8 — classic mechanics (what the community and early measurements were made
> against); **v10 is the current one** (jelly, lens, tears, ±25% size pulsation, random start, frame streaming).

---

## 🧬 Part 1. How the captcha is built

**Field:** 160×160 px · **~9200 dots** · 6-color palette + dark background `#0e0e18`
**Grid:** 2 px step · **Glyph cell:** 5 px · **Code block:** 23×7 cells (115×35 px) · **20 fps, 30 seconds**

### The "zero knowledge per frame" principle

```
┌───────────────────────── one frame ─────────────────────────┐
│  ● ●●  ●   ●●●  ●● ●    ← background: dots indistinguishable│
│ ●  ●● ●●●●  ● ●●  ● ●●      from the code (density, colors, │
│  ●● ●  ● ●●●● ●  ●●●        sizes, lifetimes — all equal)   │
│ ●  ● ●● ●  ● ●●  ●  ●                                      │
└──────────────────────────────────────────────────────────────┘
```

| Mechanic | What it does | Why |
|---|---|---|
| 🎬 Assembly | 1 s the code is invisible → 1.5 s the dots fly into the block | a rigid body is the signal for the eye |
| 🫁 "Breathing" + rotation | translation ±2.5 px, rotation ±12° | breaks per-pixel tracking |
| 🧲 Dot capture | a drifting dot entering a cell is "captured" | removes birth/death artifacts |
| 🔒 Verification | salted SHA-256 on the server, the code lives 30 s | intercepting an answer is useless |

---

## ⚔️ Part 2. In-house solver R&D (full chronology)

### 🧪 Experiment 1 — `analyze.rs` (blob analysis)

**Idea:** find individual dots and match them between frames by color and position.

**Outcome:**
- field coverage ~0.52, ~850 blobs/frame, of which ~450 "clean" dots;
- best shift `d=(0,1)` with 705‰ match — but it is a smeared peak, not the block.

**Verdict:** ❌ dots overlap (up to 4 per pixel); per-pixel tracking is impossible.

### 🧪 Experiment 2 — `flow.rs` (optical flow)

**Idea:** build a velocity field and find the region with coherent motion.

**Outcome:** in the ASCII flow visualization the background's common drift dominates; the block stands out in no way.

**Verdict:** ❌ background and text directions are too close; a "rigid" model is needed, not flow.

### 🧪 Experiment 3 — `vtest.rs` (window displacement estimation)

**Idea:** estimate the dominant displacement between frames for 40×40 windows.

**Outcome:** estimate `est=(1,−1)` vs. true `(0.2,−0.4)`; error 0.78–1.82 px.

**Verdict:** ⚠️ motion direction is measurable, but not accurate enough for alignment.

### 🧪 Experiment 4 — `tsearch.rs` (transform search) — 💡 breakthrough

**Idea:** subtract the **temporal mean** (a background estimate) from the frames and find the 115×35
rectangle that best explains the residual — by scanning position and angle.

**Outcome:** localization error 2.8–11.3 px (mean ~4.7 px) — the block is found reliably.

**Verdict:** ✅ this became the foundation of the final solver.

### 🧪 Experiment 5 — `track.rs` (predictive tracker)

**Idea:** follow the block frame to frame with velocity smoothing and a local ±3 px search.

**Outcome:** the block-vs-background density signal is only **0.035**; there is no cell-level contrast.

**Verdict:** ❌ continuous tracking is unnecessary: independent per-frame localization + EM is more robust.

### 🏆 Final — `solve2.rs`

```
frames ──► temporal mean (background) ──► subtraction ──► matched filter (position + angle)
   │                                                      │
   └────────► 23×7 cell accumulation ◄── EM×4: template → re-align → weights → 30% cut
                        │
                 decode with the 5×7 font
```

| Stage | Essence |
|---|---|
| 1. Background | per-pixel time-average of coverage after assembly (t ≥ 2.2 s) |
| 2. Localization | rectangular matched filter, coarse (3 px / 2°) → fine (1 px / 0.5°) |
| 3. Smoothing | median over 7 frames + interpolation |
| 4. Accumulation | project every frame into the block's canonical coordinates |
| 5. EM ×4 | soft template → re-align → correlation weighting, worst 30% cut |
| 6. Decode | "lit−unlit" contrast for each of the 10 font digits; confidence = margin to runner-up |

**Results (original captcha build):**

| Set | Exact | Per digit |
|---|---|---|
| 6 samples (old set) | 5/6 | 87.5% |
| 30 fresh (seed 2024) | 20/30 (66.7%) | 81.7% |
| 30 fresh (seed 2025) | 25/30 (83.3%) | 93.3% |
| **Total** | **50/66 ≈ 75.8%** | **≈ 87.5%** |

**⚡ Optimization:** the hot loops (localization, EM, accumulation) were parallelized with
`std::thread::scope` and no external dependencies: **37 s → 8.5 s per GIF (×4.4)**, 12 threads.

### 🌊 `solve3.rs` — the flow-mode variant

During the flow-mode experiment (a moving shape with no static structure) `solve3` was created:
the contrast is inverted — in flow, **lit cells are poorer than the background**:

```rust
// flow:  score = us/un − ls/ln      (lit cells are "leaner")
// classic/jelly: score = ls/ln − us/un
```

**Why it did not work on the current captcha:** 🔍
1. localization and decoding looked for the "inverted" contrast (valid for flow only);
2. starting from frame 0 included the assembly phase (when no block exists yet) and polluted the background estimate;
3. it did not account for the ±25% size pulsation.

**Fixed:** a `flow` argument, start at 2.2 s, contrast sign per mode.

### 🔬 `solve2.rs` v2.1 (WIP) — scale-pulsation adaptation

A scale parameter `s` was added to the cell projection. **Current state:** a regression
(0/4 classic, 0/10 jelly) — the scale search is not calibrated yet. Kept in the repository as is,
for history.

---

## 🌍 Part 3. Community attacks (v8 protection)

> Both reports were made against the public demo on **v8 protection** (classic mechanics, before the
> scale pulsation). Their approaches were not tested on the current **v10**, but all in-house
> measurements show 0%.

### 🤖 The Z.ai "Super Z" report (30.09.2026)

Four CV solver iterations:

| # | Approach | Result |
|---|---|---|
| 1 | FFT frame cross-correlation | peak (+1,−1), SNR 1.72 — but accumulation gives noise |
| 2 | Motion-compensated accumulation | ~100% false positives (46% density) |
| 3 | Multi-frame consistency (K=2…6) | survivors 5779 → 2850 → 1377 → 716 → 401 px, largest component **14 px** |
| 4 | Temporal correlation map | excess: mean +8.75, max +50.39, σ=13.29 — uniform noise |

> "The technology really implements the 'zero knowledge per frame' principle in the strict sense"
> — Z.ai report

The formal demo "success" was obtained through a **metadata leak**: `samples/labels.csv`
in the public repository → identifying the current GIF on the page → entering the answer.
This path is **forbidden by the challenge rules** and does not work against production
(the .NET app generates codes on the fly and stores only a salted hash).

Their honest CV metrics: **4/100 exact (4%)**, 36/100 with 3+ digits, 52% average per digit.

### 👤 Dmitrii's report

14–20 solver iterations: FFT, motion compensation, KDTree+RANSAC, motion voting, template matching,
a CNN (PyTorch) — trained on 1000 generated GIFs.

| Metric | Value |
|---|---|
| Exact matches | **4/100 (4%)** |
| 3+ correct digits | 36/100 (36%) |
| Per digit | 37% / 72% / 78% / 20% (52% average) |

> "This captcha really works as advertised. The 'zero knowledge per frame' principle is real —
> a single frame is indistinguishable from the background, and multi-frame analysis requires
> non-trivial ML infrastructure" — Dmitrii

---

## 🛡️ Part 4. How the captcha improved (and what it did to the solvers)

| Version | What was added | Effect on the solvers |
|---|---|---|
| **v8** (classic) | base mechanics (rigid body), 50% threshold | `solve.rs` 0/50 · `solve2` **66–83%** · community **4%** |
| v9 (intermediate) | living jelly, liquid lens, drifting tears; a flow experiment | `solve2` **~12%** on jelly · flow 0% (flow removed) |
| **v10 (current)** | **±25% size pulsation** + random block start + faster pulsation + frame streaming | **all solvers 0%** ✅ |
| web | file-less streaming, canvas cover, hover reveal | offline benchmarking becomes impossible, the stream is one-shot |

**Key takeaway:** the deadliest improvement is not the visual distortions but the **size pulsation**:
it breaks the very idea of "one rigid transform per frame" that `solve2` was built on.

---

## 📊 Part 5. All measurements, one table

**Method:** 10 new samples (jelly, 30 s) + 6 fresh classic (30 s); exactness = exact 4-digit matches;
"per digit" = share of correct positions.

| Solver | Captcha version | Exact | Per digit |
|---|---|---|---|
| `solve.rs` (reference) | v8 classic | 0/50 | ~10–13% |
| `solve.rs` (reference) | v10 jelly | 0/10 | 4/40 (10%) |
| `solve2.rs` v2.0 | v8 classic | 50/66 (75.8%) | ~87.5% |
| `solve2.rs` v2.0 | v10 classic | **0/6** | 5/24 (21%) |
| `solve2.rs` v2.0 | v10 jelly | **0/10** | 10/40 (25%) |
| `solve2.rs` v2.1 (WIP) | v10 classic | 0/4 | — |
| `solve2.rs` v2.1 (WIP) | v10 jelly | 0/10 | 10/40 (25%) |
| `solve3.rs` (fixed) | v10 jelly | 0/10 | 10/40 (25%) |
| Community Z.ai (CV) | v8 classic | 4/100 (4%) | ~52% |
| Community Dmitrii | v8 classic | 4/100 (4%) | ~52% |

> ⚠️ Small samples (10–66 GIFs) show noticeable variance: `solve2`'s classic batches ranged
> 66.7% vs 83.3%.

---

## 🧠 Part 6. Conclusions

1. **"Zero knowledge per frame" works.** No per-pixel, per-frame-pair or correlation method
   extracted the digits: the block's density signal is only ~0.035.
2. **The only breakthrough comes from a rigid model + accumulation.** The single working approach is
   temporal mean + matched filter + EM (our `solve2`).
3. **The size pulsation breaks even that.** After ±25%, none of the four solvers passes the threshold.
4. **The community confirms it:** the best honest CV result is 4% (threshold 50%).
   The only demo "successes" were `labels.csv` leaks, forbidden by the rules.
5. **What's next:** a robust solver needs long-horizon tracking + RANSAC rigid-body segmentation
   + OCR trained on generated data (estimated at 5–10 days of work).

---

## 🔧 Appendix: reproduction

```powershell
# build
cd generator; cargo build --release

# generation: <code> <file> [fps] [seconds] [classic|jelly]
.\target\release\capgen.exe 7945 sample.gif 20 30 jelly

# solvers
.\target\release\solve.exe  sample.gif            # reference (0%)
.\target\release\solve2.exe sample.gif            # strong (breaks on the current build)
.\target\release\solve2.exe sample.gif debug      # + accumulated template
.\target\release\solve3.exe sample.gif            # flow variant (fixed)
.\target\release\solve3.exe sample.gif flow       # flow mode
```

```powershell
# benchmark on your own set
.\generator\eval.ps1 -N 100
.\metrics.ps1 -Csv answers.csv
```

---

*Compiled from: the solver development log (DeepSeek V4.1 session), the Z.ai "Super Z" community
report (11 pages), Dmitrii's report, and the captcha author's measurements.*
