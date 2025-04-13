mod shared;
#[allow(unused_imports)]
use shared::*;

use ahash::AHashSet;
use alphabet_detector::{ScriptLanguage::*, *};
use rstest::*;

#[rstest(expected_language, text, expected_languages,
    case(ChineseTraditional, "也有越來越多的人開始飼養寵物", ahashset!(ChineseCantoneseTraditional, ChineseTraditional, Japanese)),
    case(Japanese, "昨日、東京で大切な友達に会いました。", ahashset!(Japanese)), // Kanji (Han) + Hiragana
    case::simplified_chinese(ChineseSimplified, "经济", ahashset![ChineseSimplified]),
    case::traditional_chinese(ChineseTraditional, "經濟", ahashset![ChineseTraditional, ChineseCantoneseTraditional, Japanese]),
    case::kanji(Japanese, "経済", ahashset![Korean, Japanese]),
    case::kanji2(Japanese, "自動販売機", ahashset![Japanese]),
)]
fn test_fulltext_filter_max(
    expected_language: ScriptLanguage,
    text: &str,
    expected_languages: AHashSet<ScriptLanguage>,
) {
    let languages: AHashSet<_> = fulltext_filter_max::<bool>(text.char_indices()).1.collect();

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
