mod shared;
#[allow(unused_imports)]
use shared::*;

use ::core::ops::Range;
use ahash::AHashSet;
use alphabet_detector::*;
use rstest::*;

#[rstest(text, expected_words,
    case("a", ahashset!("a")),
    case("word", ahashset!("word")),
    case("worda wordb", ahashset!("worda", "wordb")),
    case("word'", ahashset!("word")),
    case("'word", ahashset!("word")),
    case("'word'", ahashset!("word")),
    case("''word''", ahashset!("word")),
    case("can't", ahashset!("can't")),
    case("can'фыв", ahashset!("can", "фыв")),
    case("worda' wordb", ahashset!("worda", "wordb")),
    case("worda 'wordb", ahashset!("worda", "wordb")),
    case("'worda', 'wordb'", ahashset!("worda", "wordb")),
    case("¿Que?", ahashset!("¿que")),
    case("¿-", ahashset!()),
    case(" ¿-", ahashset!()),
    case("a-b", ahashset!("a-b")),
    case("a- asd", ahashset!("a", "asd")),
    case("asd-фыв", ahashset!("asd", "фыв")),
    case("3-rd", ahashset!("rd")),
    case("3ʳᵈ", ahashset!("ʳᵈ")),
    case("ПроSto", ahashset!("про", "sto")),
    case::chinese("中文", ahashset!("中文")),
    case("worda 🙈", ahashset!("worda")),
    case::kanji("昨日、東京で大切な友達に会いました。", ahashset!("昨日", "東京で大切な友達に会いました")),
    case("this is a sentence", ahashset!("this", "is", "a", "sentence")),
    case("I can't do this", ahashset!("i", "can't", "do", "this")),
    case(
        "上海大学是一个好大学 this is a sentence",
        ahashset!("上海大学是一个好大学", "this", "is", "a", "sentence")
    ),
    case(
        "Weltweit    gibt es ungefähr 6.000 Sprachen.",
        ahashset!("weltweit", "gibt", "es", "ungefähr", "sprachen")
    ),
    case("This,is ok", ahashset!("this", "is", "ok")),
    case::chinese("中,文", ahashset!("中", "文")),
    case::chinese("和little", ahashset!("和", "little")),
    case(
        "Thi̇s is one word", // This = THİS with lowered İ
        ahashset!("this", "is", "one", "word")
    ),
    case("Spanish Ñ two chars", ahashset!("spanish", "ñ", "two", "chars")),
    case("Spanish lowered ñ two chars", ahashset!("spanish", "lowered", "ñ", "two", "chars")),
    case::combine("ầ Ɛ̌", ahashset!("ầ", "\u{f0c5b}")),
    // case::deva("हूँ", ahashset!("हूँ")),
    // case::hangul("ㄹ語幹に付く態転換接尾辞に", ahashset!("ㄹ", "語幹に付く態転換接尾辞に")),
    // case::hangul2("ㅈ語幹用言に付く場合には", ahashset!("ㅈ", "語幹用言に付く場合には")),
    // case::hangul3("現代朝鮮語にも存在する上昇二重母音ㅑ", ahashset!("現代朝鮮語にも存在する上昇二重母音", "ㅑ")),
)]
fn test_text_to_words(text: &str, expected_words: AHashSet<&str>) {
    let found_words: Vec<_> = words::from_ch_ind::<String>(text.char_indices())
        .map(|wld| wld.buf)
        .collect();
    let words: AHashSet<&str> = found_words.iter().map(|w| w.as_str()).collect();

    assert_eq!(words, expected_words, "text: {}", text);
}

#[rstest(word, expected_range,
    case("¿Que?", 0..5),
    case("'word'", 1..5),
    case("aﬁre", 0..6),
    case("oﬃce", 0..6),
)]
fn test_word_range(word: &str, expected_range: Range<usize>) {
    let found_words: Vec<_> = words::from_ch_ind::<String>(word.char_indices()).collect();
    if found_words.len() > 1 {
        panic!("Not a word: {} got: {:?}", word, found_words);
    }
    let range = found_words[0].range.clone();

    assert_eq!(range, expected_range, "word '{}'", word);
}

#[rstest(text, expected_ranges,
    case("ЧтоWhat", vec![0..6, 6..10]),
    case("¿ОнаShe", vec![0..2, 2..8, 8..11]),
    case("Привет John", vec![0..12, 13..17]),
)]
fn test_text_ranges(text: &str, expected_ranges: Vec<Range<usize>>) {
    let found_words: Vec<_> = words::from_ch_ind::<String>(text.char_indices()).collect();
    assert_eq!(
        found_words.len(),
        expected_ranges.len(),
        "invalid word count"
    );

    for (i, wd) in found_words.into_iter().enumerate() {
        let expected_range = expected_ranges.get(i).unwrap().clone();

        assert_eq!(wd.range, expected_range, "word '{}'", wd.buf);
    }
}
