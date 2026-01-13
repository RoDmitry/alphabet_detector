use ::std::hint::black_box;
use alphabet_detector::{ch_norm, fulltext, fulltext_filter_max, fulltext_filter_with_margin};
use criterion::{criterion_group, criterion_main, Criterion};

const SENTENCES: &[&str] = &[
    "و في نفس الوقت أقول بأن الشيخ صالح لم يشر إلى مسؤولية الدولة التي تسمح لمواطنيها بملكية قنوات تبث ما تبث بل إنه حصر المسؤولية على ملاك هذه القنوات.",
    "102年度彰化县劳工运动会暨园游会于12月1日(星期日)上午在县立体育场盛大登场，来自全县共61个事业单位及职业工会超过3,000多位选手参加，运动会场将展开一系列的竞技对战。",
    "Aan de fysieke gesteldheid van de aspirant-beoefenaar worden geen bijzondere eisen gesteld anders dan een goede gezondheid.",
    "Here, in a region abundant with natural beauty, golfers will surely be rewarded with an exceptional golf experience.",
    "Les affranchissements étaient très rares et s'ils accordaient la liberté à l'ancien esclave, ils ne lui conféraient pas le titre de citoyen.",
    "Natürlich war sie kein Pferd, dachte sie, aber warum wurde sie dann geritten, hatte einen Reiter zu tragen, war gesattelt, bekam Sporen und Lederpeitsche?",
    "अब इन्हें एक अलग प्लेट में निकाल कर गरमा-गरम आलू की सब्जी, हरे धनिये की चटनी या मीठी चटनी के साथ परोस कर खाइये और सबको खिलाइये।",
    "Alla fine del secolo cambiarono nome, divenendo uno Capitano e l’altro Difensore, ma mantenendo le stesse caratteristiche degli anni precedenti.",
    "・京都大学施設に電離圏における電子数などの状況を取得可能なイオノゾンデ受信機（斜入射観測装置）を設置することで、新たな観測手法が地震先行現象検出に資するかを検証する。",
    "아울러 가장 많은 수가 일하고 있는 직업은 곡식작물 재배자(109만6천명)로 조사됐고, 상점판매 및 관리인(97만8천명), 상점판매원(87만3천명), 일반 영업원(59만명) 등이 뒤를 이었다.",
    "Dizer que não estou, significaria explicar porquê e não me apetece nada desfiar o rosário das minhas lamentações.",
    "То есть присяжные не сочли возможным осудить за соучастие в убийстве и убийство людей, доказательства вины которых не были предъявлены.",
    "Con frecuencia creo que Francia es malinterpretada, seala, aludiendo a la imagen que tiene el pas internacionalmente en materia de tica de trabajo.",
    "Med dagens stadshusmajoritet är det övervikt för ett enplanstorg med bostäder, alltså för en ombyggnad i linje med alternativ maxi.",
    "Mezuniyet hediyesi olarak yerleşkenin kuzey batı bölümüne dikilmiş vişnelerin meyvesini, tohumunu almışlardır.",
    "Cuốn sách là cẩm nang hữu ích để tham khảo và học hỏi, giúp các bà mẹ Việt tự tin hơn trong cách dạy con.",
];

fn benchmark(c: &mut Criterion) {
    let mut group1 = c.benchmark_group("Sentences");
    group1.bench_function("fulltext", |bencher| {
        bencher.iter(|| {
            SENTENCES.iter().for_each(|sentence| {
                let data = fulltext::<Vec<char>>(sentence.char_indices());
                black_box(data);
            });
        });
    });
    group1.bench_function("fulltext_filter_with_margin", |bencher| {
        bencher.iter(|| {
            SENTENCES.iter().for_each(|sentence| {
                let data = fulltext_filter_with_margin::<Vec<char>, 95>(sentence.char_indices());
                #[allow(unused_must_use)]
                black_box(data);
            });
        });
    });
    group1.bench_function("fulltext_filter_max", |bencher| {
        bencher.iter(|| {
            SENTENCES.iter().for_each(|sentence| {
                let data = fulltext_filter_max::<Vec<char>>(sentence.char_indices());
                let _ = black_box(data);
            });
        });
    });
    group1.bench_function("ch_norm", |bencher| {
        bencher.iter(|| {
            SENTENCES.iter().for_each(|sentence| {
                let chars = ch_norm::from_ch_ind(sentence.char_indices());
                for ch in chars {
                    black_box(ch);
                }
            });
        });
    });
    group1.finish();
}

const TO_COMPOSE: &str = "ầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöűầûöű";

fn benchmark_compose(c: &mut Criterion) {
    let mut group1 = c.benchmark_group("Char compose");
    group1.bench_function("ch_norm", |bencher| {
        bencher.iter(|| {
            let chars = ch_norm::from_ch_ind(TO_COMPOSE.char_indices());
            for ch in chars {
                black_box(ch);
            }
        });
    });
    group1.finish();
}

criterion_group!(benches, benchmark, benchmark_compose);
criterion_main!(benches);
