mod shared;
#[allow(unused_imports)]
use shared::*;

use alphabet_detector::*;
use rstest::*;

#[rstest(text, expected_chars,
    case("word", vec!('w','o','r','d')),
    case("'word'", vec!('\'','w','o','r','d','\'')),
    case::combine("ÑầM̄", vec!('Ñ','ầ','\u{f046d}')),
    case("ﬁ", vec!('f','i')),
    case("ﬃ", vec!('f','f','i')),
    case("ﬁre", vec!('f','i','r','e')),
    case("aﬁre", vec!('a','f','i','r','e')),
    case("aﬁ", vec!('a','f','i')),
    case("ﬃx", vec!('f','f','i','x')),
    case("oﬃce", vec!('o','f','f','i','c','e')),
    case("aﬃ", vec!('a','f','f','i')),
)]
fn test_ch_norm_iter_chars(text: &str, expected_chars: Vec<char>) {
    let chars: Vec<_> = ch_norm_iter::from_ch_iter(text.char_indices())
        .map(|data| data.2)
        .collect();

    assert_eq!(chars, expected_chars, "text: {}", text);
}
