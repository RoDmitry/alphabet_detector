#![cfg_attr(feature = "files_read", feature(string_into_chars))]

pub mod ch_norm_iter;
mod fulltext;
mod isocode;
mod lang;
mod langs_filter;
#[cfg(feature = "files_read")]
pub mod read_iter;
pub mod word_iter;

pub use ch_norm_iter::CharNormalizingIterator;
pub use fulltext::*;
pub use isocode::{IsoCode639_1, IsoCode639_3};
pub use lang::*;
pub use langs_filter::*;
pub use word_iter::{WordLangsData, WordIterator};

#[cfg(feature = "files_read")]
pub fn str_to_langs(s: &str) -> &[Language] {
    match s {
        "Arab" => script_char_to_langs(Script::Arabic, char::default()),
        "Armn" => script_char_to_langs(Script::Armenian, char::default()),
        "Beng" => script_char_to_langs(Script::Bengali, char::default()),
        "Cyrl" => script_char_to_langs(Script::Cyrillic, char::default()),
        "Deva" => script_char_to_langs(Script::Devanagari, char::default()),
        "Ethi" => script_char_to_langs(Script::Ethiopic, char::default()),
        "Geor" => script_char_to_langs(Script::Georgian, char::default()),
        "Grek" => script_char_to_langs(Script::Greek, char::default()),
        "Gujr" => script_char_to_langs(Script::Gujarati, char::default()),
        "Guru" => script_char_to_langs(Script::Gurmukhi, char::default()),
        "Hang" => script_char_to_langs(Script::Hangul, char::default()),
        "Hans" | "Hant" | "Jpan" => script_char_to_langs(Script::Han, char::default()),
        "Hebr" => script_char_to_langs(Script::Hebrew, char::default()),
        "Khmr" => script_char_to_langs(Script::Khmer, char::default()),
        "Knda" => script_char_to_langs(Script::Kannada, char::default()),
        "Laoo" => script_char_to_langs(Script::Lao, char::default()),
        "Latn" => script_char_to_langs(Script::Latin, char::default()),
        "Mlym" => script_char_to_langs(Script::Malayalam, char::default()),
        "Mymr" => script_char_to_langs(Script::Myanmar, char::default()),
        "Olck" => script_char_to_langs(Script::OlChiki, char::default()),
        "Orya" => script_char_to_langs(Script::Oriya, char::default()),
        "Sinh" => script_char_to_langs(Script::Sinhala, char::default()),
        "Taml" => script_char_to_langs(Script::Tamil, char::default()),
        "Telu" => script_char_to_langs(Script::Telugu, char::default()),
        "Tfng" => script_char_to_langs(Script::Tifinagh, char::default()),
        "Thai" => script_char_to_langs(Script::Thai, char::default()),
        "Tibt" => script_char_to_langs(Script::Tibetan, char::default()),
        _ => unreachable!(),
    }
}
