mod shared;
#[allow(unused_imports)]
use shared::*;

use alphabet_detector::*;
use rstest::*;

#[rstest(text, expected_chars,
    case("word", vec!['w','o','r','d']),
    case("'word'", vec!['\'','w','o','r','d','\'']),
    case::combine("ÑầƐ̌", vec!['Ñ','ầ','\u{f0c5b}']),
    case::arabic_8e1_correct("\u{0628}\u{064E}\u{0651}", vec!['\u{0628}', '\u{064E}', '\u{0651}']),
    case::arabic_81e("\u{0628}\u{0651}\u{064E}", vec!['\u{0628}', '\u{064E}', '\u{0651}']),
    case::arabic_801_3_correct("\u{0628}\u{0650}\u{0651}\u{06E3}", vec!['\u{0628}', '\u{0650}', '\u{0651}', '\u{06E3}']),
    case::arabic_810_3("\u{0628}\u{0651}\u{0650}\u{06E3}", vec!['\u{0628}', '\u{0650}', '\u{0651}', '\u{06E3}']),
    case::arabic_81_30("\u{0628}\u{0651}\u{06E3}\u{0650}", vec!['\u{0628}', '\u{0650}', '\u{0651}', '\u{06E3}']),
    case("ﬁ", vec!['f','i']),
    case("ﬃ", vec!['f','f','i']),
    case("ﬁre", vec!['f','i','r','e']),
    case("aﬁre", vec!['a','f','i','r','e']),
    case("aﬁ", vec!['a','f','i']),
    case("ﬃx", vec!['f','f','i','x']),
    case("oﬃce", vec!['o','f','f','i','c','e']),
    case("aﬃ", vec!['a','f','f','i']),
    case::combine2("oﬃ́ce", vec!['o','f','f','í','c','e']),
    case::inherited_no_ccc("a\u{fe0f}b\u{fe0e}", vec!['a','b']),
    case::inherited_no_ccc2("a\u{fe0f}b\u{fe0e}c", vec!['a','b', 'c']),
    case::inherited_no_ccc3("á\u{fe0f}ó\u{fe0e}", vec!['á', 'ó']),
    case::inherited_no_ccc4("á\u{fe0f}ó\u{fe0e}c", vec!['á', 'ó', 'c']),
    case::lao("\u{e82}\u{ec8}\u{ec9}", vec!['\u{e82}', '\u{ec8}', '\u{ec9}']),
    case::lao2("\u{ec8}\u{ec9}", vec!['\u{ec8}', '\u{ec9}']),
    case::lao3("\u{e82}\u{ec8}\u{ec9}\u{e81}", vec!['\u{e82}', '\u{ec8}', '\u{ec9}', '\u{e81}']),
    case::lao4("\u{e82}\u{ec8}\u{301}\u{ec9}\u{e81}", vec!['\u{e82}', '\u{ec8}', '\u{ec9}', '\u{301}', '\u{e81}']),
    case::lao4("\u{e82}\u{ec8}\u{ec9}\u{e81}\u{fe0f}", vec!['\u{e82}', '\u{ec8}', '\u{ec9}', '\u{e81}']),
)]
fn test_ch_norm_iter_chars(text: &str, expected_chars: Vec<char>) {
    let chars: Vec<_> = ch_norm::from_ch_ind(text.char_indices())
        .map(|data| data.ch)
        .collect();

    assert_eq!(chars, expected_chars, "text: {}", text);
}
