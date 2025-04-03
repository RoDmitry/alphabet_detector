mod shared;
#[allow(unused_imports)]
use shared::*;

use ahash::AHashSet;
use alphabet_detector::{ScriptLanguage::*, *};
use rstest::*;

#[rstest(expected_language, word,
    case(Czech, "tvořen"),
    case(Esperanto, "intermiksiĝis"),
    case(Esperanto, "kreitaĵoj"),
    case(Esperanto, "monaĥinoj"),
    case(Esperanto, "nesufiĉecon"),
    case(Esperanto, "ŝpinante"),
    case(German, "groß"),
    case(Hungarian, "fekvő"),
    case(Hungarian, "meggyűrűzni"),
    case(Icelandic, "þagnarskyldu"),
    case(Kazakh, "ақын"),
    case(Kazakh, "шұрайлы"),
    case(Macedonian, "ќерка"),
    case(Macedonian, "припаѓа"),
    case(Polish, "wystąpią"),
    case(Romanian, "ialomiţa"),
    case(Serbian, "наслеђивања"),
    case(Serbian, "неисквареношћу"),
    case(Slovak, "mŕtvych"),
    case(Slovak, "podĺa"),
    case(Slovak, "pohľade"),
    case(Ukrainian, "ґрунтовому"),
    case(Ukrainian, "пропонує"),
    case(Vietnamese, "biểu"),
    case(Vietnamese, "bỡi"),
    case(Vietnamese, "cằm"),
    case(Vietnamese, "chẳng"),
    case(Vietnamese, "chất"),
    case(Vietnamese, "chức"),
    case(Vietnamese, "đạp"),
    case(Vietnamese, "dắt"),
    case(Vietnamese, "diễm"),
    case(Vietnamese, "giới"),
    case(Vietnamese, "hậu"),
    case(Vietnamese, "hiền"),
    case(Vietnamese, "hợp"),
    case(Vietnamese, "hưng"),
    case(Vietnamese, "kỳ"),
    case(Vietnamese, "kỷ"),
    case(Vietnamese, "mặn"),
    case(Vietnamese, "mỗi"),
    case(Vietnamese, "một"),
    case(Vietnamese, "mỵ"),
    case(Vietnamese, "nguồn"),
    case(Vietnamese, "nhẫn"),
    case(Vietnamese, "nhở"),
    case(Vietnamese, "những"),
    case(Vietnamese, "phế"),
    case(Vietnamese, "quẩy"),
    case(Vietnamese, "sẵn"),
    case(Vietnamese, "sử"),
    case(Vietnamese, "thần"),
    case(Vietnamese, "thơ"),
    case(Vietnamese, "thờ"),
    case(Vietnamese, "thực"),
    case(Vietnamese, "tổng"),
    case(Vietnamese, "tốt"),
    case(Vietnamese, "từng"),
    case(Vietnamese, "việc"),
    case(Yoruba, "aṣiwèrè"),

    // unique script
    case(Armenian, "ունենա"),
    case(Georgian, "გარეუბან"),
    case(Greek, "σταμάτησε"),
    case(Greek, "σχέδια"),
    case(Gujarati, "ઉપકરણોની"),
    case(Japanese, "ヴェダイヤモンド"),
    case(Japanese, "びさ"),
    case(Korean, "대결구도가"),
    case(PunjabiEastern, "ਮੋਟਰਸਾਈਕਲਾਂ"),
    case(Tamil, "துன்பங்களை"),
    case(Telugu, "కృష్ణదేవరాయలు"),
    case(Thai, "ในทางหลวงหมายเลข"),
)]
fn test_word_uniq(expected_language: ScriptLanguage, word: &str) {
    let found_words: Vec<_> = word_iter::from_ch_iter::<String>(word.char_indices()).collect();
    if found_words.len() > 1 {
        panic!("Not a word: {} got: {:?}", word, found_words);
    }
    let languages: AHashSet<_> = langs_filter_max(found_words[0].langs_cnt).0.collect();

    assert!(
        languages.len() == 1,
        "{:?} word '{}', got {:?}",
        expected_language,
        word,
        languages
    );

    let language = languages.iter().next().copied().unwrap();
    assert_eq!(
        language, expected_language,
        "word '{}', got {:?}",
        word, languages
    );
}

#[rstest(
    expected_language,
    word,
    case::eng_like(Slovak, "šefčovič's"),
    case(AlbanianTosk, "hashemidëve"),
    case(Arabic, "والموضوع"),
    case(AzerbaijaniNorth, "məhərrəm"),
    case(Belarusian, "павінен"),
    case(Belarusian, "раскрывае"),
    case(Bengali, "জানাতে"),
    case(Bulgarian, "довършат"),
    case(Bulgarian, "плаваща"),
    case(Catalan, "contradicció"),
    case(Catalan, "només"),
    case(Catalan, "pràctiques"),
    case(Catalan, "substituïts"),
    case(Croatian, "nađete"),
    case(Croatian, "prihvaćanju"),
    case(Czech, "jeďte"),
    case(Czech, "navržen"),
    case(Czech, "rozdělit"),
    case(Czech, "rtuť"),
    case(Czech, "subjektů"),
    case(Czech, "vývoj"),
    case(Czech, "zaručen"),
    case(Czech, "zkouškou"),
    case(Danish, "direktør"),
    case(Danish, "indebærer"),
    case(Danish, "måned"),
    case(English, "house"),
    case(Esperanto, "apenaŭ"),
    case(Estonian, "päralt"),
    case(Estonian, "tõeliseks"),
    case(French, "contrôle"),
    case(French, "façonnage"),
    case(French, "forêt"),
    case(French, "où"),
    case(French, "succèdent"),
    case(German, "höher"),
    case(German, "überrascht"),
    case(Hawaiian, "pu'u'ō'ō"),
    case(Hebrew, "בתחרויות"),
    case(Icelandic, "minjaverðir"),
    case(Italian, "venerdì"),
    case(Kazakh, "әлем"),
    case(Kazakh, "оның"),
    case(Kazakh, "шаруашылығы"),
    case(Latvian, "aizklātā"),
    case(Latvian, "blaķene"),
    case(Latvian, "ceļojumiem"),
    case(Latvian, "labāk"),
    case(Latvian, "nebūtu"),
    case(Latvian, "numuriņu"),
    case(Latvian, "palīdzi"),
    case(Latvian, "sistēmas"),
    case(Latvian, "teoloģiska"),
    case(Latvian, "viņiem"),
    case(Lithuanian, "įrengus"),
    case(Lithuanian, "mergelės"),
    case(Lithuanian, "nebūsime"),
    case(Lithuanian, "slegiamų"),
    case(Macedonian, "затоплување"),
    case(Macedonian, "ѕидови"),
    case(Macedonian, "набљудувач"),
    case(Macedonian, "ректасцензија"),
    case(Macedonian, "џамиите"),
    case(Marathi, "मिळते"),
    case(MongolianHalh, "дөхөж"),
    case(MongolianHalh, "үндсэн"),
    case(Polish, "budowę"),
    case(Polish, "groźne"),
    case(Polish, "kradzieżami"),
    case(Polish, "mniejszości"),
    case(Polish, "państwowych"),
    case(Polish, "zmieniły"),
    case(Portuguese, "catedráticos"),
    case(Portuguese, "música"),
    case(Portuguese, "política"),
    case(Portuguese, "visão"),
    case(Romanian, "afişate"),
    case(Romanian, "înviat"),
    case(Romanian, "pregătire"),
    case(Russian, "огнём"),
    case(Russian, "сопротивление"),
    case(Russian, "этот"),
    case(Slovak, "rozohňuje"),
    case(Spanish, "¿que?"),
    case(Spanish, "años"),
    case(Ukrainian, "пристрої"),
    case(Vietnamese, "chỉnh"),
    case(Vietnamese, "chọn"),
    case(Vietnamese, "của"),
    case(Vietnamese, "cũng"),
    case(Vietnamese, "dụng"),
    case(Vietnamese, "kẽm"),
    case(Vietnamese, "lẻn"),
    case(Vietnamese, "mỹ"),
    case(Vietnamese, "nhẹn"),
    case(Vietnamese, "ravị"),
    case(Vietnamese, "sỏi"),
    case(Vietnamese, "trĩ"),
    case(Yoruba, "ṣaaju")
)]
fn test_word_multiple_langs(expected_language: ScriptLanguage, word: &str) {
    let found_words: Vec<_> = word_iter::from_ch_iter::<String>(word.char_indices()).collect();
    if found_words.len() > 1 {
        panic!("Not a word '{}' got {:?}", word, found_words);
    }
    let languages: AHashSet<_> = langs_filter_max(found_words[0].langs_cnt).0.collect();
    if languages.len() == 1 {
        panic!("Unique word '{}'", word);
    }

    assert!(
        languages.contains(&expected_language),
        "{:?} word '{}', got {:?}",
        expected_language,
        word,
        languages
    );
}
