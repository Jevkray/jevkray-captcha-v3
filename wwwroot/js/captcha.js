(() => {
    const img = document.getElementById('cap-gif');
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

    const setGif = id => {
        if (idEl) idEl.value = id;
        if (img) img.src = '?handler=Gif&id=' + encodeURIComponent(id) + '&t=' + Date.now();
    };

    const resetTimer = () => { left = TTL; paintTimer(); };

    const newCode = async () => {
        try {
            const r = await fetch('?handler=New', { headers: { 'X-Requested-With': 'fetch' } });
            if (!r.ok) return;
            const j = await r.json();
            setGif(j.id);
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
                } else {
                    showMsg(T('err'), false);
                    if (j.id) { setGif(j.id); resetTimer(); }
                }
            } catch { /* network */ }
        });
    }

    paintTimer();
    setInterval(() => { left--; paintTimer(); if (left <= 0) newCode(); }, 1000);
})();