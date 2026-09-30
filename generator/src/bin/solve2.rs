use std::env;
use std::f64::consts::PI;
use std::fs::File;

const COLS: i32 = 23;
const ROWS: i32 = 7;
const CELL: i32 = 5;

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

struct Frames {
    w: i32,
    h: i32,
    cov: Vec<Vec<u8>>,
    delay: u16,
}

fn load(path: &str) -> Frames {
    let file = File::open(path).expect("open gif");
    let mut opts = gif::DecodeOptions::new();
    opts.set_color_output(gif::ColorOutput::Indexed);
    let mut dec = opts.read_info(file).expect("gif header");
    let w = dec.width() as i32;
    let h = dec.height() as i32;
    let mut canvas = vec![0u8; (w * h) as usize];
    let mut cov = Vec::new();
    let mut delay = 5u16;
    while let Some(frame) = dec.read_next_frame().expect("frame") {
        let (fw, fh) = (frame.width as i32, frame.height as i32);
        let (left, top) = (frame.left as i32, frame.top as i32);
        if frame.delay > 0 { delay = frame.delay; }
        let tr = frame.transparent;
        for y in 0..fh {
            for x in 0..fw {
                let v = frame.buffer[(y * fw + x) as usize];
                if Some(v) == tr { continue; }
                let (gx, gy) = (left + x, top + y);
                if gx >= 0 && gy >= 0 && gx < w && gy < h { canvas[(gy * w + gx) as usize] = v; }
            }
        }
        cov.push(canvas.iter().map(|&v| if v != 0 { 1u8 } else { 0u8 }).collect());
    }
    Frames { w, h, cov, delay }
}

fn cellvals(f: &[u8], mean: &[f32], w: i32, h: i32, cx: f64, cy: f64, a: f64, bw: i32, bh: i32) -> Vec<f64> {
    let (ca, sa) = (a.cos(), a.sin());
    let mut out = vec![0.0f64; (COLS * ROWS) as usize];
    for r in 0..ROWS {
        for c in 0..COLS {
            let mut s = 0.0;
            for sy in 0..CELL {
                for sx in 0..CELL {
                    let lx = (c * CELL + sx) as f64 - bw as f64 / 2.0;
                    let ly = (r * CELL + sy) as f64 - bh as f64 / 2.0;
                    let wx = cx + lx * ca - ly * sa;
                    let wy = cy + lx * sa + ly * ca;
                    let xi = wx.round() as i32;
                    let yi = wy.round() as i32;
                    if xi >= 0 && yi >= 0 && xi < w && yi < h {
                        s += f[(yi * w + xi) as usize] as f64 - mean[(yi * w + xi) as usize] as f64;
                    }
                }
            }
            out[(r * COLS + c) as usize] = s;
        }
    }
    out
}


fn rectscore(f: &[u8], mean: &[f32], w: i32, h: i32, cx: f64, cy: f64, ang: f64, bw: i32, bh: i32, step: f64) -> f64 {
    let (ca, sa) = (ang.cos(), ang.sin());
    let mut s = 0.0f64;
    let mut n = 0.0f64;
    let mut ly = -(bh as f64) / 2.0;
    while ly < bh as f64 / 2.0 {
        let mut lx = -(bw as f64) / 2.0;
        while lx < bw as f64 / 2.0 {
            let wx = cx + lx * ca - ly * sa;
            let wy = cy + lx * sa + ly * ca;
            let xi = wx.round() as i32;
            let yi = wy.round() as i32;
            if xi >= 0 && yi >= 0 && xi < w && yi < h {
                s += f[(yi * w + xi) as usize] as f64 - mean[(yi * w + xi) as usize] as f64;
            }
            n += 1.0;
            lx += step;
        }
        ly += step;
    }
    s / n.max(1.0)
}

fn parallel_chunks<T: Sync, R: Send>(items: &[T], f: impl Fn(&[T]) -> Vec<R> + Sync) -> Vec<R> {
    let nthreads = std::thread::available_parallelism().map(|v| v.get()).unwrap_or(1);
    if nthreads <= 1 || items.is_empty() {
        return f(items);
    }
    let chunk = items.len().div_ceil(nthreads);
    let f = &f;
    let mut out = Vec::with_capacity(items.len());
    std::thread::scope(|s| {
        let handles: Vec<_> = items.chunks(chunk).map(|c| s.spawn(move || f(c))).collect();
        for h in handles {
            out.extend(h.join().unwrap());
        }
    });
    out
}

fn locate(f: &[u8], mean: &[f32], w: i32, h: i32, bw: i32, bh: i32) -> (f64, f64, f64) {
    let mut best = (f64::MIN, 0.0, 0.0, 0.0);
    let mut adeg = -12.0;
    while adeg <= 12.0 {
        let a = adeg * PI / 180.0;
        let mut cyi = (bh / 2 + 4) as f64;
        while cyi <= (h - bh / 2 - 4) as f64 {
            let mut cxi = (bw / 2 + 4) as f64;
            while cxi <= (w - bw / 2 - 4) as f64 {
                let s = rectscore(f, mean, w, h, cxi, cyi, a, bw, bh, 3.0);
                if s > best.0 { best = (s, cxi, cyi, a); }
                cxi += 3.0;
            }
            cyi += 3.0;
        }
        adeg += 2.0;
    }
    let mut fb = best;
    for dcy in -4..=4 {
        for dcx in -4..=4 {
            let mut da = -3.0;
            while da <= 3.0 {
                let cx = best.1 + dcx as f64;
                let cy = best.2 + dcy as f64;
                let a = best.3 + da * PI / 180.0;
                if cx < (bw / 2 + 2) as f64 || cy < (bh / 2 + 2) as f64
                    || cx > (w - bw / 2 - 2) as f64 || cy > (h - bh / 2 - 2) as f64
                { da += 0.5; continue; }
                let s = rectscore(f, mean, w, h, cx, cy, a, bw, bh, 1.0);
                if s > fb.0 { fb = (s, cx, cy, a); }
                da += 0.5;
            }
        }
    }
    (fb.1, fb.2, fb.3)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { eprintln!("usage: solve2 <file.gif> [debug]"); std::process::exit(2); }
    let dbg = args.get(2).map(|s| s.as_str()) == Some("debug");
    let fr = load(&args[1]);
    let (w, h) = (fr.w, fr.h);
    let bw = COLS * CELL;
    let bh = ROWS * CELL;
    let n = fr.cov.len();
    let fps = (100.0 / fr.delay as f64).max(1.0);
    let start = ((2.2 * fps) as usize).min(n.saturating_sub(1));

    // временное среднее (оценка фона)
    let mut mean = vec![0.0f32; (w * h) as usize];
    let mut cnt = 0.0f32;
    for t in start..n {
        let f = &fr.cov[t];
        for i in 0..(w * h) as usize { mean[i] += f[i] as f32; }
        cnt += 1.0;
    }
    if cnt < 1.0 { eprintln!("мало кадров"); std::process::exit(1); }
    for m in mean.iter_mut() { *m /= cnt; }

    // оценка траектории: грубая локализация по кадрам
    let mut est: Vec<(f64, f64, f64)> = Vec::with_capacity(n);
    for _ in 0..n { est.push((0.0, 0.0, 0.0)); }
    let step_t = 2usize;
    let mut ts: Vec<usize> = Vec::new();
    let mut t = start;
    while t < n { ts.push(t); t += step_t; }
    for (t, v) in parallel_chunks(&ts, |chunk| {
        chunk
            .iter()
            .map(|&t| (t, locate(&fr.cov[t], &mean, w, h, bw, bh)))
            .collect::<Vec<_>>()
    }) {
        est[t] = v;
    }
    // заполнение промежуточных кадров и медианное сглаживание
    for i in 1..ts.len() {
        let (t0, t1) = (ts[i - 1], ts[i]);
        for k in t0 + 1..t1 {
            let f = (k - t0) as f64 / (t1 - t0) as f64;
            est[k] = (
                est[t0].0 + (est[t1].0 - est[t0].0) * f,
                est[t0].1 + (est[t1].1 - est[t0].1) * f,
                est[t0].2 + (est[t1].2 - est[t0].2) * f,
            );
        }
    }
    // медианная фильтрация положения (окно 7)
    let mut sm = est.clone();
    let hw = 3i64;
    for t in start..n {
        for axis in 0..2 {
            let mut v: Vec<f64> = Vec::new();
            for k in -(hw as i64)..=hw as i64 {
                let tt = t as i64 + k;
                if tt >= start as i64 && (tt as usize) < n {
                    v.push(if axis == 0 { est[tt as usize].0 } else { est[tt as usize].1 });
                }
            }
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let m = v[v.len() / 2];
            if axis == 0 { sm[t].0 = m; } else { sm[t].1 = m; }
        }
    }
    let mut est = sm;

    let tall: Vec<usize> = (start..n).collect();

    // накопление клеток в канонических координатах
    let partials = parallel_chunks(&tall, |chunk| {
        let mut local = vec![0.0f64; (COLS * ROWS) as usize];
        for &t in chunk {
            let (cx, cy, a) = est[t];
            let (ca, sa) = (a.cos(), a.sin());
            for r in 0..ROWS {
                for c in 0..COLS {
                    let mut s = 0.0;
                    for sy in 0..CELL {
                        for sx in 0..CELL {
                            let lx = (c * CELL + sx) as f64 - bw as f64 / 2.0;
                            let ly = (r * CELL + sy) as f64 - bh as f64 / 2.0;
                            let wx = cx + lx * ca - ly * sa;
                            let wy = cy + lx * sa + ly * ca;
                            let xi = wx.round() as i32;
                            let yi = wy.round() as i32;
                            if xi >= 0 && yi >= 0 && xi < w && yi < h {
                                s += fr.cov[t][(yi * w + xi) as usize] as f64
                                    - mean[(yi * w + xi) as usize] as f64;
                            }
                        }
                    }
                    local[(r * COLS + c) as usize] += s;
                }
            }
        }
        vec![local]
    });
    let mut acc = vec![0.0f64; (COLS * ROWS) as usize];
    for p in &partials {
        for k in 0..acc.len() { acc[k] += p[k]; }
    }
    let accn = (n - start) as f64;
    // среднее квадрата: контраст лит/нелит
    let mut cells = vec![0.0f64; (COLS * ROWS) as usize];
    for i in 0..cells.len() {
        cells[i] = acc[i] / (accn * (CELL * CELL) as f64);
    }

    let decode = |cells: &[f64]| -> (String, Vec<f64>) {
        let mut code = String::new();
        let mut confs = Vec::new();
        for g in 0..4 {
            let base = (g * 6) as usize;
            let mut bestd = 0usize;
            let mut bests = f64::MIN;
            let mut second = f64::MIN;
            for dgt in 0..10 {
                let mut ls = 0.0; let mut ln = 0.0;
                let mut us = 0.0; let mut un = 0.0;
                for r in 0..ROWS as usize {
                    for c in 0..5usize {
                        let v = cells[r * COLS as usize + base + c];
                        if FONT[dgt][r].as_bytes()[c] == b'1' { ls += v; ln += 1.0; }
                        else { us += v; un += 1.0; }
                    }
                }
                let sc = ls / ln - us / un;
                if sc > bests { second = bests; bests = sc; bestd = dgt; }
                else if sc > second { second = sc; }
            }
            confs.push(bests - second);
            code.push((b'0' + bestd as u8) as char);
        }
        (code, confs)
    };

    let (mut code, mut confs) = decode(&cells);

    // EM: уточняем трансформ каждой рамки под текущий шаблон цифр и перенакопляем
    for _iter in 0..4 {
        // мягкий шаблон = текущее накопленное отклонение от фона (со снятым средним)
        let tm = cells.iter().sum::<f64>() / cells.len() as f64;
        let templ: Vec<f64> = cells.iter().map(|v| v - tm).collect();
        for (t, v) in parallel_chunks(&tall, |chunk| {
            chunk
                .iter()
                .map(|&t| {
                    let (bx, by, ba) = est[t];
                    let mut best = (f64::MIN, bx, by, ba);
                    for dcy in -3..=3 {
                        for dcx in -3..=3 {
                            let mut da = -3.0;
                            while da <= 3.0 {
                                let cx = bx + dcx as f64;
                                let cy = by + dcy as f64;
                                let a = ba + da * PI / 180.0;
                                da += 0.5;
                                if cx < (bw / 2 + 2) as f64 || cy < (bh / 2 + 2) as f64
                                    || cx > (w - bw / 2 - 2) as f64 || cy > (h - bh / 2 - 2) as f64
                                { continue; }
                                let cv = cellvals(&fr.cov[t], &mean, w, h, cx, cy, a, bw, bh);
                                let mut s = 0.0;
                                for i in 0..cv.len() { s += cv[i] * templ[i]; }
                                if s > best.0 { best = (s, cx, cy, a); }
                            }
                        }
                    }
                    (t, (best.1, best.2, best.3))
                })
                .collect::<Vec<_>>()
        }) {
            est[t] = v;
        }
        // уверенность рамки + мягкая отсечка выбросов
        let mut sc: Vec<f64> = vec![0.0; n - start];
        let mut cvs: Vec<Vec<f64>> = vec![Vec::new(); n - start];
        for (t, s, cv) in parallel_chunks(&tall, |chunk| {
            chunk
                .iter()
                .map(|&t| {
                    let (cx, cy, a) = est[t];
                    let cv = cellvals(&fr.cov[t], &mean, w, h, cx, cy, a, bw, bh);
                    let mut s = 0.0;
                    for i in 0..cv.len() { s += cv[i] * templ[i]; }
                    (t, s, cv)
                })
                .collect::<Vec<_>>()
        }) {
            sc[t - start] = s;
            cvs[t - start] = cv;
        }
        let mut order: Vec<usize> = (0..sc.len()).collect();
        order.sort_by(|&i, &j| sc[i].partial_cmp(&sc[j]).unwrap());
        let cut = order[order.len() * 30 / 100];
        let thr = sc[cut];
        let partials = parallel_chunks(&tall, |chunk| {
            let mut local = vec![0.0f64; (COLS * ROWS) as usize];
            for &t in chunk {
                let i = t - start;
                let wgt = (sc[i] - thr).max(0.0);
                let cv = &cvs[i];
                for k in 0..cv.len() { local[k] += cv[k] * wgt; }
            }
            vec![local]
        });
        let mut acc = vec![0.0f64; (COLS * ROWS) as usize];
        for p in &partials {
            for k in 0..acc.len() { acc[k] += p[k]; }
        }
        let tot: f64 = (0..sc.len()).map(|i| (sc[i] - thr).max(0.0)).sum();
        let den = (tot.max(1e-9)) * (CELL * CELL) as f64;
        for i in 0..cells.len() { cells[i] = acc[i] / den; }
        let (c2, f2) = decode(&cells);
        code = c2; confs = f2;
    }

    if dbg {
        eprintln!("frames={} fps={:.1} start={}", n, fps, start);
        let m = cells.iter().sum::<f64>() / cells.len() as f64;
        for r in 0..ROWS as usize {
            let mut line = String::new();
            for c in 0..COLS as usize {
                let v = cells[r * COLS as usize + c];
                line.push(if v > m + 0.01 { '#' } else { '.' });
            }
            eprintln!("{}", line);
        }
        eprintln!("mean={:.3} conf={:?}", m, confs.iter().map(|v| (v * 1000.0).round() / 1000.0).collect::<Vec<_>>());
    }
    println!("code={}", code);
}


