#![cfg_attr(feature = "files_read", feature(string_into_chars))]

pub mod ch_norm_iter;
mod fulltext;
mod isocode;
mod lang;
mod langs_filter;
#[cfg(feature = "files_read")]
pub mod read_iter;
pub mod word_iter;

pub use ch_norm_iter::{CharData, CharNormalizingIterator};
pub use fulltext::*;
pub use isocode::{IsoCode639_1, IsoCode639_3};
pub use lang::*;
pub use langs_filter::*;
pub use word_iter::{WordIterator, WordLangsData};

#[cfg(feature = "files_read")]
pub fn str_to_langs(s: &str) -> &[ScriptLanguage] {
    match s {
        "Arab" => script_char_to_slangs(Script::Arabic, char::default()),
        "Armn" => script_char_to_slangs(Script::Armenian, char::default()),
        "Beng" => script_char_to_slangs(Script::Bengali, char::default()),
        "Cyrl" => script_char_to_slangs(Script::Cyrillic, char::default()),
        "Deva" => script_char_to_slangs(Script::Devanagari, char::default()),
        "Ethi" => script_char_to_slangs(Script::Ethiopic, char::default()),
        "Geor" => script_char_to_slangs(Script::Georgian, char::default()),
        "Grek" => script_char_to_slangs(Script::Greek, char::default()),
        "Gujr" => script_char_to_slangs(Script::Gujarati, char::default()),
        "Guru" => script_char_to_slangs(Script::Gurmukhi, char::default()),
        "Hans" | "Hant" | "Jpan" | "Hang" => script_char_to_slangs(Script::Han, char::default()),
        "Hebr" => script_char_to_slangs(Script::Hebrew, char::default()),
        "Khmr" => script_char_to_slangs(Script::Khmer, char::default()),
        "Knda" => script_char_to_slangs(Script::Kannada, char::default()),
        "Laoo" => script_char_to_slangs(Script::Lao, char::default()),
        "Latn" => script_char_to_slangs(Script::Latin, char::default()),
        "Mlym" => script_char_to_slangs(Script::Malayalam, char::default()),
        "Mymr" => script_char_to_slangs(Script::Myanmar, char::default()),
        "Olck" => script_char_to_slangs(Script::OlChiki, char::default()),
        "Orya" => script_char_to_slangs(Script::Oriya, char::default()),
        "Sinh" => script_char_to_slangs(Script::Sinhala, char::default()),
        "Taml" => script_char_to_slangs(Script::Tamil, char::default()),
        "Telu" => script_char_to_slangs(Script::Telugu, char::default()),
        "Tfng" => script_char_to_slangs(Script::Tifinagh, char::default()),
        "Thai" => script_char_to_slangs(Script::Thai, char::default()),
        "Tibt" => script_char_to_slangs(Script::Tibetan, char::default()),
        _ => unreachable!(),
    }
}
