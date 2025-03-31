mod shared;
#[allow(unused_imports)]
use shared::*;

use ahash::AHashSet;
use alphabet_detector::{Language::*, *};
use rstest::*;

#[rstest(expected_language, text, expected_languages,
    case(ChineseTraditional, "也有越來越多的人開始飼養寵物", ahashset!(ChineseCantoneseTraditional, ChineseTraditional, Japanese)),
    case(Japanese, "昨日、東京で大切な友達に会いました。", ahashset!(Japanese)), // Kanji (Han) + Hiragana
)]
fn test_fulltext_langs_max(
    expected_language: Language,
    text: &str,
    expected_languages: AHashSet<Language>,
) {
    let languages: AHashSet<_> = fulltext_langs_max::<bool>(text.char_indices()).1.collect();

    assert!(
        languages.contains(&expected_language),
        "{:?} text '{}', got {:?}",
        expected_language,
        text,
        languages
    );
    assert_eq!(
        languages, expected_languages,
        "{:?} text '{}'",
        expected_language, text
    );
}
