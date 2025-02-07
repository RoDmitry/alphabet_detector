mod shared;
#[allow(unused_imports)]
use shared::*;

use alphabet_detector::*;
use rstest::*;

#[rstest(text, expected_chars,
    case("word", vec!('w','o','r','d','\0')),
    case("'word'", vec!('\'','w','o','r','d','\'','\0')),
    case::combine("ÑầM̄", vec!('Ñ','ầ','\u{f046d}','\0')),
    case("ﬁ", vec!('f','i','\0')),
    case("ﬃ", vec!('f','f','i','\0')),
    case("ﬁre", vec!('f','i','r','e','\0')),
    case("aﬁre", vec!('a','f','i','r','e','\0')),
    case("aﬁ", vec!('a','f','i','\0')),
    case("ﬃx", vec!('f','f','i','x','\0')),
    case("oﬃce", vec!('o','f','f','i','c','e','\0')),
    case("aﬃ", vec!('a','f','f','i','\0')),
)]
fn test_ch_norm_iter_chars(text: &str, expected_chars: Vec<char>) {
    let chars: Vec<_> = ch_norm_iter::from_ch_iter(text.char_indices())
        .map(|data| data.2)
        .collect();

    assert_eq!(chars, expected_chars, "text: {}", text);
}
