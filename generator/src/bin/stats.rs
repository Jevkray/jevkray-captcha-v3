use std::env;
use std::fs::File;

struct Blob { x: f64, y: f64, c: u8, w: i32, h: i32, n: i32 }

fn extract(frame: &[u8], w: i32, h: i32) -> Vec<Blob> {
    let mut seen = vec![false; (w * h) as usize];
    let mut out = Vec::new();
    let mut stack: Vec<(i32, i32)> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let i = (y * w + x) as usize;
            if frame[i] == 0 || seen[i] { continue; }
            seen[i] = true;
            stack.push((x, y));
            let (mut sx, mut sy, mut n) = (0f64, 0f64, 0i32);
            let (mut x0, mut x1, mut y0, mut y1) = (x, x, y, y);
            let mut colors = [0i32; 8];
            while let Some((cx, cy)) = stack.pop() {
                sx += cx as f64 + 0.5; sy += cy as f64 + 0.5; n += 1;
                if cx < x0 { x0 = cx; } if cx > x1 { x1 = cx; }
                if cy < y0 { y0 = cy; } if cy > y1 { y1 = cy; }
                colors[(frame[(cy * w + cx) as usize] % 8) as usize] += 1;
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (nx, ny) = (cx + dx, cy + dy);
                    if nx >= 0 && ny >= 0 && nx < w && ny < h {
                        let ni = (ny * w + nx) as usize;
                        if frame[ni] != 0 && !seen[ni] { seen[ni] = true; stack.push((nx, ny)); }
                    }
                }
            }
            let mut cc = 0u8; let mut uniq = 0;
            for k in 1..8 { if colors[k] > 0 { uniq += 1; cc = k as u8; } }
            out.push(Blob { x: sx / n as f64, y: sy / n as f64, c: cc, w: x1 - x0 + 1, h: y1 - y0 + 1, n });
            if uniq == 0 { let _ = cc; }
        }
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("open");
    let mut opts = gif::DecodeOptions::new();
    opts.set_color_output(gif::ColorOutput::Indexed);
    let mut dec = opts.read_info(file).expect("hdr");
    let w = dec.width() as i32;
    let h = dec.height() as i32;
    let mut canvas = vec![0u8; (w * h) as usize];
    let mut frames: Vec<Vec<u8>> = Vec::new();
    let mut delays: Vec<u16> = Vec::new();
    while let Some(frame) = dec.read_next_frame().expect("frame") {
        let (fw, fh) = (frame.width as i32, frame.height as i32);
        let (left, top) = (frame.left as i32, frame.top as i32);
        delays.push(frame.delay);
        let tr = frame.transparent;
        for y in 0..fh {
            for x in 0..fw {
                let v = frame.buffer[(y * fw + x) as usize];
                if Some(v) == tr { continue; }
                let (gx, gy) = (left + x, top + y);
                if gx >= 0 && gy >= 0 && gx < w && gy < h { canvas[(gy * w + gx) as usize] = v; }
            }
        }
        frames.push(canvas.clone());
    }
    println!("size={}x{} frames={} delay={:?}", w, h, frames.len(), delays.first());
    let mut total_px = 0i64;
    for y in 0..h { for x in 0..w { if frames[0][(y * w + x) as usize] != 0 { total_px += 1; } } }
    println!("frame0 coverage={:.2}", total_px as f64 / (w * h) as f64);

    for &t in &[10usize, 30, 45, 50, 55, 100, 300] {
        if t >= frames.len() { continue; }
        let blobs = extract(&frames[t], w, h);
        let clean: Vec<&Blob> = blobs.iter().filter(|b| b.n <= 4 && b.w <= 2 && b.h <= 2).collect();
        let mut hist = [0i32; 10];
        for b in blobs.iter() { hist[b.n.min(9) as usize] += 1; }
        println!("t={} blobs={} clean={} sizes={:?}", t, blobs.len(), clean.len(), hist);
    }
}