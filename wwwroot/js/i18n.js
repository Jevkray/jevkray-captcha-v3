(() => {
    const dict = {
        ru: {
            solve_title: "Как её проходит человек",
            demo_hint: "В этой демонстрации всего 6 тестовых GIF. Полноценная генерация — в приложении на .NET + Rust, либо сгенерируйте свои GIF напрямую утилитой на Rust (см. репозиторий).",
            dl_gen: "Скачать настоящий генератор (Rust, Windows, 200 КБ)",
            contact: "Сотрудничество и вопросы:",
            s1: "Наведи мышь на поле — без наведения кадры полностью скрыты и открываются только под курсором.",
            s2: "Дождись сборки: перед тобой 4 цифры, которые движутся и деформируются как единое желе.",
            s3: "Введи их и нажми «Проверить»: даётся 30 секунд.",
            s4: "Плохо видно — «Новый код» пере-рандомизирует разметку, движение и искажения.",
            title: "Проверка",
            head: "Проверка, что вы человек",
            sub: "Наведи мышь на поле и введи 4 цифры",
            left: "Осталось", sec: "с",
            check: "Проверить", newcode: "Новый код",
            ok: "Правильно!", err: "Неверно, попробуйте ещё раз.",
            footer: "© 2026 jevkray. Все права защищены.",
            about_title: "Что это",
            about_p1: "Капча нового типа: код из 4 цифр собран из тех же точек, что и фоновый шум. Отдельного слоя текста нет — цифры собираются в подвижный блок, который ведёт себя как живое желе: тянется, дышит, пульсирует по размеру, а по фону гуляют дрейфующие разрывы и вихри.",
            about_p2: "Одиночный кадр почти неотличим от фона: плотность точек, время жизни и частота появления в штрихах и в фоне специально выровнены. Кадры приходят потоком в реальном времени — готового файла нет, а без наведения поле полностью закрыто.",
            how_title: "Как это работает",
            how1: "Поле 160×160, ~9200 цветных точек из палитры 6 сочетающихся цветов.",
            how2: "Первую секунду код невидим, затем 1.5 с точки слетаются в свои клетки.",
            how3: "Поверх — «жидкая линза» (глобальная рябь и вихри) и «живое желе»: штрихи деформируются, клетки разрываются и склеиваются обратно, по фону дрейфуют пустые места.",
            how4: "Кадры транслируются потоком (20 fps), повторно скачать поток нельзя; ответ проверяется по хэшу на сервере, код живёт 30 секунд.",
            ch_title: "Публичный вызов",
            ch_p1: "Встроенный CV-решатель на 50 свежих GIF даёт 0 точных совпадений. Цель для сообщества — 50%.",
            ch_p2: "Исходники генератора, решатель-эталон, правила и примеры:"
        },
        en: {
            solve_title: "How a human solves it",
            demo_hint: "This demo ships only 6 test GIFs. Full generation lives in the .NET + Rust application — or generate your own GIFs directly with the Rust tool (see the repository).",
            dl_gen: "Download the real generator (Rust, Windows, 200 KB)",
            contact: "Collaboration and inquiries:",
            s1: "Hover the field — without hovering the frames are fully hidden and only reveal under the cursor.",
            s2: "Wait for the assembly: the 4 digits move and deform as a single jelly body.",
            s3: "Type them and hit Verify: you have 30 seconds.",
            s4: "Hard to read — New code re-randomizes layout, motion and distortions.",
            title: "Verification",
            head: "Human verification",
            sub: "Hover the field and enter the 4 digits",
            left: "Time left", sec: "s",
            check: "Verify", newcode: "New code",
            ok: "Correct!", err: "Wrong, try again.",
            footer: "© 2026 jevkray. All rights reserved.",
            about_title: "What is this",
            about_p1: "A new kind of CAPTCHA: the 4-digit code is made of the very same dots as the background noise. There is no separate text layer — the digits assemble into a moving block that behaves like living jelly: stretching, breathing, pulsing in size, while drifting tears and vortices roam the background.",
            about_p2: "A single frame is almost indistinguishable from the background: dot density, lifetime and appearance rate inside glyph strokes and in the background are deliberately equalized. Frames arrive as a live stream — there is no downloadable file, and without hovering the field is fully hidden.",
            how_title: "How it works",
            how1: "160x160 field, ~9200 colored dots from a palette of 6 matching colors.",
            how2: "For the first second the code is invisible, then dots fly into their cells for 1.5 s.",
            how3: "On top — a “liquid lens” (global ripples and vortices) and “living jelly”: strokes deform, cells tear and re-glue, empty spots drift across the background.",
            how4: "Frames are streamed at 20 fps and cannot be re-downloaded; the answer is verified against a server-side hash, the code lives 30 seconds.",
            ch_title: "Public challenge",
            ch_p1: "The built-in CV solver scores 0 exact matches on 50 fresh GIFs. The community target is 50%.",
            ch_p2: "Generator sources, reference solver, rules and samples:"
        },
        zh: {
            solve_title: "人类如何通过",
            demo_hint: "本演示只包含 6 个测试 GIF。完整生成在 .NET + Rust 应用中，或直接用 Rust 工具自行生成 GIF（见仓库）。",
            dl_gen: "下载真正的生成器（Rust，Windows，200 KB）",
            contact: "合作与咨询：",
            s1: "将鼠标悬停在场上——未悬停时画面完全隐藏，只在光标附近显现。",
            s2: "等待汇聚完成：4 位数字像一块果冻一样整体运动并变形。",
            s3: "输入后点击“验证”，限时 30 秒。",
            s4: "看不清就点“换一个”，布局、运动与扭曲都会重新随机化。",
            title: "验证",
            head: "人机验证",
            sub: "将鼠标悬停在场上并输入 4 位数字",
            left: "剩余", sec: "秒",
            check: "验证", newcode: "换一个",
            ok: "正确！", err: "错误，请重试。",
            footer: "© 2026 jevkray. 保留所有权利。",
            about_title: "这是什么",
            about_p1: "一种新型验证码：4 位数字由与背景噪点完全相同的点构成。没有单独的文字图层——数字汇成一个像“活果冻”一样运动的块：拉伸、呼吸、尺寸脉动，背景中还有漂移的裂口与漩涡。",
            about_p2: "单帧几乎无法与背景区分：数字笔画与背景中的点密度、寿命和出现频率都被刻意对齐。画面以实时流传输——没有可下载的文件，未悬停时整个场地完全隐藏。",
            how_title: "工作原理",
            how1: "160×160 的场地，约 9200 个彩色点，取自 6 种协调配色。",
            how2: "第一秒数字不可见，随后 1.5 秒内点飞入各自的格子。",
            how3: "其上叠加“液态镜头”（全局波纹与漩涡）和“活果冻”：笔画变形，格子撕裂后又重新粘合，空斑在背景中漂移。",
            how4: "画面以 20 fps 流式传输，无法重复下载；答案在服务器端按哈希校验，验证码有效期为 30 秒。",
            ch_title: "公开挑战",
            ch_p1: "内置 CV 求解器在 50 个新生成的 GIF 上取得 0 次完全匹配。社区目标为 50%。",
            ch_p2: "生成器源码、参考求解器、规则与样例："
        },
        ja: {
            solve_title: "人間はどう解くか",
            demo_hint: "このデモにはテスト用 GIF が 6 つだけです。完全版は .NET + Rust アプリ、または Rust ツールで直接 GIF を生成できます（リポジトリ参照）。",
            dl_gen: "本物のジェネレーターをダウンロード（Rust、Windows、200KB）",
            contact: "協業・お問い合わせ：",
            s1: "フィールドにマウスを合わせてください。合わせていない間は映像が完全に隠れ、カーソル付近だけが見えます。",
            s2: "集合を待つと、4 桁の数字が 1 つのゼリーのように動きながら変形します。",
            s3: "入力して「確認」を押します（30 秒）。",
            s4: "読みにくければ「新しいコード」で配置・動き・歪みが再ランダム化されます。",
            title: "確認",
            head: "人間であることの確認",
            sub: "フィールドにマウスを合わせて 4 桁の数字を入力してください",
            left: "残り", sec: "秒",
            check: "確認", newcode: "新しいコード",
            ok: "正解です！", err: "間違いです。もう一度お試しください。",
            footer: "© 2026 jevkray. 無断転載を禁じます。",
            about_title: "これは何か",
            about_p1: "新しいタイプの CAPTCHA です。4 桁のコードは背景ノイズとまったく同じ点で構成されています。文字のレイヤーはなく、数字は「生きたゼリー」のように動く塊として現れます：伸び、呼吸し、大きさが脈動し、背景には漂う裂け目と渦が走ります。",
            about_p2: "1 フレームだけでは背景とほぼ区別できません。数字のストロークと背景で、点の密度・寿命・出現頻度が意図的に揃えられています。映像はリアルタイムのストリームで届き、ダウンロードできるファイルはありません。ホバーしない限りフィールドは完全に隠されています。",
            how_title: "仕組み",
            how1: "160×160 の領域に約 9200 個の色付きの点、6 色の調和したパレット。",
            how2: "最初の 1 秒はコードは見えず、その後 1.5 秒かけて点がそれぞれのセルへ飛び込みます。",
            how3: "さらに「液体レンズ」（全体の波紋と渦）と「生きたゼリー」：ストロークは変形し、セルは裂けて再び接着し、空き穴が背景を漂います。",
            how4: "映像は 20 fps のストリームで配信され、再ダウンロードはできません。回答はサーバー側でハッシュ照合され、コードの有効期限は 30 秒です。",
            ch_title: "公開チャレンジ",
            ch_p1: "内蔵の CV ソルバーは新しい 50 個の GIF で完全一致 0 件。コミュニティの目標は 50% です。",
            ch_p2: "生成器のソース、参照ソルバー、ルール、サンプル："
        },
        fr: {
            solve_title: "Comment un humain le résout",
            demo_hint: "Cette démo ne contient que 6 GIF de test. La génération complète est dans l'application .NET + Rust, ou générez vos GIF avec l'outil Rust (voir le dépôt).",
            dl_gen: "Télécharger le vrai générateur (Rust, Windows, 200 Ko)",
            contact: "Collaboration et contact :",
            s1: "Survolez le champ — sans survol les images sont entièrement masquées et ne se révèlent que sous le curseur.",
            s2: "Attendez l'assemblage : les 4 chiffres bougent et se déforment comme une seule gelée.",
            s3: "Saisissez-les et cliquez sur Vérifier : 30 secondes.",
            s4: "Difficile à lire — Nouveau code re-randomise disposition, mouvement et distorsions.",
            title: "Vérification",
            head: "Vérification humaine",
            sub: "Survolez le champ et saisissez les 4 chiffres",
            left: "Temps restant", sec: "s",
            check: "Vérifier", newcode: "Nouveau code",
            ok: "Correct !", err: "Incorrect, réessayez.",
            footer: "© 2026 jevkray. Tous droits réservés.",
            about_title: "De quoi s'agit-il",
            about_p1: "Un CAPTCHA d'un nouveau type : le code à 4 chiffres est composé des mêmes points que le bruit de fond. Il n'y a pas de calque de texte — les chiffres s'assemblent en un bloc mouvant qui se comporte comme une gelée vivante : il s'étire, respire, pulse en taille, tandis que des déchirures et des tourbillons dérivent sur le fond.",
            about_p2: "Une image seule est presque indiscernable du fond : densité des points, durée de vie et fréquence d'apparition dans les traits et dans le fond sont volontairement égalisées. Les images arrivent en flux temps réel — aucun fichier téléchargeable, et sans survol le champ est entièrement masqué.",
            how_title: "Comment ça marche",
            how1: "Champ 160×160, ~9200 points colorés issus d'une palette de 6 couleurs harmonieuses.",
            how2: "La première seconde, le code est invisible, puis les points rejoignent leurs cellules en 1,5 s.",
            how3: "Par-dessus — une « lentille liquide » (ondulations globales et tourbillons) et une « gelée vivante » : les traits se déforment, les cellules se déchirent et se recollent, des trous dérivent sur le fond.",
            how4: "Les images sont diffusées en flux à 20 fps, impossible de les retélécharger ; la réponse est vérifiée par hachage côté serveur, le code vit 30 secondes.",
            ch_title: "Défi public",
            ch_p1: "Le solveur CV intégré obtient 0 correspondance exacte sur 50 GIF récents. L'objectif de la communauté est de 50 %.",
            ch_p2: "Sources du générateur, solveur de référence, règles et exemples :"
        },
        es: {
            solve_title: "Cómo lo resuelve un humano",
            demo_hint: "Esta demo solo incluye 6 GIF de prueba. La generación completa está en la aplicación .NET + Rust, o genera tus GIF con la herramienta Rust (ver el repositorio).",
            dl_gen: "Descargar el generador real (Rust, Windows, 200 KB)",
            contact: "Colaboración y consultas:",
            s1: "Pasa el ratón por el campo: sin hover los fotogramas quedan totalmente ocultos y solo se revelan bajo el cursor.",
            s2: "Espera al ensamblaje: los 4 dígitos se mueven y se deforman como una sola gelatina.",
            s3: "Introdúcelos y pulsa Verificar: 30 segundos.",
            s4: "Si cuesta leerlo — Nuevo código vuelve a aleatorizar la disposición, el movimiento y las distorsiones.",
            title: "Verificación",
            head: "Verificación humana",
            sub: "Pasa el ratón por el campo e introduce los 4 dígitos",
            left: "Tiempo restante", sec: "s",
            check: "Verificar", newcode: "Nuevo código",
            ok: "¡Correcto!", err: "Incorrecto, inténtalo de nuevo.",
            footer: "© 2026 jevkray. Todos los derechos reservados.",
            about_title: "Qué es esto",
            about_p1: "Un CAPTCHA de nuevo tipo: el código de 4 dígitos está formado por los mismos puntos que el ruido de fondo. No hay capa de texto separada: los dígitos se ensamblan en un bloque móvil que se comporta como una gelatina viva: se estira, respira, pulsa de tamaño, mientras por el fondo vagan desgarros y vórtices.",
            about_p2: "Un solo fotograma es casi indistinguible del fondo: la densidad de puntos, la vida útil y la frecuencia de aparición en los trazos y en el fondo están igualadas a propósito. Los fotogramas llegan en flujo en tiempo real: no hay archivo descargable, y sin pasar el ratón el campo queda totalmente oculto.",
            how_title: "Cómo funciona",
            how1: "Campo de 160×160, ~9200 puntos de color de una paleta de 6 colores armonizados.",
            how2: "Durante el primer segundo el código es invisible; después los puntos vuelan a sus celdas en 1,5 s.",
            how3: "Encima — una «lente líquida» (ondas globales y vórtices) y una «gelatina viva»: los trazos se deforman, las celdas se desgarran y se vuelven a pegar, y por el fondo vagan huecos.",
            how4: "Los fotogramas se transmiten en flujo a 20 fps y no se pueden volver a descargar; la respuesta se verifica con un hash en el servidor y el código vive 30 segundos.",
            ch_title: "Reto público",
            ch_p1: "El solucionador CV integrado obtiene 0 coincidencias exactas en 50 GIF recientes. El objetivo de la comunidad es el 50 %.",
            ch_p2: "Código del generador, solucionador de referencia, reglas y ejemplos:"
        }
    };
    const codes = ["ru", "en", "zh", "ja", "fr", "es"];
    const norm = l => {
        l = (l || "").slice(0, 2).toLowerCase();
        if (l === "zh" || l === "cn") return "zh";
        return codes.indexOf(l) >= 0 ? l : "ru";
    };
    const safeGet = k => { try { return localStorage.getItem(k); } catch (e) { return null; } };
    const safeSet = (k, v) => { try { localStorage.setItem(k, v); } catch (e) { } };
    let lang = norm(safeGet("jp-lang") || navigator.language);
    const apply = () => {
        document.documentElement.lang = lang;
        document.documentElement.dataset.lang = lang;
        document.title = dict[lang].title + " — jevkray-captcha-v3";
        document.querySelectorAll("[data-i18n]").forEach(el => {
            const k = el.dataset.i18n;
            if (dict[lang][k] !== undefined) el.textContent = dict[lang][k];
        });
        document.querySelectorAll("[data-i18n-ph]").forEach(el => {
            const k = el.dataset.i18nPh;
            if (dict[lang][k] !== undefined) el.placeholder = dict[lang][k];
        });
        document.querySelectorAll("[data-set-lang]").forEach(b => b.classList.toggle("active", b.dataset.setLang === lang));
    };
    const setLang = l => { l = norm(l); lang = l; safeSet("jp-lang", l); apply(); };
    window.i18n = { t: k => (dict[lang][k] !== undefined ? dict[lang][k] : k), setLang, apply, lang: () => lang };
    document.addEventListener("DOMContentLoaded", () => {
        document.querySelectorAll("[data-set-lang]").forEach(b => b.addEventListener("click", () => setLang(b.dataset.setLang)));
        apply();
    });
    apply();
})();