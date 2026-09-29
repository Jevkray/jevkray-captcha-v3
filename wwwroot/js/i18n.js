(() => {
    const dict = {
        ru: {
            solve_title: "Как её проходит человек",
            demo_hint: "В этой демонстрации всего 6 тестовых GIF. Полноценная генерация — в приложении на .NET + Rust, либо сгенерируйте свои GIF напрямую утилитой на Rust (см. репозиторий).",
            s1: "Подожди ~1 секунду — код проявится за 1.5 с, точки слетятся на места.",
            s2: "Смотри на группу точек, движущуюся единым блоком — это и есть 4 цифры.",
            s3: "Введи их и нажми «Проверить»: даётся 30 секунд.",
            s4: "Плохо видно — «Новый код» пере-рандомизирует разметку и движение.",
            title: "Проверка",
            head: "Проверка, что вы человек",
            sub: "Введите 4 цифры с картинки",
            left: "Осталось", sec: "с",
            check: "Проверить", newcode: "Новый код",
            ok: "Правильно!", err: "Неверно, попробуйте ещё раз.",
            footer: "© 2026 jevkray. Все права защищены.",
            about_title: "Что это",
            about_p1: "Капча нового типа: код из 4 цифр собран из тех же точек, что и фоновый шум. Отдельного слоя текста нет — цифры видны только как группа точек, которая движется как единое целое: переносится, медленно вращается и «дышит».",
            about_p2: "Одиночный кадр почти неотличим от фона: плотность точек, время жизни и частота появления в штрихах цифр и в фоне специально выровнены (принцип «zero knowledge per frame»).",
            how_title: "Как это работает",
            how1: "Поле 160×160, ~9200 цветных точек из палитры 6 сочетающихся цветов.",
            how2: "Первую секунду код невидим, затем 1.5 с точки слетаются в свои клетки.",
            how3: "Фон движется раздробленно — у каждой точки свой разброс скорости и направления; надпись — единственное жёсткое тело.",
            how4: "Проверка идёт по хэшу на сервере, код живёт 30 секунд.",
            ch_title: "Публичный вызов",
            ch_p1: "Встроенный CV-решатель на 50 свежих GIF даёт 0 точных совпадений. Цель для сообщества — 30%.",
            ch_p2: "Исходники генератора, решатель-эталон, правила и примеры:"
        },
        en: {
            solve_title: "How a human solves it",
            demo_hint: "This demo ships only 6 test GIFs. Full generation lives in the .NET + Rust application — or generate your own GIFs directly with the Rust tool (see the repository).",
            s1: "Wait ~1 second — the code appears over 1.5 s.",
            s2: "Watch the group of dots moving as one block — that is the 4 digits.",
            s3: "Type them and hit Verify: you have 30 seconds.",
            s4: "Hard to read — New code re-randomizes layout and motion.",
            title: "Verification",
            head: "Human verification",
            sub: "Enter the 4 digits from the animation",
            left: "Time left", sec: "s",
            check: "Verify", newcode: "New code",
            ok: "Correct!", err: "Wrong, try again.",
            footer: "© 2026 jevkray. All rights reserved.",
            about_title: "What is this",
            about_p1: "A new kind of CAPTCHA: the 4-digit code is made of the very same dots as the background noise. There is no separate text layer — the digits are visible only as a group of dots moving as a single body: translating, slowly rotating and breathing.",
            about_p2: "A single frame is almost indistinguishable from the background: dot density, lifetime and appearance rate inside glyph strokes and in the background are deliberately equalized (the zero knowledge per frame principle).",
            how_title: "How it works",
            how1: "160x160 field, ~9200 colored dots from a palette of 6 matching colors.",
            how2: "For the first second the code is invisible, then dots fly into their cells for 1.5 s.",
            how3: "The background moves in a fragmented way — every dot has its own speed and direction spread; the text is the only rigid body.",
            how4: "The answer is verified against a hash on the server; the code lives for 30 seconds.",
            ch_title: "Public challenge",
            ch_p1: "The built-in CV solver scores 0 exact matches on 50 fresh GIFs. The community target is 30%.",
            ch_p2: "Generator sources, reference solver, rules and samples:"
        },
        zh: {
            solve_title: "人类如何通过",
            demo_hint: "本演示只包含 6 个测试 GIF。完整生成在 .NET + Rust 应用中，或直接用 Rust 工具自行生成 GIF（见仓库）。",
            s1: "等待约 1 秒——代码在 1.5 秒内显现。",
            s2: "盯着整体移动的那一组点——那就是 4 位数字。",
            s3: "输入后点击“验证”，限时 30 秒。",
            s4: "看不清就点“换一个”，布局与运动都会重新随机化。",
            title: "验证",
            head: "人机验证",
            sub: "请输入动图中的 4 位数字",
            left: "剩余", sec: "秒",
            check: "验证", newcode: "换一个",
            ok: "正确！", err: "错误，请重试。",
            footer: "© 2026 jevkray. 保留所有权利。",
            about_title: "这是什么",
            about_p1: "一种新型验证码：4 位数字由与背景噪点完全相同的点构成。没有单独的文字图层——数字只表现为一组整体运动的点：平移、缓慢旋转并“呼吸”。",
            about_p2: "单帧几乎无法与背景区分：数字笔画与背景中的点密度、寿命和出现频率都被刻意对齐（zero knowledge per frame 原则）。",
            how_title: "工作原理",
            how1: "160×160 的场地，约 9200 个彩色点，取自 6 种协调配色。",
            how2: "第一秒数字不可见，随后 1.5 秒内点飞入各自的格子。",
            how3: "背景运动是无序的——每个点都有自己的速度与方向离散；文字是唯一的刚体。",
            how4: "答案在服务器端按哈希校验，验证码有效期为 30 秒。",
            ch_title: "公开挑战",
            ch_p1: "内置 CV 求解器在 50 个新生成的 GIF 上取得 0 次完全匹配。社区目标为 30%。",
            ch_p2: "生成器源码、参考求解器、规则与样例："
        },
        ja: {
            solve_title: "人間はどう解くか",
            demo_hint: "このデモにはテスト用 GIF が 6 つだけです。完全版は .NET + Rust アプリ、または Rust ツールで直接 GIF を生成できます（リポジトリ参照）。",
            s1: "約 1 秒待つとコードが現れます。",
            s2: "まとまって動く点の集まりが 4 桁の数字です。",
            s3: "入力して「確認」を押します（30 秒）。",
            s4: "読みにくければ「新しいコード」で再ランダム化できます。",
            title: "確認",
            head: "人間であることの確認",
            sub: "アニメーションの 4 桁の数字を入力してください",
            left: "残り", sec: "秒",
            check: "確認", newcode: "新しいコード",
            ok: "正解です！", err: "間違いです。もう一度お試しください。",
            footer: "© 2026 jevkray. 無断転載を禁じます。",
            about_title: "これは何か",
            about_p1: "新しいタイプの CAPTCHA です。4 桁のコードは背景ノイズとまったく同じ点で構成されています。文字のレイヤーは存在せず、数字は「ひとつの剛体として動く点の集まり」としてのみ見えます：並進し、ゆっくり回転し、「呼吸」します。",
            about_p2: "1 フレームだけでは背景とほぼ区別できません。数字のストロークと背景で、点の密度・寿命・出現頻度が意図的に揃えられています（zero knowledge per frame の原則）。",
            how_title: "仕組み",
            how1: "160×160 の領域に約 9200 個の色付きの点、6 色の調和したパレット。",
            how2: "最初の 1 秒はコードは見えず、その後 1.5 秒かけて点がそれぞれのセルへ飛び込みます。",
            how3: "背景はばらばらに動きます（各点が独自の速度・方向のばらつきを持つ）。テキストだけが唯一の剛体です。",
            how4: "回答はサーバー側でハッシュ照合され、コードの有効期限は 30 秒です。",
            ch_title: "公開チャレンジ",
            ch_p1: "内蔵の CV ソルバーは新しい 50 個の GIF で完全一致 0 件。コミュニティの目標は 30% です。",
            ch_p2: "生成器のソース、参照ソルバー、ルール、サンプル："
        },
        fr: {
            solve_title: "Comment un humain le résout",
            demo_hint: "Cette démo ne contient que 6 GIF de test. La génération complète est dans l'application .NET + Rust, ou générez vos GIF avec l'outil Rust (voir le dépôt).",
            s1: "Attendez ~1 seconde — le code apparaît en 1,5 s.",
            s2: "Regardez le groupe de points qui bouge d'un seul bloc — ce sont les 4 chiffres.",
            s3: "Saisissez-les et cliquez sur Vérifier : 30 secondes.",
            s4: "Difficile à lire — Nouveau code re-randomise disposition et mouvement.",
            title: "Vérification",
            head: "Vérification humaine",
            sub: "Saisissez les 4 chiffres de l'animation",
            left: "Temps restant", sec: "s",
            check: "Vérifier", newcode: "Nouveau code",
            ok: "Correct !", err: "Incorrect, réessayez.",
            footer: "© 2026 jevkray. Tous droits réservés.",
            about_title: "De quoi s'agit-il",
            about_p1: "Un CAPTCHA d'un nouveau type : le code à 4 chiffres est composé des mêmes points que le bruit de fond. Il n'y a pas de calque de texte séparé — les chiffres ne sont visibles que comme un groupe de points se déplaçant comme un seul corps : translation, rotation lente et respiration.",
            about_p2: "Une image seule est presque indiscernable du fond : la densité des points, leur durée de vie et leur fréquence d'apparition dans les traits des chiffres et dans le fond sont volontairement égalisées (principe zero knowledge per frame).",
            how_title: "Comment ça marche",
            how1: "Champ 160×160, ~9200 points colorés issus d'une palette de 6 couleurs harmonieuses.",
            how2: "La première seconde, le code est invisible, puis les points rejoignent leurs cellules en 1,5 s.",
            how3: "Le fond bouge de façon fragmentée — chaque point a sa propre dispersion de vitesse et de direction ; le texte est le seul corps rigide.",
            how4: "La réponse est vérifiée par hachage côté serveur ; le code vit 30 secondes.",
            ch_title: "Défi public",
            ch_p1: "Le solveur CV intégré obtient 0 correspondance exacte sur 50 GIF récents. L'objectif de la communauté est de 30 %.",
            ch_p2: "Sources du générateur, solveur de référence, règles et exemples :"
        },
        es: {
            solve_title: "Cómo lo resuelve un humano",
            demo_hint: "Esta demo solo incluye 6 GIF de prueba. La generación completa está en la aplicación .NET + Rust, o genera tus GIF con la herramienta Rust (ver el repositorio).",
            s1: "Espera ~1 segundo: el código aparece en 1,5 s.",
            s2: "Fíjate en el grupo de puntos que se mueve como un bloque — esos son los 4 dígitos.",
            s3: "Introdúcelos y pulsa Verificar: 30 segundos.",
            s4: "Si cuesta leerlo — Nuevo código vuelve a aleatorizar la disposición y el movimiento.",
            title: "Verificación",
            head: "Verificación humana",
            sub: "Introduce los 4 dígitos de la animación",
            left: "Tiempo restante", sec: "s",
            check: "Verificar", newcode: "Nuevo código",
            ok: "¡Correcto!", err: "Incorrecto, inténtalo de nuevo.",
            footer: "© 2026 jevkray. Todos los derechos reservados.",
            about_title: "Qué es esto",
            about_p1: "Un CAPTCHA de nuevo tipo: el código de 4 dígitos está formado por los mismos puntos que el ruido de fondo. No hay una capa de texto separada: los dígitos solo se ven como un grupo de puntos que se mueve como un solo cuerpo: se traslada, gira lentamente y respira.",
            about_p2: "Un solo fotograma es casi indistinguible del fondo: la densidad de puntos, la vida útil y la frecuencia de aparición en los trazos de los dígitos y en el fondo están igualadas a propósito (principio zero knowledge per frame).",
            how_title: "Cómo funciona",
            how1: "Campo de 160×160, ~9200 puntos de color de una paleta de 6 colores armonizados.",
            how2: "Durante el primer segundo el código es invisible; después los puntos vuelan a sus celdas en 1,5 s.",
            how3: "El fondo se mueve de forma fragmentada: cada punto tiene su propia dispersión de velocidad y dirección; el texto es el único cuerpo rígido.",
            how4: "La respuesta se verifica con un hash en el servidor; el código vive 30 segundos.",
            ch_title: "Reto público",
            ch_p1: "El solucionador CV integrado obtiene 0 coincidencias exactas en 50 GIF recientes. El objetivo de la comunidad es el 30 %.",
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