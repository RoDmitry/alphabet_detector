mod shared;
#[allow(unused_imports)]
use shared::*;

use ahash::AHashSet;
use alphabet_detector::Language::*;
use alphabet_detector::*;
use rstest::*;

#[rstest(expected_language, word,
    case(Some(Czech), "subjektů"),
    case(Some(Esperanto), "apenaŭ"),
    case(Some(Esperanto), "intermiksiĝis"),
    case(Some(Esperanto), "kreitaĵoj"),
    case(Some(Esperanto), "monaĥinoj"),
    case(Some(Esperanto), "nesufiĉecon"),
    case(Some(Esperanto), "ŝpinante"),
    case(Some(German), "groß"),
    case(Some(Greek), "σχέδια"),
    case(Some(Hawaiian), "pu'u'ō'ō"),
    case(Some(Hungarian), "fekvő"),
    case(Some(Hungarian), "meggyűrűzni"),
    case(Some(Icelandic), "þagnarskyldu"),
    case(Some(Japanese), "ヴェダイヤモンド"),
    case(Some(Kazakh), "ақын"),
    case(Some(Kazakh), "шұрайлы"),
    case(Some(Lithuanian), "įrengus"),
    case(Some(Lithuanian), "mergelės"),
    case(Some(Lithuanian), "slegiamų"),
    case(Some(Macedonian), "ќерка"),
    case(Some(Macedonian), "припаѓа"),
    case(Some(Polish), "budowę"),
    case(Some(Polish), "groźne"),
    case(Some(Polish), "mniejszości"),
    case(Some(Polish), "państwowych"),
    case(Some(Polish), "wystąpią"),
    case(Some(Romanian), "ialomiţa"),
    case(Some(Serbian), "наслеђивања"),
    case(Some(Serbian), "неисквареношћу"),
    case(Some(Slovak), "mŕtvych"),
    case(Some(Slovak), "podĺa"),
    case(Some(Slovak), "pohľade"),
    case(Some(Ukrainian), "ґрунтовому"),
    case(Some(Ukrainian), "пропонує"),
    case(Some(Vietnamese), "biểu"),
    case(Some(Vietnamese), "bỡi"),
    case(Some(Vietnamese), "cằm"),
    case(Some(Vietnamese), "chẳng"),
    case(Some(Vietnamese), "chất"),
    case(Some(Vietnamese), "chỉnh"),
    case(Some(Vietnamese), "chức"),
    case(Some(Vietnamese), "của"),
    case(Some(Vietnamese), "đạp"),
    case(Some(Vietnamese), "dắt"),
    case(Some(Vietnamese), "diễm"),
    case(Some(Vietnamese), "giới"),
    case(Some(Vietnamese), "hậu"),
    case(Some(Vietnamese), "hiền"),
    case(Some(Vietnamese), "hợp"),
    case(Some(Vietnamese), "hưng"),
    case(Some(Vietnamese), "kỳ"),
    case(Some(Vietnamese), "kỷ"),
    case(Some(Vietnamese), "lẻn"),
    case(Some(Vietnamese), "mặn"),
    case(Some(Vietnamese), "mỗi"),
    case(Some(Vietnamese), "một"),
    case(Some(Vietnamese), "mỵ"),
    case(Some(Vietnamese), "nguồn"),
    case(Some(Vietnamese), "nhẫn"),
    case(Some(Vietnamese), "nhở"),
    case(Some(Vietnamese), "những"),
    case(Some(Vietnamese), "phế"),
    case(Some(Vietnamese), "quẩy"),
    case(Some(Vietnamese), "sẵn"),
    case(Some(Vietnamese), "sỏi"),
    case(Some(Vietnamese), "sử"),
    case(Some(Vietnamese), "thần"),
    case(Some(Vietnamese), "thơ"),
    case(Some(Vietnamese), "thờ"),
    case(Some(Vietnamese), "thực"),
    case(Some(Vietnamese), "tổng"),
    case(Some(Vietnamese), "tốt"),
    case(Some(Vietnamese), "từng"),
    case(Some(Vietnamese), "việc"),
    case(Some(Yoruba), "aṣiwèrè"),
    case(Some(Yoruba), "ṣaaju"),

    // unique script
    case(Some(Armenian), "ունենա"),
    case(Some(Georgian), "გარეუბან"),
    case(Some(Greek), "σταμάτησε"),
    case(Some(Gujarati), "ઉપકરણોની"),
    case(Some(Japanese), "びさ"),
    case(Some(Korean), "대결구도가"),
    case(Some(PunjabiEastern), "ਮੋਟਰਸਾਈਕਲਾਂ"),
    case(Some(Tamil), "துன்பங்களை"),
    case(Some(Telugu), "కృష్ణదేవరాయలు"),
    case(Some(Thai), "ในทางหลวงหมายเลข"),
)]
fn test_word_uniq(expected_language: Option<Language>, word: &str) {
    let found_words: Vec<_> = alphabet_detector::from_ch_iter(word.char_indices()).collect();
    if found_words.len() > 1 {
        panic!("Not a word: {} got: {:?}", word, found_words);
    }
    let languages = langs_count_max(found_words[0].langs_cnt).0;

    let language = if languages.len() > 1 {
        None
    } else {
        languages.iter().next().copied()
    };

    assert_eq!(
        language, expected_language,
        "expected {:?} for word '{}', got {:?}",
        expected_language, word, languages
    );
}

#[rstest(expected_language, word, expected_languages,
    case::eng_like(Slovak, "šefčovič's", ahashset!(Bosnian, Croatian, Czech, Latgalian, Latvian, Lithuanian, Silesian, Slovak, Slovene)),
    case(AlbanianTosk, "hashemidëve", ahashset!(Afrikaans, AlbanianTosk, AymaraCentral, Catalan, Dutch, French, Limburgish, Luxembourgish)),
    case(Arabic, "والموضوع", ahashset!(AcehneseJawi, Arabic, ArabicEgyptian, ArabicMesopotamian, ArabicMoroccan, ArabicNajdi, ArabicNorthLevantine, ArabicSouthernYemeni, ArabicSouthLevantine, ArabicTunisian, AzerbaijaniSouth, BanjarJawi, Dari, KanuriCentralAjami, Kashmiri, Kurdish, KurdishCentral, Pashto, PastoSouthern, Persian, PersianWestern, Sindhi, Urdu, Uyghur)),
    case(AzerbaijaniNorth, "məhərrəm", ahashset!(AzerbaijaniNorth, TatarCrimean)),
    case(Belarusian, "павінен", ahashset!(Belarusian, Kazakh, Ukrainian)),
    case(Belarusian, "раскрывае", ahashset!(Bashkir, Belarusian, Kazakh, Kyrgyz, MongolianHalh, Russian, Tatar)),
    case(Bengali, "জানাতে", ahashset!(Assamese, Bengali, BishnupriyaManipuri, Meitei)),
    case(Bulgarian, "довършат", ahashset!(Bashkir, Bulgarian, Kazakh, Kyrgyz, MongolianHalh, OldChurchSlavonic, Russian, Tajik, Tatar)),
    case(Bulgarian, "плаваща", ahashset!(Bashkir, Bulgarian, Kazakh, Kyrgyz, MongolianHalh, OldChurchSlavonic, Russian, Tatar, Ukrainian)),
    case(Catalan, "contradicció", ahashset!(Afrikaans, Asturian, AymaraCentral, Catalan, Czech, Dutch, Fon, Galician, Guarani, Hungarian, Igbo, Irish, Kabuverdianu, Limburgish, Lombard, Luxembourgish, Occitan, Papiamento, Polish, Portuguese, Sardinian, Shona, Sicilian, Slovak, Spanish, Venetian, Vietnamese)),
    case(Catalan, "només", ahashset!(Acehnese, Afrikaans, Asturian, AymaraCentral, Balinese, Bambara, Banjar, Bokmal, Catalan, CreoleHaitian, Czech, Danish, Dutch, Ewe, Fon, French, Galician, Guarani, Hungarian, Icelandic, Igbo, Ilocano, Irish, Italian, Kabuverdianu, Ligurian, Limburgish, Lombard, Luxembourgish, Occitan, Papiamento, Portuguese, Sardinian, Shona, Sicilian, Slovak, Spanish, Venetian, Vietnamese, Waray, Yoruba)),
    case(Catalan, "pràctiques", ahashset!(Catalan, French, Italian, Portuguese, Sardinian, Sicilian, Venetian, Vietnamese)),
    case(Catalan, "substituïts", ahashset!(Afrikaans, AymaraCentral, Catalan, Dutch, French)),
    case(Croatian, "nađete", ahashset!(Bosnian, Croatian, Vietnamese)),
    case(Croatian, "prihvaćanju", ahashset!(Bosnian, Croatian)),
    case(Czech, "jeďte", ahashset!(Czech, Slovak)),
    case(Czech, "navržen", ahashset!(Bosnian, Croatian, Czech, Estonian, Latgalian, Latvian, Lithuanian, Silesian, Slovak, Slovene)),
    case(Czech, "rozdělit", ahashset!(Czech, Fon)),
    case(Czech, "rtuť", ahashset!(Czech, Slovak)),
    case(Czech, "tvořen", ahashset!(Czech, Silesian)),
    case(Czech, "vývoj", ahashset!(Afrikaans, Czech, Faroese, Guarani, Icelandic, Slovak)),
    case(Czech, "zaručen", ahashset!(Bosnian, Croatian, Czech, Kabyle, Latgalian, Latvian, Lithuanian, Silesian, Slovak, Slovene)),
    case(Czech, "zkouškou", ahashset!(Bosnian, Croatian, Czech, Estonian, Latgalian, Latvian, Lithuanian, Sepedi, Silesian, Slovak, Slovene)),
    case(Danish, "direktør", ahashset!(Bokmal, Czech, Danish, Faroese, Nynorsk)),
    case(Danish, "indebærer", ahashset!(Bokmal, Danish, Faroese, French, Icelandic, Latin, Nynorsk)),
    case(Danish, "måned", ahashset!(Bokmal, Danish, Nynorsk, Swedish)),
    case(English, "house", ahashset!(Acehnese, Afrikaans, AlbanianTosk, Asturian, AymaraCentral, AzerbaijaniNorth, Balinese, Bambara, Banjar, Basque, Bemba, Bokmal, Bosnian, Buginese, Catalan, Cebuano, Chokwe, CreoleHaitian, Croatian, Czech, Danish, Dutch, Dyula, English, Esperanto, Estonian, Ewe, Faroese, Finnish, Fon, French, Friulian, FulfuldeNigerian, GaelicScottish, Galician, Ganda, German, Guarani, Hausa, Hungarian, Icelandic, Igbo, Indonesian, Irish, Italian, Javanese, Jingpho, Kabiye, Kabuverdianu, Kabyle, Kamba, KanuriCentral, Kikongo, Kikuyu, Kimbundu, Kinyarwanda, KurdishNorthern, Latgalian, Latin, Latvian, Ligurian, Limburgish, Lingala, Lithuanian, Lombard, LubaKasai, Luo, Luxembourgish, Malay, Maltese, Minangkabau, Mizo, Mossi, Nuer, Nyanja, Nynorsk, Occitan, OromoWestCentral, Papiamento, Polish, Portuguese, Romanian, Rundi, Sango, Sardinian, Sepedi, Sesotho, Shona, Sicilian, Silesian, Slovak, Slovene, Somali, Spanish, Sundanese, Swahili, Swati, Swedish, Tagalog, Tamasheq, TatarCrimean, TokPisin, Tsonga, Tswana, Tumbuka, Turkish, Turkmen, Twi, Umbundu, UzbekNorthern, Venetian, Vietnamese, Waray, Welsh, Wolof, Xhosa, Yoruba, Zulu)),
    case(Estonian, "päralt", ahashset!(Afrikaans, AymaraCentral, DinkaSouthwestern, Dutch, Estonian, Finnish, German, Limburgish, Luxembourgish, Slovak, Swedish, Turkmen)),
    case(Estonian, "tõeliseks", ahashset!(Estonian, Guarani, Kabuverdianu, Portuguese, Vietnamese)),
    case(French, "contrôle", ahashset!(Acehnese, Afrikaans, Bambara, French, Friulian, Limburgish, Portuguese, Sepedi, Slovak, Vietnamese, Welsh)),
    case(French, "façonnage", ahashset!(AlbanianTosk, AzerbaijaniNorth, Bokmal, Bosnian, Catalan, French, Friulian, Kabuverdianu, KurdishNorthern, Ligurian, Occitan, Portuguese, TatarCrimean, Turkish, Turkmen, Venetian)),
    case(French, "forêt", ahashset!(Afrikaans, French, Friulian, Kabuverdianu, KurdishNorthern, Portuguese, Sepedi, Welsh)),
    case(French, "où", ahashset!(Czech, Fon, French, Friulian, GaelicScottish, Italian, Limburgish, Lombard, Sardinian, Sicilian, Venetian, Vietnamese, Yoruba)),
    case(French, "succèdent", ahashset!(Acehnese, Afrikaans, Catalan, CreoleHaitian, Czech, Dutch, Fon, French, Friulian, GaelicScottish, Italian, Limburgish, Lombard, Occitan, Papiamento, Sardinian, Sicilian, Venetian, Vietnamese)),
    case(German, "höher", ahashset!(Acehnese, Afrikaans, AymaraCentral, AzerbaijaniNorth, DinkaSouthwestern, Dutch, Estonian, Finnish, German, Hungarian, Icelandic, Limburgish, Luxembourgish, Swedish, TatarCrimean, Turkish, Turkmen)),
    case(German, "überrascht", ahashset!(Afrikaans, Asturian, AymaraCentral, AzerbaijaniNorth, Catalan, Dutch, French, German, Hungarian, Limburgish, Luxembourgish, Papiamento, Portuguese, Spanish, TatarCrimean, Turkish)),
    case(Hebrew, "בתחרויות", ahashset!(YiddishEastern, Yiddish, Hebrew)),
    case(Icelandic, "minjaverðir", ahashset!(Faroese, Icelandic)),
    case(Italian, "venerdì", ahashset!(Czech, Fon, Friulian, Italian, Limburgish, Lombard, Sardinian, Sicilian, Venetian, Vietnamese)),
    case(Kazakh, "әлем", ahashset!(Bashkir, Kazakh, Tatar)),
    case(Kazakh, "оның", ahashset!(Bashkir, Kazakh, Kyrgyz, Tatar)),
    case(Kazakh, "шаруашылығы", ahashset!(Bashkir, Kazakh)),
    case(Latvian, "aizklātā", ahashset!(Latgalian, Latin, Latvian)),
    case(Latvian, "blaķene", ahashset!(Latgalian, Latvian)),
    case(Latvian, "ceļojumiem", ahashset!(Latgalian, Latvian)),
    case(Latvian, "labāk", ahashset!(Latgalian, Latin, Latvian)),
    case(Latvian, "nebūtu", ahashset!(Latgalian, Latin, Latvian, Lithuanian)),
    case(Latvian, "numuriņu", ahashset!(Latgalian, Latvian)),
    case(Latvian, "palīdzi", ahashset!(Latgalian, Latin, Latvian)),
    case(Latvian, "sistēmas", ahashset!(Latgalian, Latin, Latvian)),
    case(Latvian, "teoloģiska", ahashset!(Latgalian, Latvian)),
    case(Latvian, "viņiem", ahashset!(Latgalian, Latvian)),
    case(Lithuanian, "nebūsime", ahashset!(Latgalian, Latin, Latvian, Lithuanian)),
    case(Macedonian, "затоплување", ahashset!(Macedonian, Serbian)),
    case(Macedonian, "ѕидови", ahashset!(Macedonian, OldChurchSlavonic)),
    case(Macedonian, "набљудувач", ahashset!(Macedonian, Serbian)),
    case(Macedonian, "ректасцензија", ahashset!(Macedonian, Serbian)),
    case(Macedonian, "џамиите", ahashset!(Macedonian, Serbian)),
    case(Marathi, "मिळते", ahashset!(Awadhi, Bhojpuri, Chhattisgarhi, Hindi, Kashmiri, Magahi, Maithili, Marathi, Nepali, Sanskrit)),
    case(MongolianHalh, "дөхөж", ahashset!(Bashkir, Kazakh, Kyrgyz, MongolianHalh, Tatar)),
    case(MongolianHalh, "үндсэн", ahashset!(Bashkir, Kazakh, Kyrgyz, MongolianHalh, Tatar)),
    case(Polish, "kradzieżami", ahashset!(Maltese, Polish)),
    case(Polish, "zmieniły", ahashset!(Polish, Silesian)),
    case(Portuguese, "catedráticos", ahashset!(Afrikaans, Asturian, AymaraCentral, Czech, Dutch, Fon, Galician, Guarani, Hungarian, Igbo, Irish, Kabuverdianu, Limburgish, Lombard, Occitan, Papiamento, Portuguese, Shona, Slovak, Spanish, Vietnamese)),
    case(Portuguese, "música", ahashset!(Afrikaans, Asturian, AymaraCentral, Catalan, Czech, Fon, Galician, Guarani, Hungarian, Igbo, Irish, Kabuverdianu, Kikuyu, Kinyarwanda, Limburgish, Lombard, Occitan, Papiamento, Portuguese, Sardinian, Shona, Slovak, Spanish, Vietnamese)),
    case(Portuguese, "política", ahashset!(Afrikaans, Asturian, AymaraCentral, Catalan, Czech, Fon, Galician, Guarani, Hungarian, Igbo, Irish, Kabuverdianu, Kikuyu, Kinyarwanda, Limburgish, Lombard, Occitan, Portuguese, Sardinian, Shona, Slovak, Spanish, Vietnamese)),
    case(Portuguese, "visão", ahashset!(Ewe, Guarani, Portuguese, Vietnamese)),
    case(Romanian, "afişate", ahashset!(AzerbaijaniNorth, KurdishNorthern, Romanian, TatarCrimean, Turkish)),
    case(Romanian, "înviat", ahashset!(Afrikaans, French, Friulian, Guarani, KurdishNorthern, Limburgish, Romanian)),
    case(Romanian, "pregătire", ahashset!(Romanian, Vietnamese)),
    case(Russian, "огнём", ahashset!(Bashkir, Belarusian, Kazakh, Kyrgyz, MongolianHalh, Russian, Tajik, Tatar)),
    case(Russian, "сопротивление", ahashset!(Bashkir, Bulgarian, Kazakh, Kyrgyz, Macedonian, MongolianHalh, Russian, Serbian, Tajik, Tatar, Ukrainian)),
    case(Russian, "этот", ahashset!(Bashkir, Belarusian, Kazakh, Kyrgyz, MongolianHalh, Russian, Tajik, Tatar)),
    case(Slovak, "rozohňuje", ahashset!(Czech, Silesian, Slovak)),
    case(Spanish, "años", ahashset!(Asturian, AymaraCentral, Basque, Galician, Guarani, Ilocano, Papiamento, Spanish, Waray, Wolof)),
    case(Ukrainian, "пристрої", ahashset!(OldChurchSlavonic, Ukrainian)),
    case(Vietnamese, "chọn", ahashset!(Igbo, Vietnamese)),
    case(Vietnamese, "cũng", ahashset!(Guarani, Vietnamese)),
    case(Vietnamese, "dụng", ahashset!(Igbo, Vietnamese)),
    case(Vietnamese, "kẽm", ahashset!(Ewe, Guarani, Vietnamese)),
    case(Vietnamese, "mỹ", ahashset!(Guarani, Vietnamese)),
    case(Vietnamese, "nhẹn", ahashset!(Vietnamese, Yoruba)),
    case(Vietnamese, "ravị", ahashset!(Igbo, Vietnamese)),
    case(Vietnamese, "trĩ", ahashset!(Ewe, Guarani, Vietnamese)),
)]
fn test_word_multiple_langs(
    expected_language: Language,
    word: &str,
    expected_languages: AHashSet<Language>,
) {
    let found_words: Vec<_> = alphabet_detector::from_ch_iter(word.char_indices()).collect();
    if found_words.len() > 1 {
        panic!("Not a word: {} got: {:?}", word, found_words);
    }
    let languages = langs_count_max(found_words[0].langs_cnt).0;

    assert!(
        languages.contains(&expected_language),
        "expected {:?} for word '{}', got {:?}",
        expected_language,
        word,
        languages
    );
    assert_eq!(
        languages, expected_languages,
        "expected {:?} for word '{}', got {:?}",
        expected_languages, word, languages
    );
}
