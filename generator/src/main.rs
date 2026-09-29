use std::env;
use std::f64::consts::PI;
use std::fs::File;
use std::io::BufWriter;

// ---- параметры генератора (те же, что были в JS) ----
const W: i32 = 192;
const H: i32 = 192;
const AREA: i32 = 160;
const CROP: i32 = (W - AREA) / 2;
// число точек = (больший размер поля)^2 / 4
const FIELD: i32 = if W >= H { W } else { H };
const TARGET_DOTS: i32 = FIELD * FIELD / 4;

// dark background + palette of 6 matching colours
const BG_RGB: (u8, u8, u8) = (14, 14, 18);
const FG_S: f64 = 0.65;
const FG_L: f64 = 0.68;
const CELL: i32 = 5;
const FPS: f64 = 20.0;
const SECONDS: f64 = 30.0;
const REVEAL_DELAY: f64 = 1.0;
const REVEAL_TIME: f64 = 1.5;
const LIFE: f64 = 2.0; // equal lifetime for all dots: no birth-rate leak
const BG_MIN: f64 = 0.8;
const BG_MAX: f64 = 1.4;
const GL_MIN: f64 = 0.25;
const GL_MAX: f64 = 1.0;
const TX_MIN: f64 = 1.5;
const TX_MAX: f64 = 2.2;
const COLOR_SPEED: f64 = 1.0;
const BG_SWING: f64 = 0.5;
const MIN_ANGLE: f64 = 10.0;
const SPAWN_GRAD: f64 = 0.75;

const TAU: f64 = 2.0 * PI;
// пересчёт скоростей/жизни с 20 fps (как в JS) на текущий fps
const TS: f64 = 20.0 / FPS;

const FONT: [[&str; 7]; 10] = [
    ["01110", "10001", "10011", "10101", "11001", "10001", "01110"],
    ["00100", "01100", "00100", "00100", "00100", "00100", "01110"],
    ["01110", "10001", "00001", "00010", "00100", "01000", "11111"],
    ["11111", "00010", "00100", "00010", "00001", "10001", "01110"],
    ["00010", "00110", "01010", "10010", "11111", "00010", "00010"],
    ["11111", "10000", "11110", "00001", "00001", "10001", "01110"],
    ["00110", "01000", "10000", "11110", "10001", "10001", "01110"],
    ["11111", "00001", "00010", "00100", "01000", "01000", "01000"],
    ["01110", "10001", "10001", "01110", "10001", "10001", "01110"],
    ["01110", "10001", "10001", "01111", "00001", "00010", "01100"],
];

struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self { Rng(seed | 1) }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn f(&mut self) -> f64 { (self.next() >> 11) as f64 / (1u64 << 53) as f64 }
    fn range(&mut self, a: f64, b: f64) -> f64 { a + self.f() * (b - a) }
}

struct Osc { phase: f64, omega: f64 }
impl Osc {
    fn new(rng: &mut Rng) -> Self { Osc { phase: rng.range(0.0, TAU), omega: rng.range(0.006, 0.02) * TS } }
    fn val(&mut self, lo: f64, hi: f64) -> f64 {
        self.phase += self.omega;
        let mid = (lo + hi) / 2.0;
        let half = (hi - lo) / 2.0;
        mid + half * self.phase.sin()
    }
}

struct Particle {
    g: bool,
    x: f64, y: f64,
    gc: i32, gr: i32,
    fx: f64, fy: f64,
    ox: f64, oy: f64,
    dx: f64, dy: f64,
    sw: i32, sh: i32,
    t: f64,
    dt: f64,
    hue: f64,
    hspd: f64,
    vs: f64,   // ±35% разброс скорости: фон не единый поток
    va: f64,   // ±0.25 рад разброс направления
}


fn ang_diff(a: f64, b: f64) -> f64 {
    let mut d = (a - b) % TAU;
    if d > PI { d -= TAU; }
    if d < -PI { d += TAU; }
    d
}

fn clamp(v: f64, lo: f64, hi: f64) -> f64 { if v < lo { lo } else if v > hi { hi } else { v } }

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let hp = h / 60.0;
    let x = c * (1.0 - ((hp % 2.0) - 1.0).abs());
    let (r, g, b) = if hp < 1.0 { (c, x, 0.0) }
        else if hp < 2.0 { (x, c, 0.0) }
        else if hp < 3.0 { (0.0, c, x) }
        else if hp < 4.0 { (0.0, x, c) }
        else if hp < 5.0 { (x, 0.0, c) }
        else { (c, 0.0, x) };
    let m = l - c / 2.0;
    ((((r + m) * 255.0).round()) as u8, (((g + m) * 255.0).round()) as u8, (((b + m) * 255.0).round()) as u8)
}

fn build_palette(hues: &[f64; 6]) -> Vec<u8> {
    let mut p = vec![BG_RGB.0, BG_RGB.1, BG_RGB.2];
    for i in 0..6 {
        let (r, g, b) = hsl_to_rgb(hues[i], FG_S, FG_L);
        p.push(r); p.push(g); p.push(b);
    }
    p.extend_from_slice(&[BG_RGB.0, BG_RGB.1, BG_RGB.2]);
    p
}

const TRANSPARENT: u8 = 7;

fn hue_idx(hue: f64) -> u8 {
    let h = hue.rem_euclid(360.0);
    (1 + ((h / 360.0 * 6.0).floor() as i32 % 6)) as u8
}

fn square2(buf: &mut [u8], cx: f64, cy: f64, w: i32, h: i32, idx: u8) {
    let x0 = cx.round() as i32;
    let y0 = cy.round() as i32;
    for py in y0..y0 + h {
        if py < CROP || py >= CROP + AREA { continue; }
        for px in x0..x0 + w {
            if px < CROP || px >= CROP + AREA { continue; }
            buf[((py - CROP) * AREA + (px - CROP)) as usize] = idx;
        }
    }
}

fn build_mask(code: &str) -> (Vec<Vec<bool>>, i32) {
    let grid_w = 5usize;
    let grid_h = 7usize;
    let cols = code.len() * grid_w + (code.len() - 1);
    let mut rows = vec![vec![false; cols]; grid_h];
    for (i, ch) in code.chars().enumerate() {
        let d = (ch as u8 - b'0') as usize;
        let base = i * (grid_w + 1);
        for r in 0..grid_h {
            for c in 0..grid_w {
                if FONT[d][r].as_bytes()[c] == b'1' {
                    rows[r][base + c] = true;
                }
            }
        }
    }
    (rows, cols as i32)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: capgen <code> <out.gif>");
        std::process::exit(2);
    }
    let code = &args[1];
    let out = &args[2];
    let fps: f64 = args.get(3).and_then(|s| s.parse().ok()).filter(|v: &f64| *v > 0.0).unwrap_or(FPS);
    let seconds: f64 = args.get(4).and_then(|s| s.parse().ok()).filter(|v: &f64| *v > 0.0).unwrap_or(SECONDS);

    let (mask, cols) = build_mask(code);
    let bw = cols * CELL;
    let bh = 7 * CELL;

    // единая равномерная сетка точек (дробный шаг — чтобы набрать нужное число)
    let step = ((W * H) as f64 / TARGET_DOTS as f64).sqrt();
    let mut grid: Vec<(f64, f64)> = Vec::new();
    let mut gy = step / 2.0;
    while gy < H as f64 {
        let mut gx = step / 2.0;
        while gx < W as f64 {
            grid.push((gx, gy));
            gx += step;
        }
        gy += step;
    }

    let seed = (code.bytes().fold(1469598103934665603u64, |a, b| (a ^ b as u64).wrapping_mul(1099511628211)))
        ^ (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64);
    let mut rng = Rng::new(seed);

    let lit: Vec<(i32, i32)> = {
        let mut v = Vec::new();
        for (r, row) in mask.iter().enumerate() {
            for (c, &on) in row.iter().enumerate() {
                if on { v.push((c as i32, r as i32)); }
            }
        }
        v
    };

    let start_bx = clamp((W - bw) as f64 / 2.0, 0.0, W as f64);
    let start_by = clamp((H - bh) as f64 / 2.0, 0.0, H as f64);

    let make_particle = |rng: &mut Rng, x: f64, y: f64| -> Particle {
        let lx = x - start_bx;
        let ly = y - start_by;
        let g = lx >= 0.0 && ly >= 0.0 && lx < bw as f64 && ly < bh as f64
            && mask[(ly / CELL as f64) as usize][(lx / CELL as f64) as usize];
        Particle {
            g, x, y,
            gc: if g { (lx / CELL as f64) as i32 } else { 0 },
            gr: if g { (ly / CELL as f64) as i32 } else { 0 },
            fx: if g { (lx - CELL as f64 * (lx / CELL as f64).floor()) / CELL as f64 } else { 0.0 },
            fy: if g { (ly - CELL as f64 * (ly / CELL as f64).floor()) / CELL as f64 } else { 0.0 },
            ox: if g { rng.range(0.0, W as f64) } else { 0.0 },
            oy: if g { rng.range(0.0, H as f64) } else { 0.0 },
            dx: 0.0, dy: 0.0,
            sw: 1 + (rng.f() * 2.0) as i32, sh: 1 + (rng.f() * 2.0) as i32,
            t: rng.f(),
            dt: rng.range(0.006, 0.02) * TS,
            hue: rng.range(0.0, 360.0),
            hspd: rng.range(0.5, 2.0),
            vs: rng.range(0.65, 1.35),
            va: rng.range(-0.25, 0.25),
        }
    };

    let mut parts: Vec<Particle> = grid.iter().map(|&(x, y)| make_particle(&mut rng, x, y)).collect();
    // перемешиваем
    for i in (1..parts.len()).rev() {
        let j = (rng.f() * (i + 1) as f64) as usize;
        parts.swap(i, j);
    }

    let target_att = parts.iter().filter(|p| p.g).count() as i32 * 3 / 2;
    let mut att_count = target_att;
    let ang_glyph = rng.range(0.0, TAU);
    let mut ang_text = rng.range(0.0, TAU);
    let mut bg_base = rng.range(0.0, TAU);
    let mut bg_phase = rng.range(0.0, TAU);
    let bg_omega = rng.range(0.01, 0.03) * TS;
    let mut bx = start_bx;
    let mut by = start_by;

    let mut osc_bg = Osc::new(&mut rng);
    let mut osc_gl = Osc::new(&mut rng);
    let mut osc_tx = Osc::new(&mut rng);

let schemes: [[f64; 6]; 4] = [
        [0.0, 25.0, 50.0, 75.0, 100.0, 125.0],
        [0.0, 15.0, 30.0, 180.0, 195.0, 210.0],
        [0.0, 20.0, 40.0, 120.0, 140.0, 160.0],
        [0.0, 30.0, 60.0, 90.0, 120.0, 150.0],
    ];
    let scheme = &schemes[(rng.f() * schemes.len() as f64) as usize % schemes.len()];
    let base = rng.range(0.0, 360.0);
    let mut hues = [0.0f64; 6];
    for i in 0..6 { hues[i] = (scheme[i] + base) % 360.0; }
    let palette = build_palette(&hues);
    let frames = (seconds * fps) as usize;
    let delay = (100.0 / fps).round().max(1.0) as u16;

    let file = File::create(out).expect("create gif");
    let mut enc = gif::Encoder::new(BufWriter::new(file), AREA as u16, AREA as u16, &palette).expect("encoder");
    enc.set_repeat(gif::Repeat::Infinite).expect("repeat");

    let px = (AREA * AREA) as usize;
    let mut buf = vec![0u8; px];
    let mut prev = vec![0u8; px];
    let mut out_buf = vec![0u8; px];

    for frame in 0..frames {
        // фон качается и держит отрыв от текста
        bg_phase += bg_omega;
        let mut ang_bg = bg_base + BG_SWING * bg_phase.sin();
        let min_sep = MIN_ANGLE * PI / 180.0;
        let ds = ang_diff(ang_bg, ang_text);
        if ds.abs() < min_sep {
            bg_base += (min_sep - ds.abs() + 0.05) * if ds >= 0.0 { 1.0 } else { -1.0 };
            ang_bg = bg_base + BG_SWING * bg_phase.sin();
        }
        let bg_cos = ang_bg.cos();
        let bg_sin = ang_bg.sin();
        let pp = |x: f64, y: f64| x * bg_cos + y * bg_sin;
        let p_min = pp(0.0, 0.0).min(pp(W as f64, 0.0)).min(pp(0.0, H as f64)).min(pp(W as f64, H as f64));
        let p_max = pp(0.0, 0.0).max(pp(W as f64, 0.0)).max(pp(0.0, H as f64)).max(pp(W as f64, H as f64));
        let proj_t = |x: f64, y: f64| clamp((x * bg_cos + y * bg_sin - p_min) / (p_max - p_min), 0.0, 1.0);

        let v_bg = osc_bg.val(BG_MIN, BG_MAX) * TS;
        let v_gl = osc_gl.val(GL_MIN, GL_MAX) * TS;
        let v_tx = osc_tx.val(TX_MIN, TX_MAX) * TS;

        // текст ходит и отскакивает внутри видимой зоны 128x128
        bx += ang_text.cos() * v_tx;
        by += ang_text.sin() * v_tx;
        if bx < CROP as f64 { bx = CROP as f64; ang_text = PI - ang_text; }
        else if bx > (CROP + AREA - bw) as f64 { bx = (CROP + AREA - bw) as f64; ang_text = PI - ang_text; }
        if by < CROP as f64 { by = CROP as f64; ang_text = -ang_text; }
        else if by > (CROP + AREA - bh) as f64 { by = (CROP + AREA - bh) as f64; ang_text = -ang_text; }

        // общее «дыхание» текста: единый сдвиг для всех точек штрихов
        let breath = (osc_gl.phase * 0.5).sin();
        let gdx = ang_glyph.cos() * breath * 2.5;
        let gdy = ang_glyph.sin() * breath * 2.5;
        osc_gl.phase += osc_gl.omega * 0.5;
        // медленное вращение надписи (±8°): жёсткое тело, сильный признак для глаза
        let rot = (osc_gl.phase * 0.37).sin() * 0.21;
        let rc = rot.cos();
        let rs = rot.sin();
        let rcx = bx + bw as f64 / 2.0;
        let rcy = by + bh as f64 / 2.0;

        let t_sec = frame as f64 / fps;
        let reveal = clamp((t_sec - REVEAL_DELAY) / REVEAL_TIME, 0.0, 1.0);

        buf.iter_mut().for_each(|v| *v = 0);

        for p in parts.iter_mut() {
            p.t += p.dt * LIFE;
            if p.t >= 1.0 {
                // all dots respawn free at a uniform random place: identical statistics everywhere
                let mut placed = false;
                for _ in 0..32 {
                    let x = rng.range(0.0, W as f64);
                    let y = rng.range(0.0, H as f64);
                    let w = 1.0 + SPAWN_GRAD * (1.0 - proj_t(x, y));
                    if rng.f() < w / (1.0 + SPAWN_GRAD) { p.x = x; p.y = y; placed = true; break; }
                }
                if !placed { p.x = rng.range(0.0, W as f64); p.y = rng.range(0.0, H as f64); }
                if p.g { p.g = false; att_count -= 1; }
                p.hue = rng.range(0.0, 360.0);
                p.hspd = rng.range(0.5, 2.0);
                p.vs = rng.range(0.65, 1.35);
                p.va = rng.range(-0.25, 0.25);
                p.sw = 1 + (rng.f() * 2.0) as i32; p.sh = 1 + (rng.f() * 2.0) as i32;
                p.t = 0.0;
                p.dt = rng.range(0.006, 0.02) * TS;
            }
            p.hue = (p.hue + COLOR_SPEED * p.hspd * TS) % 360.0;

            if p.g {
                if reveal <= 0.0 {
                    p.x += bg_cos * v_bg;
                    p.y += bg_sin * v_bg;
                    if p.x < 0.0 || p.x > W as f64 { p.x = rng.range(0.0, W as f64); }
                    if p.y < 0.0 || p.y > H as f64 { p.y = rng.range(0.0, H as f64); }
                    p.ox = p.x; p.oy = p.y;
                } else {
                    // rigid body: all stroke dots share one common offset (common fate cue)
                    p.dx = gdx;
                    p.dy = gdy;
                    let lx = (p.gc as f64 + p.fx) * CELL as f64 - bw as f64 / 2.0 + p.dx;
                    let ly = (p.gr as f64 + p.fy) * CELL as f64 - bh as f64 / 2.0 + p.dy;
                    let tx = rcx + lx * rc - ly * rs;
                    let ty = rcy + lx * rs + ly * rc;
                    p.x = p.ox + (tx - p.ox) * reveal;
                    p.y = p.oy + (ty - p.oy) * reveal;
                }
            } else {
                let av = ang_bg + p.va;
                p.x += av.cos() * v_bg * p.vs;
                p.y += av.sin() * v_bg * p.vs;
                if p.x < 0.0 || p.x > W as f64 || p.y < 0.0 || p.y > H as f64 {
                    // ушли за край — заново
                    let mut placed = false;
                    for _ in 0..32 {
                        let x = rng.range(0.0, W as f64);
                        let y = rng.range(0.0, H as f64);
                        let w = 1.0 + SPAWN_GRAD * (1.0 - proj_t(x, y));
                        if rng.f() < w / (1.0 + SPAWN_GRAD) { p.x = x; p.y = y; placed = true; break; }
                    }
                    if !placed { p.x = rng.range(0.0, W as f64); p.y = rng.range(0.0, H as f64); }
                    p.hue = rng.range(0.0, 360.0);
                    p.hspd = rng.range(0.5, 2.0);
                p.vs = rng.range(0.65, 1.35);
                p.va = rng.range(-0.25, 0.25);
                    p.sw = 1 + (rng.f() * 2.0) as i32; p.sh = 1 + (rng.f() * 2.0) as i32;
                    p.t = 0.0;
                    p.dt = rng.range(0.006, 0.02) * TS;
                    continue;
                }
                // a free dot drifting into a stroke starts moving with the text (no hiding at all)
                if reveal >= 1.0 && att_count < target_att {
                    let lx = p.x - bx;
                    let ly = p.y - by;
                    if lx >= 0.0 && ly >= 0.0 && lx < bw as f64 && ly < bh as f64
                        && mask[(ly / CELL as f64) as usize][(lx / CELL as f64) as usize] {
                        let gc = (lx / CELL as f64) as i32;
                        let gr = (ly / CELL as f64) as i32;
                        p.g = true;
                        p.gc = gc; p.gr = gr;
                        p.fx = (lx - CELL as f64 * gc as f64) / CELL as f64;
                        p.fy = (ly - CELL as f64 * gr as f64) / CELL as f64;
                        p.ox = p.x; p.oy = p.y;
                        p.dx = 0.0; p.dy = 0.0;
                        att_count += 1;
                    }
                }
            }
            
            square2(&mut buf, p.x, p.y, p.sw, p.sh, hue_idx(p.hue));
        }

        let mut f = if frame == 0 {
            gif::Frame::from_indexed_pixels(AREA as u16, AREA as u16, buf.clone(), None)
        } else {
            for i in 0..px {
                out_buf[i] = if buf[i] == prev[i] { TRANSPARENT } else { buf[i] };
            }
            let mut fr = gif::Frame::from_indexed_pixels(AREA as u16, AREA as u16, out_buf.clone(), None);
            fr.transparent = Some(TRANSPARENT);
            fr.dispose = gif::DisposalMethod::Keep;
            fr
        };
        f.delay = delay;
        enc.write_frame(&f).expect("write frame");
        prev.copy_from_slice(&buf);
    }
}
