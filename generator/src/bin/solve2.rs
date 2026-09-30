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

struct Frames { w: i32, h: i32, cov: Vec<Vec<u8>>, delay: u16 }

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

fn cellvals(f: &[u8], mean: &[f32], w: i32, h: i32, cx: f64, cy: f64, a: f64, s: f64, bw: i32, bh: i32) -> Vec<f64> {
    let (ca, sa) = (a.cos(), a.sin());
    let mut out = vec![0.0f64; (COLS * ROWS) as usize];
    for r in 0..ROWS {
        for c in 0..COLS {
            let mut acc = 0.0;
            for sy in 0..CELL {
                for sx in 0..CELL {
                    let lx = ((c * CELL + sx) as f64 - bw as f64 / 2.0) * s;
                    let ly = ((r * CELL + sy) as f64 - bh as f64 / 2.0) * s;
                    let wx = cx + lx * ca - ly * sa;
                    let wy = cy + lx * sa + ly * ca;
                    let xi = wx.round() as i32;
                    let yi = wy.round() as i32;
                    if xi >= 0 && yi >= 0 && xi < w && yi < h {
                        acc += f[(yi * w + xi) as usize] as f64 - mean[(yi * w + xi) as usize] as f64;
                    }
                }
            }
            out[(r * COLS + c) as usize] = acc;
        }
    }
    out
}

fn rectscore(f: &[u8], mean: &[f32], w: i32, h: i32, cx: f64, cy: f64, ang: f64, s: f64, bw: i32, bh: i32, step: f64) -> f64 {
    let (ca, sa) = (ang.cos(), ang.sin());
    let hwi = bw as f64 * s / 2.0;
    let hhi = bh as f64 * s / 2.0;
    let mut sum = 0.0f64;
    let mut n = 0.0f64;
    let mut ly = -hhi;
    while ly < hhi {
        let mut lx = -hwi;
        while lx < hwi {
            let wx = cx + lx * ca - ly * sa;
            let wy = cy + lx * sa + ly * ca;
            let xi = wx.round() as i32;
            let yi = wy.round() as i32;
            if xi >= 0 && yi >= 0 && xi < w && yi < h {
                sum += f[(yi * w + xi) as usize] as f64 - mean[(yi * w + xi) as usize] as f64;
            }
            n += 1.0;
            lx += step;
        }
        ly += step;
    }
    sum / n.max(1.0)
}

fn parallel_chunks<T: Sync, R: Send>(items: &[T], f: impl Fn(&[T]) -> Vec<R> + Sync) -> Vec<R> {
    let nthreads = std::thread::available_parallelism().map(|v| v.get()).unwrap_or(1);
    if nthreads <= 1 || items.is_empty() { return f(items); }
    let chunk = items.len().div_ceil(nthreads);
    let f = &f;
    let mut out = Vec::with_capacity(items.len());
    std::thread::scope(|s| {
        let handles: Vec<_> = items.chunks(chunk).map(|c| s.spawn(move || f(c))).collect();
        for h in handles { out.extend(h.join().unwrap()); }
    });
    out
}

// локализация при scale=1 (положение центра + угол)
fn locate(f: &[u8], mean: &[f32], w: i32, h: i32, bw: i32, bh: i32) -> (f64, f64, f64) {
    let mut best = (f64::MIN, 0.0, 0.0, 0.0);
    let mut adeg = -12.0;
    while adeg <= 12.0 {
        let a = adeg * PI / 180.0;
        let mut cyi = (bh / 2 + 4) as f64;
        while cyi <= (h - bh / 2 - 4) as f64 {
            let mut cxi = (bw / 2 + 4) as f64;
            while cxi <= (w - bw / 2 - 4) as f64 {
                let sc = rectscore(f, mean, w, h, cxi, cyi, a, 1.0, bw, bh, 3.0);
                if sc > best.0 { best = (sc, cxi, cyi, a); }
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
                let sc = rectscore(f, mean, w, h, cx, cy, a, 1.0, bw, bh, 1.0);
                if sc > fb.0 { fb = (sc, cx, cy, a); }
                da += 0.5;
            }
        }
    }
    (fb.1, fb.2, fb.3)
}

fn decode_cells(cells: &[f64]) -> (String, Vec<f64>) {
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
}

fn contrast_sum(cells: &[f64]) -> f64 {
    let mut total = 0.0;
    for g in 0..4 {
        let base = (g * 6) as usize;
        let mut best = f64::MIN;
        for dgt in 0..10 {
            let mut ls = 0.0; let mut ln = 0.0; let mut us = 0.0; let mut un = 0.0;
            for r in 0..ROWS as usize {
                for c in 0..5usize {
                    let v = cells[r * COLS as usize + base + c];
                    if FONT[dgt][r].as_bytes()[c] == b'1' { ls += v; ln += 1.0; }
                    else { us += v; un += 1.0; }
                }
            }
            let sc = ls / ln - us / un;
            if sc > best { best = sc; }
        }
        total += best;
    }
    total
}

fn accumulate4(cov: &[Vec<u8>], mean: &[f32], est: &[(f64, f64, f64, f64)], tall: &[usize], w: i32, h: i32, bw: i32, bh: i32) -> Vec<f64> {
    let partials = parallel_chunks(tall, |chunk| {
        let mut local = vec![0.0f64; (COLS * ROWS) as usize];
        for &t in chunk {
            let (cx, cy, a, s) = est[t];
            let cv = cellvals(&cov[t], mean, w, h, cx, cy, a, s, bw, bh);
            for k in 0..cv.len() { local[k] += cv[k]; }
        }
        vec![local]
    });
    let mut acc = vec![0.0f64; (COLS * ROWS) as usize];
    for p in &partials { for k in 0..acc.len() { acc[k] += p[k]; } }
    acc
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

    let mut mean = vec![0.0f32; (w * h) as usize];
    let mut cnt = 0.0f32;
    for t in start..n {
        let f = &fr.cov[t];
        for i in 0..(w * h) as usize { mean[i] += f[i] as f32; }
        cnt += 1.0;
    }
    if cnt < 1.0 { eprintln!("мало кадров"); std::process::exit(1); }
    for m in mean.iter_mut() { *m /= cnt; }

    // центр/угол при scale=1
    let mut est3: Vec<(f64, f64, f64)> = vec![(0.0, 0.0, 0.0); n];
    let step_t = 2usize;
    let mut ts: Vec<usize> = Vec::new();
    let mut t = start;
    while t < n { ts.push(t); t += step_t; }
    for (t, v) in parallel_chunks(&ts, |chunk| {
        chunk.iter().map(|&t| (t, locate(&fr.cov[t], &mean, w, h, bw, bh))).collect::<Vec<_>>()
    }) { est3[t] = v; }
    for i in 1..ts.len() {
        let (t0, t1) = (ts[i - 1], ts[i]);
        for k in t0 + 1..t1 {
            let f = (k - t0) as f64 / (t1 - t0) as f64;
            est3[k] = (
                est3[t0].0 + (est3[t1].0 - est3[t0].0) * f,
                est3[t0].1 + (est3[t1].1 - est3[t0].1) * f,
                est3[t0].2 + (est3[t1].2 - est3[t0].2) * f,
            );
        }
    }
    let mut sm = est3.clone();
    for t in start..n {
        for axis in 0..3 {
            let mut v: Vec<f64> = Vec::new();
            for k in -3i64..=3i64 {
                let tt = t as i64 + k;
                if tt >= start as i64 && (tt as usize) < n {
                    let e = est3[tt as usize];
                    v.push(match axis { 0 => e.0, 1 => e.1, _ => e.2 });
                }
            }
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let med = v[v.len() / 2];
            match axis { 0 => sm[t].0 = med, 1 => sm[t].1 = med, _ => sm[t].2 = med }
        }
    }
    let est3 = sm;

    // подбор глобальной траектории масштаба: scale(t)=1+0.25*sin(phase+omega*t)
    let tall: Vec<usize> = (start..n).collect();
    let tall_fit: Vec<usize> = (start..n).step_by(2).collect();
    let mut best_fit = (f64::MIN, 0.03, 0.0);
    for &omega in &[0.020, 0.025, 0.030, 0.035, 0.040, 0.045f64] {
        for kphase in 0..16 {
            let phase = (kphase as f64) * PI / 8.0;
            let est: Vec<(f64, f64, f64, f64)> = (0..n).map(|t| {
                let s = 1.0 + 0.25 * (phase + omega * t as f64).sin();
                (est3[t].0, est3[t].1, est3[t].2, s)
            }).collect();
            let acc = accumulate4(&fr.cov, &mean, &est, &tall_fit, w, h, bw, bh);
            let cells: Vec<f64> = acc.iter().map(|v| v / (tall_fit.len() as f64 * (CELL * CELL) as f64)).collect();
            let sc = contrast_sum(&cells);
            if sc > best_fit.0 { best_fit = (sc, omega, phase); }
        }
    }
    // уточнение omega/phase
    let (mut bo, mut bp) = (best_fit.1, best_fit.2);
    for _ in 0..2 {
        for do_ in [-0.005f64, -0.0025, 0.0, 0.0025, 0.005] {
            for dp in [-0.3f64, -0.15, 0.0, 0.15, 0.3] {
                let omega = bo + do_;
                let phase = bp + dp;
                let est: Vec<(f64, f64, f64, f64)> = (0..n).map(|t| {
                    let s = 1.0 + 0.25 * (phase + omega * t as f64).sin();
                    (est3[t].0, est3[t].1, est3[t].2, s)
                }).collect();
                let acc = accumulate4(&fr.cov, &mean, &est, &tall_fit, w, h, bw, bh);
                let cells: Vec<f64> = acc.iter().map(|v| v / (tall_fit.len() as f64 * (CELL * CELL) as f64)).collect();
                let sc = contrast_sum(&cells);
                if sc > best_fit.0 { best_fit = (sc, omega, phase); bo = omega; bp = phase; }
            }
        }
    }
    let (bm, bph) = (best_fit.1, best_fit.2);
    let mut est: Vec<(f64, f64, f64, f64)> = (0..n).map(|t| {
        let s = 1.0 + 0.25 * (bph + bm * t as f64).sin();
        (est3[t].0, est3[t].1, est3[t].2, s)
    }).collect();

    let acc = accumulate4(&fr.cov, &mean, &est, &tall, w, h, bw, bh);
    let accn = (n - start) as f64;
    let mut cells: Vec<f64> = acc.iter().map(|v| v / (accn * (CELL * CELL) as f64)).collect();
    let (mut code, mut confs) = decode_cells(&cells);

    for _iter in 0..4 {
        let tm = cells.iter().sum::<f64>() / cells.len() as f64;
        let templ: Vec<f64> = cells.iter().map(|v| v - tm).collect();
        for (t, v) in parallel_chunks(&tall, |chunk| {
            chunk.iter().map(|&t| {
                let (bx, by, ba, bs) = est[t];
                let mut best = (f64::MIN, bx, by, ba, bs);
                for dcy in -3..=3 {
                    for dcx in -3..=3 {
                        let mut da = -2.0;
                        while da <= 2.0 {
                            let mut ds = -0.05;
                            while ds <= 0.051 {
                                let cx = bx + dcx as f64;
                                let cy = by + dcy as f64;
                                let a = ba + da * PI / 180.0;
                                let s = bs + ds;
                                ds += 0.025;
                                if s < 0.7 || s > 1.3 { continue; }
                                let m = bw as f64 * s / 2.0 + 1.0;
                                let mh = bh as f64 * s / 2.0 + 1.0;
                                if cx < m || cy < mh || cx > w as f64 - m || cy > h as f64 - mh { continue; }
                                let cv = cellvals(&fr.cov[t], &mean, w, h, cx, cy, a, s, bw, bh);
                                let mut sc = 0.0;
                                for i in 0..cv.len() { sc += cv[i] * templ[i]; }
                                if sc > best.0 { best = (sc, cx, cy, a, s); }
                            }
                            da += 0.5;
                        }
                    }
                }
                (t, (best.1, best.2, best.3, best.4))
            }).collect::<Vec<_>>()
        }) { est[t] = v; }

        let mut sc: Vec<f64> = vec![0.0; n - start];
        let mut cvs: Vec<Vec<f64>> = vec![Vec::new(); n - start];
        for (t, s, cv) in parallel_chunks(&tall, |chunk| {
            chunk.iter().map(|&t| {
                let (cx, cy, a, ss) = est[t];
                let cv = cellvals(&fr.cov[t], &mean, w, h, cx, cy, a, ss, bw, bh);
                let mut sc = 0.0;
                for i in 0..cv.len() { sc += cv[i] * templ[i]; }
                (t, sc, cv)
            }).collect::<Vec<_>>()
        }) { sc[t - start] = s; cvs[t - start] = cv; }
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
        let mut acc2 = vec![0.0f64; (COLS * ROWS) as usize];
        for p in &partials { for k in 0..acc2.len() { acc2[k] += p[k]; } }
        let tot: f64 = (0..sc.len()).map(|i| (sc[i] - thr).max(0.0)).sum();
        let den = (tot.max(1e-9)) * (CELL * CELL) as f64;
        for i in 0..cells.len() { cells[i] = acc2[i] / den; }
        let (c2, f2) = decode_cells(&cells);
        code = c2; confs = f2;
    }

    if dbg {
        eprintln!("frames={} fps={:.1} start={} scale omega={:.3} phase={:.2}", n, fps, start, bm, bph);
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
