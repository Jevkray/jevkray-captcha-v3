(() => {
    const canvas = document.getElementById('cap-canvas');
    const ctx = canvas ? canvas.getContext('2d') : null;

    // закрывашка рисуется прямо в canvas — убрать её через DOM/CSS нельзя
    let hoverDist = 0;
    let revealed = false;
    let lastX = null;
    let lastY = null;

    const drawCover = () => {
        if (!ctx) return;
        ctx.fillStyle = '#0e0e12';
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = '#8b9ab3';
        ctx.font = '12px system-ui, sans-serif';
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        ctx.fillText('Наведись', canvas.width / 2, canvas.height / 2);
    };

    if (canvas) {
        canvas.addEventListener('pointerenter', e => { lastX = e.clientX; lastY = e.clientY; });
        canvas.addEventListener('pointermove', e => {
            if (lastX !== null) hoverDist += Math.abs(e.clientX - lastX) + Math.abs(e.clientY - lastY);
            lastX = e.clientX;
            lastY = e.clientY;
            if (hoverDist > 20) revealed = true;
        });
        canvas.addEventListener('pointerleave', () => {
            revealed = false;
            hoverDist = 0;
            lastX = null;
            lastY = null;
            drawCover();
        });
    }
    const form = document.querySelector('.cap-form');
    const idEl = document.getElementById('cap-id');
    const msgEl = document.getElementById('cap-msg');
    const timeEl = document.getElementById('cap-time');
    const barEl = document.getElementById('cap-bar');
    const barWrap = barEl ? barEl.parentElement : null;
    const T = k => (window.i18n ? window.i18n.t(k) : k);

    const TTL = 30;
    let left = TTL;
    const paintTimer = () => {
        const v = Math.max(left, 0);
        if (timeEl) timeEl.textContent = v;
        if (barEl) barEl.style.width = (v / TTL * 100) + '%';
        if (barWrap) barWrap.classList.toggle('low', v <= 10);
    };

    const showMsg = (text, ok) => {
        if (!msgEl) return;
        msgEl.hidden = !text;
        msgEl.textContent = text || '';
        msgEl.className = 'msg ' + (ok ? 'ok' : 'err');
    };

    // ---- приём потока кадров CRAW: заголовок + палитра, дальше кадры по 160x160 ----
    let streamAbort = null;
    let lut = null;
    let imgData = null;
    let frameBytes = 0;
    let readBuf = new Uint8Array(0);
    let headerDone = false;

    const concat = (a, b) => {
        const out = new Uint8Array(a.length + b.length);
        out.set(a); out.set(b, a.length);
        return out;
    };

    const drawFrame = frame => {
        if (!revealed) { drawCover(); return; }
        const d = imgData.data;
        for (let i = 0; i < frameBytes; i++) {
            const p = frame[i] * 4;
            const o = i * 4;
            d[o] = lut[p];
            d[o + 1] = lut[p + 1];
            d[o + 2] = lut[p + 2];
            d[o + 3] = 255;
        }
        ctx.putImageData(imgData, 0, 0);
    };

    const startStream = async id => {
        if (streamAbort) streamAbort.abort();
        streamAbort = new AbortController();
        lut = null; imgData = null; frameBytes = 0; readBuf = new Uint8Array(0); headerDone = false;
        try {
            const r = await fetch('?handler=Stream&id=' + encodeURIComponent(id), {
                signal: streamAbort.signal,
                cache: 'no-store'
            });
            if (!r.ok || !r.body) return;
            const reader = r.body.getReader();
            for (;;) {
                const { done, value } = await reader.read();
                if (done) break;
                readBuf = concat(readBuf, value);
                if (!headerDone) {
                    if (readBuf.length < 13) continue;
                    if (String.fromCharCode(readBuf[0], readBuf[1], readBuf[2], readBuf[3]) !== 'CRAW') return;
                    const w = readBuf[4] | (readBuf[5] << 8);
                    const h = readBuf[6] | (readBuf[7] << 8);
                    const palLen = readBuf[12];
                    if (readBuf.length < 13 + palLen) continue;
                    lut = new Uint8Array(256 * 4);
                    for (let i = 0; i < palLen / 3; i++) {
                        lut[i * 4] = readBuf[13 + i * 3];
                        lut[i * 4 + 1] = readBuf[14 + i * 3];
                        lut[i * 4 + 2] = readBuf[15 + i * 3];
                        lut[i * 4 + 3] = 255;
                    }
                    canvas.width = w; canvas.height = h;
                    imgData = ctx.createImageData(w, h);
                    frameBytes = w * h;
                    readBuf = readBuf.subarray(13 + palLen);
                    headerDone = true;
                }
                while (readBuf.length >= frameBytes) {
                    drawFrame(readBuf.subarray(0, frameBytes));
                    readBuf = readBuf.subarray(frameBytes);
                }
            }
        } catch { /* отмена или сеть */ }
    };

    const setChallenge = id => {
        if (idEl) idEl.value = id;
        startStream(id);
    };

    const resetTimer = () => { left = TTL; paintTimer(); };

    const newCode = async () => {
        try {
            const r = await fetch('?handler=New', { headers: { 'X-Requested-With': 'fetch' } });
            if (!r.ok) return;
            const j = await r.json();
            setChallenge(j.id);
            resetTimer();
            showMsg('', true);
        } catch { /* network */ }
    };
    window.capReset = newCode;

    if (form) {
        form.addEventListener('submit', async e => {
            e.preventDefault();
            try {
                const r = await fetch(form.action, {
                    method: 'POST',
                    body: new FormData(form),
                    headers: { 'X-Requested-With': 'fetch' }
                });
                if (!r.ok) return;
                const j = await r.json();
                if (j.solved) {
                    showMsg(T('ok'), true);
                    if (streamAbort) streamAbort.abort();
                } else {
                    showMsg(T('err'), false);
                    if (j.id) { setChallenge(j.id); resetTimer(); }
                }
            } catch { /* network */ }
        });
    }

    drawCover();
    paintTimer();
    if (idEl && idEl.value) setChallenge(idEl.value);
    setInterval(() => { left--; paintTimer(); if (left <= 0) newCode(); }, 1000);
})();
