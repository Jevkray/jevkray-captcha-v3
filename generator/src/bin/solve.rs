use std::env;
use std::fs::File;

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

const COLS: i32 = 23;
const ROWS: i32 = 7;
const CELL: i32 = 5;
const BG_TOL: i32 = 2;

fn idx(x: i32, y: i32, w: i32) -> usize { (y * w + x) as usize }

fn fast_map(cur: &[u8], prev: &[u8], w: i32, h: i32) -> Vec<u8> {
    let mut r = vec![0u8; (w * h) as usize];
    for y in 0..h {
        for x in 0..w {
            let c = cur[idx(x, y, w)];
            if c == 0 { continue; }
            let mut ok = false;
            'o: for dy in -BG_TOL..=BG_TOL {
                for dx in -BG_TOL..=BG_TOL {
                    let (px, py) = (x - dx, y - dy);
                    if px >= 0 && py >= 0 && px < w && py < h && prev[idx(px, py, w)] == c {
                        ok = true;
                        break 'o;
                    }
                }
            }
            if !ok { r[idx(x, y, w)] = c; }
        }
    }
    r
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { eprintln!("usage: solve <file.gif> [debug]"); std::process::exit(2); }

    let file = File::open(&args[1]).expect("open gif");
    let mut opts = gif::DecodeOptions::new();
    opts.set_color_output(gif::ColorOutput::Indexed);
    let mut dec = opts.read_info(file).expect("gif header");
    let w = dec.width() as i32;
    let h = dec.height() as i32;

    let mut canvas = vec![0u8; (w * h) as usize];
    let mut frames: Vec<Vec<u8>> = Vec::new();
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
                if gx >= 0 && gy >= 0 && gx < w && gy < h { canvas[idx(gx, gy, w)] = v; }
            }
        }
        frames.push(canvas.clone());
    }

    let n = frames.len();
    let fps = (100.0 / delay as f64).max(1.0);
    let t_end = ((2.5 * fps) as usize).min(n - 1);   // момент завершения сборки цифр
    let t_from = t_end.saturating_sub(8);
    if t_from < 2 { eprintln!("мало кадров"); std::process::exit(1); }

    let fm: Vec<Vec<u8>> = (t_from..=t_end).map(|t| fast_map(&frames[t], &frames[t - 1], w, h)).collect();
    if args.get(2).map(|s| s.as_str()) == Some("debug") {
        for (k, m) in fm.iter().enumerate() { eprintln!("t={} fast={}", t_from + k, m.iter().filter(|&&v| v != 0).count()); }
    }

    // трекер с «докатом»: летящие точки цифр ведём через перекрытия
    struct Trk { x: f64, y: f64, vx: f64, vy: f64, c: u8, miss: i32, hits: i32 }
    let mut acc = vec![0f32; (w * h) as usize];
    let mut tracks: Vec<Trk> = Vec::new();
    let t_all: Vec<usize> = (t_from..=t_end).collect();
    let mut seeded = 0i64;
    for kk in 1..fm.len() {
        let b = &fm[kk];
        // 1) продление треков по предсказанию
        let mut used: Vec<(i32, i32)> = Vec::new();
        for tr in tracks.iter_mut() {
            let px = tr.x + tr.vx;
            let py = tr.y + tr.vy;
            let mut best: Option<(f64, i32, i32)> = None;
            for dy in -3..=3 {
                for dx in -3..=3 {
                    let (qx, qy) = (px as i32 + dx, py as i32 + dy);
                    if qx < 0 || qy < 0 || qx >= w || qy >= h { continue; }
                    if b[idx(qx, qy, w)] != tr.c { continue; }
                    if used.contains(&(qx, qy)) { continue; }
                    let d = (((qx as f64 + 0.5 - px).powi(2) + (qy as f64 + 0.5 - py).powi(2)) as f64).sqrt();
                    if d > 2.0 { continue; }
                    if best.map(|z| d < z.0).unwrap_or(true) { best = Some((d, qx, qy)); }
                }
            }
            match best {
                Some((_, qx, qy)) => {
                    let (nx, ny) = (qx as f64 + 0.5, qy as f64 + 0.5);
                    tr.vx = tr.vx * 0.4 + (nx - tr.x) * 0.6;
                    tr.vy = tr.vy * 0.4 + (ny - tr.y) * 0.6;
                    tr.x = nx; tr.y = ny;
                    tr.miss = 0; tr.hits += 1;
                    used.push((qx, qy));
                }
                None => { tr.x += tr.vx; tr.y += tr.vy; tr.miss += 1; }
            }
        }
        tracks.retain(|t| t.miss <= 3);
        // 2) засев новых треков: пара кадров + подтверждение следующим
        if kk + 1 < fm.len() {
            let a = &fm[kk - 1];
            let c3 = &fm[kk + 1];
            for y in 0..h {
                for x in 0..w {
                    let c = b[idx(x, y, w)];
                    if c == 0 || used.contains(&(x, y)) { continue; }
                    let mut v1: Option<(f64, i32, i32)> = None;
                    for dy in -6..=6 {
                        for dx in -6..=6 {
                            let (qx, qy) = (x + dx, y + dy);
                            if qx < 0 || qy < 0 || qx >= w || qy >= h { continue; }
                            if a[idx(qx, qy, w)] != c { continue; }
                            let d = ((dx * dx + dy * dy) as f64).sqrt();
                            if d < 1.5 { continue; }
                            if v1.map(|z| d < z.0).unwrap_or(true) { v1 = Some((d, dx, dy)); }
                        }
                    }
                    let (_, vx, vy) = match v1 { Some(z) => z, None => continue };
                    let (px, py) = (x + vx, y + vy);
                    let mut ok = false;
                    for dy in -2..=2 {
                        for dx in -2..=2 {
                            let (qx, qy) = (px + dx, py + dy);
                            if qx < 0 || qy < 0 || qx >= w || qy >= h { continue; }
                            if c3[idx(qx, qy, w)] == c { ok = true; }
                        }
                    }
                    if ok {
                        seeded += 1;
                        tracks.push(Trk { x: x as f64 + 0.5, y: y as f64 + 0.5, vx: vx as f64, vy: vy as f64, c, miss: 0, hits: 3 });
                    }
                }
            }
        }
    }
    let good: Vec<&Trk> = tracks.iter().filter(|t| t.hits >= 4).collect();
    eprintln!("tracks={} good={} seeded={}", tracks.len(), good.len(), seeded);
    for t in good.iter() {
        let ax = t.x.round() as i32;
        let ay = t.y.round() as i32;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let (px, py) = (ax + dx, ay + dy);
                if px >= 0 && py >= 0 && px < w && py < h { acc[idx(px, py, w)] += 1.0; }
            }
        }
    }
    let bw = COLS * CELL;
    let bh = ROWS * CELL;
    let mut bestall = (f64::MIN, 0.0f64, 0, 0);
    for adeg in -12..=12 {
        let a = (adeg as f64).to_radians();
        let (ca, sa) = (a.cos(), a.sin());
        let cx = w as f64 / 2.0;
        let cy = h as f64 / 2.0;
        // поворачиваем карту на -угол
        let mut rot = vec![0f32; (w * h) as usize];
        for y in 0..h {
            for x in 0..w {
                let (dx0, dy0) = (x as f64 - cx, y as f64 - cy);
                let sx = (cx + dx0 * ca + dy0 * sa).round() as i32;
                let sy = (cy - dx0 * sa + dy0 * ca).round() as i32;
                if sx >= 0 && sy >= 0 && sx < w && sy < h { rot[idx(x, y, w)] = acc[idx(sx, sy, w)]; }
            }
        }
        // интегральная карта
        let mut sat = vec![0f64; ((w + 1) * (h + 1)) as usize];
        for y in 0..h {
            for x in 0..w {
                sat[((y + 1) * (w + 1) + x + 1) as usize] = rot[idx(x, y, w)] as f64
                    + sat[(y * (w + 1) + x + 1) as usize]
                    + sat[((y + 1) * (w + 1) + x) as usize]
                    - sat[(y * (w + 1) + x) as usize];
            }
        }
        for oy in -bh..h {
            for ox in -bw..w {
                let (xa, ya) = (ox.max(0), oy.max(0));
                let (xb, yb) = ((ox + bw - 1).min(w - 1), (oy + bh - 1).min(h - 1));
                if xb < xa || yb < ya { continue; }
                let s = sat[((yb + 1) * (w + 1) + xb + 1) as usize]
                    - sat[(ya * (w + 1) + xb + 1) as usize]
                    - sat[((yb + 1) * (w + 1) + xa) as usize]
                    + sat[(ya * (w + 1) + xa) as usize];
                if s > bestall.0 { bestall = (s, a, ox, oy); }
            }
        }
    }
    let (_, ang, ox, oy) = bestall;

    // читаем цифры: клетки повёрнутого блока
    let (ca, sa) = (ang.cos(), ang.sin());
    let cx = ox as f64 + bw as f64 / 2.0;
    let cy = oy as f64 + bh as f64 / 2.0;
    let mut cells = vec![0f32; (COLS * ROWS) as usize];
    for y in 0..bh {
        for x in 0..bw {
            for dy in 0..CELL {
                for dx in 0..CELL {
                    let lx = (x + dx) as f64 - bw as f64 / 2.0;
                    let ly = (y + dy) as f64 - bh as f64 / 2.0;
                    let sxp = (cx + lx * ca - ly * sa).round() as i32;
                    let syp = (cy + lx * sa + ly * ca).round() as i32;
                    if sxp < 0 || syp < 0 || sxp >= w || syp >= h { continue; }
                    let cc = (x / CELL) as usize;
                    let rr = (y / CELL) as usize;
                    if rr < ROWS as usize && cc < COLS as usize {
                        cells[rr * COLS as usize + cc] += acc[idx(sxp, syp, w)];
                    }
                }
            }
        }
    }

    let mut code = String::new();
    for g in 0..4 {
        let base = (g * 6) as usize;
        let mut bestd = 0usize;
        let mut bests = f32::MIN;
        for dgt in 0..10 {
            let mut lit_sum = 0f32;
            let mut lit_n = 0f32;
            let mut un_sum = 0f32;
            let mut un_n = 0f32;
            for r in 0..ROWS as usize {
                for c in 0..5usize {
                    let lit = FONT[dgt][r].as_bytes()[c] == b'1';
                    let v = cells[r * COLS as usize + base + c];
                    if lit { lit_sum += v; lit_n += 1.0; } else { un_sum += v; un_n += 1.0; }
                }
            }
            let sc = lit_sum / lit_n - un_sum / un_n;
            if sc > bests { bests = sc; bestd = dgt; }
        }
        code.push((b'0' + bestd as u8) as char);
    }

    if args.get(2).map(|s| s.as_str()) == Some("debug") {
        println!("angle={:.0} deg offset=({},{}) t={}..{}", ang.to_degrees(), ox, oy, t_from, t_end);
        for r in 0..ROWS as usize {
            let mut line = String::new();
            for c in 0..COLS as usize { line.push(if cells[r * COLS as usize + c] > 0.0 { '#' } else { '.' }); }
            println!("{}", line);
        }
    }
    println!("code={} frames={}", code, n);
}