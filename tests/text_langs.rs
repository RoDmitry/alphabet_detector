mod shared;
#[allow(unused_imports)]
use shared::*;

use ahash::AHashSet;
use alphabet_detector::Language::*;
use alphabet_detector::*;
use rstest::*;

#[rstest(expected_language, text, expected_languages,
    case(ChineseSimplified, "也有越來越多的人開始飼養寵物", ahashset!(ChineseCantoneseTraditional, ChineseSimplified, ChineseTraditional, Japanese, Korean)),
    case(Japanese, "昨日、東京で大切な友達に会いました。", ahashset!(Japanese)), // Kanji (Han) + Hiragana
)]
fn test_text(expected_language: Language, text: &str, expected_languages: AHashSet<Language>) {
    let languages = fulltext_langs(text.char_indices()).1;

    assert!(
        languages.contains(&expected_language),
        "text '{}', expected {:?}, got {:?}",
        text,
        expected_language,
        languages
    );
    assert_eq!(
        languages, expected_languages,
        "text '{}', expected {:?}, got {:?}",
        text, expected_languages, languages
    );
}
