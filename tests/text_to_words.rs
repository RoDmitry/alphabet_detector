mod shared;
#[allow(unused_imports)]
use shared::*;

use ahash::AHashSet;
use alphabet_detector::*;
use rstest::*;

#[rstest(text, expected_words,
    case("word", ahashset!("word")),
    case("worda wordb", ahashset!("worda", "wordb")),
    case("word'", ahashset!("word")),
    case("'word", ahashset!("word")),
    case("'word'", ahashset!("word")),
    case("''word''", ahashset!("word")),
    case("can't", ahashset!("can't")),
    case("worda' wordb", ahashset!("worda", "wordb")),
    case("worda 'wordb", ahashset!("worda", "wordb")),
    case("'worda', 'wordb'", ahashset!("worda", "wordb")),
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
    case::combine("ầ M̄", ahashset!("ầ", "\u{f046d}")),
    // case::deva("हूँ", ahashset!("हूँ")),
    // case::hangul("ㄹ語幹に付く態転換接尾辞に", ahashset!("ㄹ", "語幹に付く態転換接尾辞に")),
    // case::hangul2("ㅈ語幹用言に付く場合には", ahashset!("ㅈ", "語幹用言に付く場合には")),
    // case::hangul3("現代朝鮮語にも存在する上昇二重母音ㅑ", ahashset!("現代朝鮮語にも存在する上昇二重母音", "ㅑ")),
)]
fn test_text_to_words(text: &str, expected_words: AHashSet<&str>) {
    let found_words: Vec<_> = from_ch_iter(text.char_indices())
        .map(|wd| wd.chars.into_iter().collect::<String>())
        .collect();
    let words: AHashSet<&str> = found_words.iter().map(|w| w.as_str()).collect();

    assert_eq!(words, expected_words, "text: {}", text);
}
