use super::{Script, ScriptLanguage};
#[cfg(all(debug_assertions, feature = "test_chars"))]
use crate::ch_norm::test_chars;
use alphabet_detector_macros::alphabet_match;

pub(crate) fn char_compose_custom(ch: char, mark: char) -> char {
    match mark {
        '\u{300}' | '\u{340}' => match ch {
            'ẹ' | 'Ẹ' => '\u{f00b9}', // Ẹ̀ẹ̀
            'm' | 'M' => '\u{f006d}', // M̀m̀
            'ọ' | 'Ọ' => '\u{f00cd}', // Ọ̀ọ̀
            _ => ch,
        },
        '\u{301}' | '\u{341}' => match ch {
            'ẹ' | 'Ẹ' => '\u{f01b9}', // Ẹ́ẹ́
            'ọ' | 'Ọ' => '\u{f01cd}', // Ọ́ọ́
            'ɛ' | 'Ɛ' => '\u{f015b}', // Ɛ́ɛ́
            'ɔ' | 'Ɔ' => '\u{f0154}', // Ɔ́ɔ́
            _ => ch,
        },
        '\u{302}' => match ch {
            'ɛ' | 'Ɛ' => '\u{f025b}', // Ɛ̂ɛ̂
            'ɔ' | 'Ɔ' => '\u{f0254}', // Ɔ̂ɔ̂
            _ => ch,
        },
        // '\u{303}' | '\u{342}' => match ch { // Ã
        '\u{304}' => match ch {
            'ẹ' | 'Ẹ' => '\u{f04b9}', // Ẹ̄ẹ̄
            'm' | 'M' => '\u{f046d}', // M̄m̄
            'n' | 'N' => '\u{f046e}', // N̄n̄
            'ọ' | 'Ọ' => '\u{f04cd}', // Ọ̄ọ̄
            _ => ch,
        },
        '\u{308}' => match ch {
            'ɛ' | 'Ɛ' => '\u{f0890}', // Ɛ̈ɛ̈
            'ɔ' | 'Ɔ' => '\u{f0854}', // Ɔ̈ɔ̈
            _ => ch,
        },
        '\u{30c}' => match ch {
            'ɛ' | 'Ɛ' => '\u{f0c5b}', // Ɛ̌ɛ̌
            'ɔ' | 'Ɔ' => '\u{f0c54}', // Ɔ̌ɔ̌
            _ => ch,
        },
        '\u{331}' | '\u{320}' => match ch {
            '\u{f0890}' => '\u{f3190}', // Ɛ̱̈ɛ̱̈
            'a' | 'A' => '\u{f3161}',   // A̱a̱
            'e' | 'E' => '\u{f3165}',   // E̱e̱
            'i' | 'I' => '\u{f3169}',   // I̱i̱
            'o' | 'O' => '\u{f316f}',   // O̱o̱
            'ɔ' | 'Ɔ' => '\u{f3154}',   // Ɔ̱ɔ̱
            _ => ch,
        },
        _ => ch,
    }
}

pub(crate) const WORD_COMMON_FIRST_CHAR_NOT_SKIPPABLE: &[char] = &['¡', '¿'];

// How to add a new alphabet:
// Add all the letters of all the alphabets in the script group.
//  Even though it's possible to add not all of the letters,
//   (for example when all alphabets include them, like in `Script::Han`),
//   it's recommended to add common letters (if alphabet is not very big),
//   anyway they will be filtered out by `alphabet_match!` macro.
// Do not add same `ScriptLanguage` to different scripts, except `Script::Common` or
//  languages written with different scripts in a one word (like Japanese),
//  instead create a new alphabet.
// There is no reason to add letters if the script contains only one `ScriptLanguage`.
// Do not add letters used only for loanwords, but save them in a comment.
/// Returns all `ScriptLanguage`s by `Script` and `char`
pub fn script_char_to_slangs(script: Script, ch: char) -> &'static [ScriptLanguage] {
    use Script::*;
    match script {
        Adlam => &[ScriptLanguage::Fulani, ScriptLanguage::PularAdlam],
        Ahom => &[ScriptLanguage::Ahom],
        AnatolianHieroglyphs => &[ScriptLanguage::LuwianHieroglyphic],
        Arabic => alphabet_match!([
            (
                ScriptLanguage::AcehneseJawi,
                [
                    'ا', 'ب', 'ت', 'ث', 'ج', 'چ', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض',
                    'ط', 'ظ', 'ع', 'غ', 'ڠ', 'ف', 'ڤ', 'ق', 'ك', 'گ', 'ل', 'م', 'ن', 'ڽ', 'و', 'ه',
                    'ء', 'ي', 'ى', 'ک', 'ئ'
                ]
            ),
            (
                ScriptLanguage::Arabic, //+
                [
                    'ا', 'ب', 'ت', 'ث', 'ج', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط',
                    'ظ', 'ع', 'غ', 'ف', 'ق', 'ك', 'ل', 'م', 'ن', 'ه', 'و', 'ي', 'ء', 'ى',
                    'ئ', /* 'ة', 'أ', 'إ', */
                ]
            ),
            (
                ScriptLanguage::ArabicEgyptian,
                [
                    'ا', 'ب', 'ت', 'ث', 'ج', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط',
                    'ظ', 'ع', 'غ', 'ف', 'ق', 'ك', 'ل', 'م', 'ن', 'ه', 'و', 'ي', 'ء', 'ى',
                    'ئ', /* 'ة', 'أ', 'إ', */
                ]
            ),
            (
                ScriptLanguage::ArabicMesopotamian,
                [
                    'ا', 'ب', 'ت', 'ث', 'ج', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط',
                    'ظ', 'ع', 'غ', 'ف', 'ق', 'ك', 'ل', 'م', 'ن', 'ه', 'و', 'ي', 'ء', 'ى', 'ئ',
                    'گ', /* 'ة', 'أ', 'إ', */
                ]
            ),
            (
                ScriptLanguage::ArabicMoroccan,
                [
                    'ا', 'ب', 'ت', 'ث', 'ج', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط',
                    'ظ', 'ع', 'غ', 'ف', 'ق', 'ك', 'ل', 'م', 'ن', 'ه', 'و', 'ي', 'ء', 'گ', 'ﺉ', 'ݣ',
                    'ى', 'ئ',
                    /* 'ة', 'أ', 'إ', */
                    'ڭ',
                    // transcription of loanwords 'پ', 'ڤ'
                ]
            ),
            (
                ScriptLanguage::ArabicNajdi,
                [
                    'ا', 'ب', 'ت', 'ث', 'ج', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط',
                    'ظ', 'ع', 'غ', 'ف', 'ق', 'ك', 'ل', 'م', 'ن', 'ه', 'و', 'ي', 'ء', 'ى',
                    'ئ', /* 'ة', 'أ', 'إ', */
                ]
            ),
            (
                ScriptLanguage::ArabicNorthLevantine,
                [
                    'ا', 'ب', 'ت', 'ث', 'ج', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط',
                    'ظ', 'ع', 'غ', 'ف', 'ق', 'ك', 'ل', 'م', 'ن', 'ه', 'و', 'ي', 'ء', 'ى',
                    'ئ', /* 'ة', 'أ', 'إ', */
                ]
            ),
            (
                ScriptLanguage::ArabicSouthernYemeni,
                [
                    'ا', 'ب', 'ت', 'ث', 'ج', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط',
                    'ظ', 'ع', 'غ', 'ف', 'ق', 'ك', 'ل', 'م', 'ن', 'ه', 'و', 'ي', 'ء', 'ى',
                    'ئ', /* 'ة', 'أ', 'إ', */
                ]
            ),
            (
                ScriptLanguage::ArabicSouthLevantine,
                [
                    'ا', 'ب', 'ت', 'ث', 'ج', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط',
                    'ظ', 'ع', 'غ', 'ف', 'ق', 'ك', 'ل', 'م', 'ن', 'ه', 'و', 'ي', 'ء', 'ى',
                    'ئ', /* 'ة', 'أ', 'إ', */
                ]
            ),
            (
                ScriptLanguage::ArabicTunisian,
                [
                    'ا', 'ب', 'ت', 'ث', 'ج', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط',
                    'ظ', 'ع', 'غ', 'ف', 'ق', 'ك', 'ل', 'م', 'ن', 'ه', 'و', 'ي', 'ء', 'گ', 'ﺉ', 'ݣ',
                    'ى', 'ئ', /* 'ة', 'أ', 'إ', */
                ]
            ),
            (
                ScriptLanguage::AzerbaijaniSouth,
                [
                    'ا', 'ب', 'پ', 'ت', 'ث', 'ج', 'چ', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'ژ', 'س', 'ش',
                    'ص', 'ض', 'ط', 'ظ', 'ع', 'غ', 'ف', 'ق', 'ك', 'گ', 'ل', 'م', 'ن', 'و', 'ه', 'ی',
                    'ء', 'ڤ', 'ک', 'ئ', 'ي', 'ۇ', 'ۆ'
                ]
            ),
            (
                ScriptLanguage::BanjarJawi,
                [
                    'ا', 'ب', 'ت', 'ث', 'ج', 'چ', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض',
                    'ط', 'ظ', 'ع', 'غ', 'ڠ', 'ف', 'ڤ', 'ق', 'ك', 'گ', 'ل', 'م', 'ن', 'ڽ', 'و', 'ه',
                    'ء', 'ي', 'ى', 'ڭ', 'ک', 'ئ', 'پ'
                ]
            ),
            (
                ScriptLanguage::DogriPersoArabic,
                [
                    'ا', 'ب', 'پ', 'ت', 'ٹ', 'ث', 'ج', 'چ', 'ح', 'خ', 'د', 'ڈ', 'ذ', 'ر', 'ڑ', 'ز',
                    'ژ', 'س', 'ش', 'ص', 'ض', 'ط', 'ظ', 'ع', 'غ', 'ف', 'ق', 'ک', 'گ', 'ل', 'م', 'ن',
                    'و', 'ہ', 'ء', 'ی', 'ے', 'ڤ'
                ]
            ),
            (
                ScriptLanguage::KanuriCentralAjami,
                [
                    'ا', 'ب', 'ت', 'ث', 'ج', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'س', 'ش', 'ص', 'ض', 'ط',
                    'ظ', 'ع', 'غ', 'ف', 'ق', 'ك', 'ل', 'م', 'ن', 'و', 'ه', 'ي', 'ء', 'ى', 'ی'
                ]
            ),
            (
                ScriptLanguage::Kashmiri,
                [
                    'ا', 'ب', 'پ', 'ت', 'ٹ', 'ث', 'ج', 'چ', 'ح', 'خ', 'د', 'ڈ', 'ذ', 'ر', 'ڑ', 'ز',
                    'ژ', 'س', 'ش', 'ص', 'ض', 'ط', 'ظ', 'ع', 'غ', 'ف', 'ق', 'ک', 'گ', 'ل', 'م', 'ن',
                    'ں', 'و', 'ہ', 'ء', 'ی', 'ے', 'ﺉ', 'ڒ', 'ئ', 'ھ'
                ]
            ),
            (
                ScriptLanguage::KurdishCentral,
                [
                    'ا', 'ب', 'پ', 'ت', 'ج', 'چ', 'ح', 'خ', 'د', 'ر', 'ز', 'ژ', 'س', 'ش', 'ع', 'غ',
                    'ف', 'ڤ', 'ق', 'ک', 'گ', 'ل', 'م', 'ن', 'ه', 'و', 'ي', 'ێ', 'ە', 'ی', 'ۆ', 'ئ',
                    'ك', 'ھ', 'ى'
                ]
            ),
            (
                ScriptLanguage::KurdishSouthern,
                [
                    'ا', 'ب', 'پ', 'ت', 'ج', 'چ', 'ح', 'خ', 'د', 'ر', 'ز', 'ژ', 'س', 'ش', 'ع', 'غ',
                    'ف', 'ڤ', 'ق', 'ک', 'گ', 'ل', 'م', 'ن', 'ه', 'و', 'ي', 'ێ', 'ە'
                ]
            ),
            (
                ScriptLanguage::PashtoSouthern,
                [
                    'ا', 'ب', 'پ', 'ت', 'ټ', 'ث', 'ج', 'چ', 'ح', 'خ', 'د', 'ډ', 'ذ', 'ر', 'ړ', 'ز',
                    'ژ', 'س', 'ش', 'ص', 'ض', 'ط', 'ظ', 'ع', 'غ', 'ف', 'ق', 'ک', 'ګ', 'ل', 'م', 'ن',
                    'ڼ', 'و', 'ه', 'ی', 'ې', 'ئ', 'ء', 'ڤ', 'گ', 'ڭ', 'ﺉ', 'ڒ', 'ٹ', 'ڈ', 'ڑ', 'ڧ',
                    'ي', 'ى', 'ے', 'ك'
                ]
            ),
            (
                ScriptLanguage::PersianDari,
                [
                    'ا', 'ب', 'پ', 'ت', 'ث', 'ج', 'چ', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'ژ', 'س', 'ش',
                    'ص', 'ض', 'ط', 'ظ', 'ع', 'غ', 'ف', 'ق', 'ک', 'گ', 'ل', 'م', 'ن', 'و', 'ه', 'ی',
                    'ء', 'ڈ', 'ڑ', 'ڧ', 'ڤ', 'ئ', 'ك', 'ي'
                ]
            ),
            (
                ScriptLanguage::PersianWestern, //+
                [
                    'ا', 'ب', 'پ', 'ت', 'ث', 'ج', 'چ', 'ح', 'خ', 'د', 'ذ', 'ر', 'ز', 'ژ', 'س', 'ش',
                    'ص', 'ض', 'ط', 'ظ', 'ع', 'غ', 'ف', 'ق', 'ک', 'گ', 'ل', 'م', 'ن', 'و', 'ه', 'ی',
                    'ء', 'ڤ', 'ئ', 'ك', 'ي', /* 'آ' */ 'ى'
                ]
            ),
            (
                ScriptLanguage::PunjabiEasternShahmukhi,
                [
                    'ا', 'ب', 'پ', 'ت', 'ٹ', 'ث', 'ج', 'چ', 'ح', 'خ', 'د', 'ڈ', 'ذ', 'ر', 'ڑ', 'ز',
                    'ژ', 'س', 'ش', 'ص', 'ض', 'ط', 'ظ', 'ع', 'غ', 'ف', 'ق', 'ک', 'گ', 'ل', 'م', 'ن',
                    'و', 'ہ', 'ء', 'ی', 'ے'
                ]
            ),
            (
                ScriptLanguage::Saraiki, //+
                [
                    'ا', 'ب', 'ٻ', 'پ', 'ت', 'ٹ', 'ث', 'ج', 'ڄ', 'چ', 'ح', 'خ', 'د', 'ڈ', 'ݙ', 'ذ',
                    'ر', 'ڑ', 'ز', 'س', 'ش', 'ص', 'ض', 'ط', 'ظ', 'ع', 'غ', 'ف', 'ق', 'ک', 'گ', 'ڳ',
                    'ل', 'م', 'ن', 'ں', 'ݨ', 'و', 'ہ', 'ھ', 'ی', 'ے',
                    // loanwords 'ژ'
                ]
            ),
            (
                ScriptLanguage::Sindhi,
                [
                    'ا', 'ب', 'پ', 'ت', 'ٿ', 'ث', 'ج', 'ڄ', 'چ', 'ح', 'خ', 'د', 'ڊ', 'ذ', 'ر', 'ڙ',
                    'ز', 'ژ', 'س', 'ش', 'ص', 'ض', 'ط', 'ظ', 'ع', 'غ', 'ف', 'ق', 'ڪ', 'گ', 'ل', 'م',
                    'ن', 'ڻ', 'و', 'ه', 'ء', 'ي', 'ی', 'ے', 'ڑ', 'ڤ', 'ڧ', 'ئ', 'ک', 'ہ', 'ھ', 'ى'
                ]
            ),
            (
                ScriptLanguage::Urdu,
                [
                    'ا', 'ب', 'پ', 'ت', 'ٹ', 'ث', 'ج', 'چ', 'ح', 'خ', 'د', 'ڈ', 'ذ', 'ر', 'ڑ', 'ز',
                    'ژ', 'س', 'ش', 'ص', 'ض', 'ط', 'ظ', 'ع', 'غ', 'ف', 'ق', 'ک', 'گ', 'ل', 'م', 'ن',
                    'ں', 'و', 'ہ', 'ء', 'ی', 'ے', 'ڧ', 'ئ', 'ه', 'ھ', 'ي'
                ]
            ),
            (
                ScriptLanguage::Uyghur,
                [
                    'ا', 'ب', 'پ', 'ت', 'ج', 'چ', 'خ', 'د', 'ر', 'ز', 'ژ', 'س', 'ش', 'غ', 'ف', 'ق',
                    'ك', 'ڭ', 'ل', 'م', 'ن', 'و', 'ۇ', 'ۆ', 'ۋ', 'ې', 'ى', 'ي', 'ئ', 'ە', 'گ',
                    'ھ',
                    // loanwords 'ص'
                ]
            ),
        ]),
        Armenian => &[ScriptLanguage::Armenian],
        Avestan => &[ScriptLanguage::Avestan],
        Balinese => &[ScriptLanguage::BalineseBalinese],
        Bamum => &[ScriptLanguage::Bamum],
        BassaVah => &[ScriptLanguage::Bassa],
        Batak => &[
            ScriptLanguage::Angkola,
            ScriptLanguage::Karo,
            ScriptLanguage::MandailingBatak,
            ScriptLanguage::Pakpak,
            ScriptLanguage::Simalungun,
            ScriptLanguage::TobaBatak,
        ],
        Bengali => &[
            ScriptLanguage::Assamese,
            ScriptLanguage::Bengali,
            ScriptLanguage::BishnupriyaManipuri,
            ScriptLanguage::MeiteiBengali,
        ],
        Bhaiksuki => &[ScriptLanguage::SanskritBhaiksuki],
        Bopomofo => &[ScriptLanguage::ChineseMandarinBopomofo],
        Brahmi => &[
            ScriptLanguage::SanskritBrahmi,
            ScriptLanguage::Prakrit,
            ScriptLanguage::MarathiBrahmi,
        ],
        Braille => &[ScriptLanguage::Braille],
        Buginese => &[
            ScriptLanguage::BugineseBuginese,
            ScriptLanguage::MakassareseBuginese,
        ],
        Buhid => &[ScriptLanguage::Buhid],
        CanadianAboriginal => &[
            ScriptLanguage::Cree,
            ScriptLanguage::Inuktitut,
            ScriptLanguage::Ojibwe,
        ],
        Carian => &[ScriptLanguage::Carian],
        CaucasianAlbanian => &[ScriptLanguage::CaucasianAlbanian],
        Chakma => &[ScriptLanguage::Chakma],
        Cham => &[ScriptLanguage::ChamEastern, ScriptLanguage::ChamWestern],
        Cherokee => &[ScriptLanguage::Cherokee],
        Chorasmian => &[ScriptLanguage::Chorasmian],
        // If you want to add something here, validate that char's range is active in `ucd.rs`.
        // During parsing these considered as connectors
        // example1: `can't` for english is one word,
        //  but `one/two` will be parsed as separate words,
        //  because if alphabets of all 3 chars do not intersect, it will be two words
        // example2: `aaa' bbb` for all langs will be parsed as two words without `'`,
        //  because next char after `'` is space, which is not a char of any alphabet
        Common => match ch {
            '\'' => &[
                // Cyrillic
                ScriptLanguage::Belarusian,
                ScriptLanguage::Ukrainian,
                // Latin
                ScriptLanguage::Acehnese,
                ScriptLanguage::Afrikaans,
                ScriptLanguage::AlbanianTosk,
                ScriptLanguage::Asturian,
                ScriptLanguage::AymaraCentral,
                ScriptLanguage::AzerbaijaniNorth,
                ScriptLanguage::Balinese,
                ScriptLanguage::Bambara,
                ScriptLanguage::Banjar,
                ScriptLanguage::Bemba,
                ScriptLanguage::Buginese,
                ScriptLanguage::Catalan,
                ScriptLanguage::Cebuano,
                ScriptLanguage::Chokwe,
                ScriptLanguage::CreoleHaitian,
                ScriptLanguage::Danish,
                ScriptLanguage::Dholuo,
                ScriptLanguage::Dutch,
                ScriptLanguage::Dyula,
                ScriptLanguage::English,
                ScriptLanguage::Faroese,
                ScriptLanguage::Fijian,
                ScriptLanguage::Finnish,
                ScriptLanguage::Fon,
                ScriptLanguage::French,
                ScriptLanguage::Friulian,
                ScriptLanguage::FulfuldeNigerian,
                ScriptLanguage::GaelicScottish,
                ScriptLanguage::Galician,
                ScriptLanguage::Ganda,
                ScriptLanguage::Guarani,
                ScriptLanguage::Hausa,
                ScriptLanguage::Hawaiian,
                ScriptLanguage::Icelandic,
                ScriptLanguage::Igbo,
                ScriptLanguage::Ilocano,
                ScriptLanguage::Indonesian,
                ScriptLanguage::Irish,
                ScriptLanguage::Italian,
                ScriptLanguage::Javanese,
                ScriptLanguage::Jingpho,
                ScriptLanguage::Kabuverdianu,
                ScriptLanguage::Kamba,
                ScriptLanguage::KanuriCentral,
                ScriptLanguage::Kikongo,
                ScriptLanguage::Kikuyu,
                ScriptLanguage::Kimbundu,
                ScriptLanguage::Kinyarwanda,
                ScriptLanguage::Ligurian,
                ScriptLanguage::Limburgish,
                ScriptLanguage::Lingala,
                ScriptLanguage::Lombard,
                ScriptLanguage::LubaKasai,
                ScriptLanguage::Luxembourgish,
                ScriptLanguage::Malay,
                ScriptLanguage::MalgasyPlateau,
                ScriptLanguage::Maltese,
                ScriptLanguage::Maori,
                ScriptLanguage::Mizo,
                ScriptLanguage::Mossi,
                ScriptLanguage::NorwegianBokmal,
                ScriptLanguage::NorwegianNynorsk,
                ScriptLanguage::Nyanja,
                ScriptLanguage::Occitan,
                ScriptLanguage::OromoSouthern,
                ScriptLanguage::OromoWestCentral,
                ScriptLanguage::Pangasinan,
                ScriptLanguage::Papiamento,
                ScriptLanguage::QuechuaAyacucho,
                ScriptLanguage::Rundi,
                ScriptLanguage::Samoan,
                ScriptLanguage::Sango,
                ScriptLanguage::Sardinian,
                ScriptLanguage::Sepedi,
                ScriptLanguage::Sesotho,
                ScriptLanguage::Shona,
                ScriptLanguage::Sicilian,
                ScriptLanguage::Somali,
                ScriptLanguage::Sundanese,
                ScriptLanguage::Swahili,
                ScriptLanguage::Swati,
                ScriptLanguage::Tagalog,
                ScriptLanguage::TamasheqLatin,
                ScriptLanguage::TokPisin,
                ScriptLanguage::Tsonga,
                ScriptLanguage::Tswana,
                ScriptLanguage::Tumbuka,
                ScriptLanguage::Turkish,
                ScriptLanguage::Twi,
                ScriptLanguage::Umbundu,
                ScriptLanguage::UzbekNorthern,
                ScriptLanguage::Venetian,
                ScriptLanguage::Welsh,
                ScriptLanguage::Wolof,
                ScriptLanguage::Xhosa,
                ScriptLanguage::Zulu,
            ],
            '¡' => &[ScriptLanguage::AymaraCentral, ScriptLanguage::Spanish],
            '¿' => &[
                ScriptLanguage::AymaraCentral,
                ScriptLanguage::QuechuaAyacucho,
                ScriptLanguage::Spanish,
            ],
            'ʻ' => &[
                ScriptLanguage::Hawaiian,
                ScriptLanguage::Samoan,
                ScriptLanguage::UzbekNorthern,
            ],
            // '-' => bypassed for all langs in `word_iter`
            _ => &[], // must be always empty
        },
        Coptic => &[ScriptLanguage::Coptic],
        Cuneiform => &[
            ScriptLanguage::Akkadian,
            ScriptLanguage::Hittite,
            ScriptLanguage::LuwianCuneiform,
            ScriptLanguage::Sumerian,
        ],
        Cypriot => &[ScriptLanguage::AncientGreek],
        CyproMinoan => &[ScriptLanguage::Minoan],
        Cyrillic => alphabet_match!([
            (
                ScriptLanguage::Bashkir, //+
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Ғ', 'ғ', 'Д', 'д', 'Ҙ', 'ҙ', 'Е', 'е',
                    'Ё', 'ё', 'Ж', 'ж', 'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Ҡ', 'ҡ', 'Л', 'л',
                    'М', 'м', 'Н', 'н', 'Ң', 'ң', 'О', 'о', 'Ө', 'ө', 'П', 'п', 'Р', 'р', 'С', 'с',
                    'Ҫ', 'ҫ', 'Т', 'т', 'У', 'у', 'Ү', 'ү', 'Ф', 'ф', 'Х', 'х', 'Һ', 'һ', 'Ц', 'ц',
                    'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ы', 'ы', 'Ь', 'ь', 'Э', 'э', 'Ә', 'ә',
                    'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                ScriptLanguage::Belarusian,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                    'З', 'з', 'І', 'і', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о',
                    'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ў', 'ў', 'Ф', 'ф', 'Х', 'х',
                    'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Ы', 'ы', 'Ь', 'ь', 'Э', 'э', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                ScriptLanguage::Bulgarian,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ж', 'ж', 'З', 'з',
                    'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о', 'П', 'п',
                    'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч',
                    'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ь', 'ь', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                ScriptLanguage::Kazakh,
                [
                    'А', 'а', 'Ә', 'ә', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Ғ', 'ғ', 'Д', 'д', 'Е', 'е',
                    'Ё', 'ё', 'Ж', 'ж', 'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Қ', 'қ', 'Л', 'л',
                    'М', 'м', 'Н', 'н', 'Ң', 'ң', 'О', 'о', 'Ө', 'ө', 'П', 'п', 'Р', 'р', 'С', 'с',
                    'Т', 'т', 'У', 'у', 'Ұ', 'ұ', 'Ү', 'ү', 'Ф', 'ф', 'Х', 'х', 'Һ', 'һ', 'Ц', 'ц',
                    'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ы', 'ы', 'І', 'і', 'Ь', 'ь', 'Э', 'э',
                    'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                ScriptLanguage::Kyrgyz,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                    'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'Ң', 'ң',
                    'О', 'о', 'Ө', 'ө', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ү', 'ү',
                    'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ы', 'ы',
                    'Ь', 'ь', 'Э', 'э', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                ScriptLanguage::Macedonian,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Ѓ', 'ѓ', 'Е', 'е', 'Ж', 'ж',
                    'З', 'з', 'Ѕ', 'ѕ', 'И', 'и', 'Ј', 'ј', 'К', 'к', 'Л', 'л', 'Љ', 'љ', 'М', 'м',
                    'Н', 'н', 'Њ', 'њ', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'Ќ', 'ќ',
                    'У', 'у', 'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Џ', 'џ', 'Ш', 'ш',
                ]
            ),
            (
                ScriptLanguage::MongolianHalh,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                    'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о',
                    'Ө', 'ө', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ү', 'ү', 'Ф', 'ф',
                    'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ы', 'ы', 'Ь', 'ь',
                    'Э', 'э', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                ScriptLanguage::ChurchSlavonicOld,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Є', 'є', 'Ж', 'ж', 'Ѕ', 'ѕ',
                    'З', 'з', 'Ꙁ', 'ꙁ', 'И', 'и', 'І', 'і', 'Ї', 'ї', 'К', 'к', 'Л', 'л', 'М', 'м',
                    'Н', 'н', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'Ѹ', 'ѹ', 'Ꙋ', 'ꙋ',
                    'Ф', 'ф', 'Х', 'х', 'Ѡ', 'ѡ', 'Ѿ', 'ѿ', 'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ',
                    'Ъ', 'ъ', 'Ꙑ', 'ꙑ', 'Ь', 'ь', 'Ѣ', 'ѣ', 'Ꙗ', 'ꙗ', 'Ѥ', 'ѥ', 'Ю', 'ю', 'Ѫ', 'ѫ',
                    'Ѭ', 'ѭ', 'Ѧ', 'ѧ', 'Ѩ', 'ѩ', 'Ѯ', 'ѯ', 'Ѱ', 'ѱ', 'Ѳ', 'ѳ', 'Ѵ', 'ѵ', 'Ҁ', 'ҁ',
                ]
            ),
            (
                ScriptLanguage::Chuvash, //+
                [
                    'А', 'а', 'Ӑ', 'ӑ', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё',
                    'Ӗ', 'ӗ', 'Ж', 'ж', 'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м',
                    'Н', 'н', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Ҫ', 'ҫ', 'Т', 'т', 'У', 'у',
                    'Ӳ', 'ӳ', 'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ',
                    'Ы', 'ы', 'Ь', 'ь', 'Э', 'э', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                ScriptLanguage::Russian, //+++
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                    'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о',
                    'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ф', 'ф', 'Х', 'х', 'Ц', 'ц',
                    'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ы', 'ы', 'Ь', 'ь', 'Э', 'э', 'Ю', 'ю',
                    'Я', 'я',
                ]
            ),
            (
                ScriptLanguage::Serbian,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Ђ', 'ђ', 'Е', 'е', 'Ж', 'ж',
                    'З', 'з', 'И', 'и', 'Ј', 'ј', 'К', 'к', 'Л', 'л', 'Љ', 'љ', 'М', 'м', 'Н', 'н',
                    'Њ', 'њ', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'Ћ', 'ћ', 'У', 'у',
                    'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Џ', 'џ', 'Ш', 'ш',
                ]
            ),
            (
                ScriptLanguage::Tajik,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Ғ', 'ғ', 'Д', 'д', 'Е', 'е', 'Ё', 'ё',
                    'Ж', 'ж', 'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Қ', 'қ', 'Л', 'л', 'М', 'м',
                    'Н', 'н', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ф', 'ф',
                    'Х', 'х', 'Ҳ', 'ҳ', 'Ч', 'ч', 'Ҷ', 'ҷ', 'Ш', 'ш', 'Ъ', 'ъ', 'Э', 'э', 'Ю', 'ю',
                    'Я', 'я',
                ]
            ),
            (
                ScriptLanguage::Tatar,
                [
                    'А', 'а', 'Ә', 'ә', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё',
                    'Ж', 'ж', 'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н',
                    'Ң', 'ң', 'О', 'о', 'Ө', 'ө', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у',
                    'Ү', 'ү', 'Ф', 'ф', 'Х', 'х', 'Һ', 'һ', 'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ',
                    'Ъ', 'ъ', 'Ы', 'ы', 'Ь', 'ь', 'Э', 'э', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                ScriptLanguage::Ukrainian,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Ґ', 'ґ', 'Д', 'д', 'Е', 'е', 'Є', 'є',
                    'Ж', 'ж', 'З', 'з', 'И', 'и', 'І', 'і', 'Ї', 'ї', 'Й', 'й', 'К', 'к', 'Л', 'л',
                    'М', 'м', 'Н', 'н', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у',
                    'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ь', 'ь', 'Ю', 'ю',
                    'Я', 'я',
                ]
            ),
        ]),
        Deseret => &[ScriptLanguage::EnglishDeseret],
        Devanagari => alphabet_match!([
            (
                ScriptLanguage::Awadhi,
                [
                    'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख', 'ग', 'घ', 'ङ',
                    'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द', 'ध', 'न', 'प',
                    'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
                ],
            ),
            (
                ScriptLanguage::Bhojpuri,
                [
                    'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख', 'ग', 'घ', 'ङ',
                    'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द', 'ध', 'न', 'प',
                    'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
                ],
            ),
            (
                ScriptLanguage::Chhattisgarhi,
                [
                    'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख', 'ग', 'घ', 'ङ',
                    'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द', 'ध', 'न', 'प',
                    'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
                ],
            ),
            (
                ScriptLanguage::Dogri,
                [
                    'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख', 'ग', 'घ', 'ङ',
                    'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द', 'ध', 'न', 'प',
                    'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
                ],
            ),
            (
                ScriptLanguage::Hindi,
                [
                    'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख', 'ग', 'घ', 'ङ',
                    'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द', 'ध', 'न', 'प',
                    'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
                ],
            ),
            (
                ScriptLanguage::KashmiriDevanagari,
                [
                    'अ', 'आ', 'ऍ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख', 'ग', 'घ',
                    'ङ', 'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द', 'ध', 'न',
                    'प', 'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
                ],
            ),
            (
                ScriptLanguage::Magahi,
                [
                    'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख', 'ग', 'घ', 'ङ',
                    'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द', 'ध', 'न', 'प',
                    'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
                ],
            ),
            (
                ScriptLanguage::Maithili,
                [
                    'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख', 'ग', 'घ', 'ङ',
                    'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द', 'ध', 'न', 'प',
                    'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
                ],
            ),
            (
                ScriptLanguage::Marathi,
                [
                    'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख', 'ग', 'घ', 'ङ',
                    'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द', 'ध', 'न', 'प',
                    'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह', 'ळ',
                ],
            ),
            (
                ScriptLanguage::Nepali,
                [
                    'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख', 'ग', 'घ', 'ङ',
                    'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द', 'ध', 'न', 'प',
                    'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
                ],
            ),
            (
                ScriptLanguage::Sanskrit,
                [
                    'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ॠ', 'ऌ', 'ॡ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख',
                    'ग', 'घ', 'ङ', 'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द',
                    'ध', 'न', 'प', 'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
                    // Vedic Sanskrit
                    'ळ',
                ],
            ),
            (
                ScriptLanguage::SaurashtraDevanagari,
                [
                    'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख', 'ग', 'घ', 'ङ',
                    'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द', 'ध', 'न', 'प',
                    'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
                ],
            ),
            (
                ScriptLanguage::SindhiDevanagari,
                [
                    'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख', 'ग', 'घ', 'ङ',
                    'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द', 'ध', 'न', 'प',
                    'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
                ],
            ),
            (
                ScriptLanguage::Wancho,
                [
                    'अ', 'आ', 'इ', 'ई', 'उ', 'ऊ', 'ऋ', 'ए', 'ऐ', 'ओ', 'औ', 'क', 'ख', 'ग', 'घ', 'ङ',
                    'च', 'छ', 'ज', 'झ', 'ञ', 'ट', 'ठ', 'ड', 'ढ', 'ण', 'त', 'थ', 'द', 'ध', 'न', 'प',
                    'फ', 'ब', 'भ', 'म', 'य', 'र', 'ल', 'व', 'श', 'ष', 'स', 'ह',
                ],
            ),
        ]),
        DivesAkuru => &[ScriptLanguage::DhivehiDivesAkuru],
        Dogra => &[ScriptLanguage::DogriDogra],
        Duployan => &[
            ScriptLanguage::EnglishDuployan,
            ScriptLanguage::FrenchDuployan,
        ],
        EgyptianHieroglyphs => &[ScriptLanguage::EgyptianHieroglyphs],
        Elbasan => &[ScriptLanguage::AlbanianElbasan],
        Elymaic => &[ScriptLanguage::AramaicElymaic],
        Ethiopic => &[
            ScriptLanguage::Amharic,
            ScriptLanguage::Geez,
            // ScriptLanguage::Oromo,
            ScriptLanguage::Tigrinya,
        ],
        Garay => &[ScriptLanguage::WolofGaray],
        Georgian => &[ScriptLanguage::Georgian],
        Glagolitic => &[ScriptLanguage::ChurchSlavonicOldGlagolitic],
        Gothic => &[ScriptLanguage::Gothic],
        Grantha => &[
            ScriptLanguage::SanskritGrantha,
            ScriptLanguage::TamilGrantha,
        ],
        Greek => &[ScriptLanguage::Greek],
        Gujarati => &[ScriptLanguage::Gujarati],
        GunjalaGondi => &[ScriptLanguage::GondiGunjala],
        Gurmukhi => &[ScriptLanguage::PunjabiEastern],
        GurungKhema => &[ScriptLanguage::GurungKhema],
        Han => alphabet_match!([
            (
                ScriptLanguage::ChineseSimplified,
                [
                    // https://en.wikisource.org/wiki/Translation:List_of_Frequently_Used_Characters_in_Modern_Chinese
                    // https://www.tutormandarin.net/en/list-of-different-simplified-and-traditional-characters/
                    // invalid '十', '三', '二', '四', '五', '後', '七', '九', '八', '六', '百',
                    // '千', '涌', '着', '污', '恒', '症', '沉', '扎', '厘',
                    '劳', '学', '绅', '浑', '么', '轮', '达', '韦', '边', '详', '练', '养', '颜',
                    '篱', '伞', '筛', '牵', '谐', '备', '捞', '国', '厅', '尔', '级', '饼', '仓',
                    '骑', '没', '蕴', '仅', '势', '冻', '纲', '厉', '决', '账', '内', '极', '灵',
                    '统', '临', '莲', '橱', '块', '战', '础', '广', '蜡', '证', '匀', '争', '腾',
                    '营', '妇', '权', '记', '梦', '页', '鸭', '从', '铲', '楼', '龄', '启', '涝',
                    '简', '诀', '扑', '资', '灾', '浆', '俭', '剧', '锐', '华', '垦', '压', '转',
                    '盖', '炼', '红', '骚', '谬', '蓝', '矿', '聋', '谎', '为', '枣', '爷', '雾',
                    '瞒', '腊', '径', '纽', '挣', '缅', '辅', '贞', '询', '赔', '吗', '丢', '迁',
                    '麦', '穷', '鲁', '贪', '窝', '设', '闸', '闲', '综', '祸', '栏', '较', '缴',
                    '钢', '冈', '厂', '盏', '亚', '滩', '号', '贼', '凉', '数', '敌', '伟', '监',
                    '狱', '卫', '饭', '纯', '俩', '惊', '庄', '鹤', '协', '尽', '脑', '关', '济',
                    '桨', '濒', '迈', '猎', '续', '厢', '韧', '侨', '帐', '裤', '温', '区', '彻',
                    '写', '宽', '陆', '赖', '饲', '优', '绸', '惭', '谋', '睁', '爱', '喷', '满',
                    '聪', '药', '铃', '缰', '赐', '译', '驰', '诚', '绒', '胁', '庙', '窜', '枢',
                    '犹', '泽', '鹅', '帘', '倾', '频', '饥', '讼', '绿', '纳', '赛', '沦', '癞',
                    '谢', '洁', '终', '会', '举', '业', '侣', '归', '婴', '亏', '输', '伤', '潇',
                    '继', '轴', '凭', '弃', '汇', '灿', '岭', '诱', '调', '叙', '缓', '镇', '谣',
                    '烁', '闯', '蚂', '独', '据', '欢', '浊', '钩', '贸', '硕', '阳', '壶', '缘',
                    '绰', '缚', '齐', '乐', '荣', '诅', '歼', '汤', '发', '丽', '衔', '谨', '办',
                    '摇', '夺', '严', '岂', '闭', '阵', '这', '胶', '厦', '杨', '妆', '额', '毁',
                    '经', '删', '疗', '问', '陕', '忧', '无', '饱', '绘', '龟', '艺', '迹', '铸',
                    '坠', '仑', '岁', '驶', '择', '摆', '够', '萝', '缝', '园', '离', '弯', '奖',
                    '谦', '哑', '挂', '桥', '垄', '粪', '锣', '荫', '颗', '滞', '录', '嚣', '态',
                    '斋', '晋', '镶', '辽', '庞', '戏', '纹', '听', '洒', '纪', '赢', '伦', '讹',
                    '题', '说', '啸', '搅', '显', '渊', '践', '气', '舱', '宾', '坚', '凛', '携',
                    '馋', '载', '偿', '蝇', '响', '皱', '键', '谴', '壮', '识', '嘱', '潜', '铭',
                    '凯', '误', '谱', '励', '怀', '峡', '类', '贺', '谓', '异', '帅', '鸠', '刹',
                    '艰', '郑', '脸', '闹', '络', '赞', '贫', '档', '鹰', '阁', '晓', '锋', '绵',
                    '审', '荡', '滚', '坝', '让', '热', '师', '线', '侠', '沟', '秆', '试', '规',
                    '读', '净', '买', '篮', '纫', '烫', '泻', '实', '筝', '锄', '缠', '袄', '时',
                    '秃', '诉', '礼', '趋', '错', '摄', '锤', '彦', '虽', '乡', '丛', '阶', '认',
                    '拦', '须', '亿', '执', '锡', '侧', '鳞', '谭', '鉴', '滤', '锻', '过', '咏',
                    '烧', '鲸', '讲', '阅', '罚', '紧', '维', '义', '编', '损', '陈', '领', '纠',
                    '贾', '箩', '锦', '论', '馒', '闻', '阔', '圆', '盗', '吴', '检', '画', '呐',
                    '捣', '窃', '绕', '绢', '泪', '厨', '讽', '宝', '网', '骂', '挤', '誉', '囱',
                    '哟', '屡', '递', '桩', '弹', '务', '盘', '渗', '蛮', '虑', '庐', '呜', '谅',
                    '晒', '猪', '献', '孙', '顷', '赠', '冲', '坏', '苏', '体', '贵', '习', '党',
                    '脱', '湾', '鳄', '阐', '门', '劝', '兰', '恶', '崭', '贴', '晕', '运', '医',
                    '辈', '凑', '违', '专', '话', '挥', '灭', '条', '顽', '轨', '葱', '粮', '种',
                    '际', '两', '宪', '琼', '赏', '觉', '涂', '坟', '状', '鸡', '毙', '庆', '狭',
                    '昼', '扬', '莱', '万', '计', '坛', '贡', '笋', '费', '该', '摊', '凫', '处',
                    '肠', '苹', '鹦', '职', '垒', '对', '样', '顿', '恋', '单', '挡', '谁', '纸',
                    '泼', '柜', '诵', '隶', '邻', '乱', '寻', '纬', '舆', '难', '遗', '萨', '责',
                    '辫', '军', '哗', '项', '训', '声', '钞', '览', '称', '龙', '苍', '驻', '馆',
                    '飘', '辉', '电', '镰', '愤', '邓', '织', '贩', '盐', '萧', '宁', '驴', '亲',
                    '撑', '现', '鸦', '拢', '颠', '茧', '则', '贱', '呛', '视', '释', '钻', '兴',
                    '鸟', '拥', '鸽', '碍', '拟', '栈', '屿', '钓', '浓', '开', '玛', '绳', '书',
                    '颊', '扫', '刘', '畅', '鹊', '肤', '渐', '饿', '旷', '翘', '烦', '抡', '复',
                    '胜', '购', '静', '茎', '护', '缎', '农', '辩', '驳', '稳', '组', '劲', '蚁',
                    '牺', '链', '胀', '缕', '斩', '笔', '兽', '芜', '讨', '汉', '邮', '帜', '鬓',
                    '惧', '吁', '闪', '诺', '虏', '诗', '韵', '烛', '验', '触', '掷', '娇', '顺',
                    '脉', '鸿', '却', '滨', '胆', '饰', '沪', '稣', '绑', '鲍', '乌', '缩', '讶',
                    '刚', '儿', '亩', '细', '跃', '残', '镜', '还', '叶', '浅', '疮', '马', '应',
                    '产', '换', '预', '衬', '壳', '挠', '锈', '狈', '骤', '员', '顶', '们', '舰',
                    '齿', '贤', '捡', '构', '夹', '轻', '叠', '滥', '逊', '铺', '剑', '报', '颈',
                    '铅', '鳖', '瘾', '纷', '竞', '册', '觅', '饺', '赌', '烂', '库', '懒', '惩',
                    '恼', '搁', '驾', '蒋', '蚀', '肿', '狮', '赵', '妈', '给', '风', '张', '枪',
                    '总', '驮', '缆', '远', '与', '阀', '笼', '遥', '间', '双', '绍', '忆', '标',
                    '谊', '痒', '拨', '辙', '图', '击', '许', '弥', '厌', '朴', '侦', '钟', '猫',
                    '烟', '镖', '剂', '质', '险', '头', '悬', '县', '软', '诊', '耻', '惯', '酿',
                    '户', '杀', '环', '贝', '顾', '宫', '颁', '货', '强', '呕', '厕', '颂', '卢',
                    '迟', '闰', '针', '败', '砖', '进', '锹', '参', '纵', '艳', '驯', '颇', '诈',
                    '讯', '创', '卖', '个', '语', '娱', '层', '适', '贿', '销', '饮', '团', '癣',
                    '骄', '躏', '墙', '鹏', '溃', '测', '芦', '辞', '订', '评', '抢', '访', '当',
                    '车', '馅', '卤', '变', '唤', '财', '诸', '绣', '隐', '旧', '锁', '横', '鳍',
                    '疯', '补', '沧', '堕', '长', '纱', '众', '耸', '属', '脏', '钦', '树', '罢',
                    '惨', '轧', '岛', '怜', '婶', '传', '苇', '词', '带', '负', '确', '恳', '废',
                    '纤', '鸣', '樱', '纺', '冯', '驱', '坞', '岖', '赋', '颖', '逻', '价', '骆',
                    '辆', '黄', '尝', '动', '钳', '汹', '储', '讥', '奋', '枫', '虾', '竖', '抠',
                    '栋', '润', '谈', '钥', '况', '断', '联', '装', '债', '禅', '抚', '丝', '将',
                    '圣', '垫', '银', '术', '炉', '袜', '轿', '拣', '选', '结', '辑', '铜', '兹',
                    '队', '锯', '吕', '伪', '观', '尘', '巩', '吨', '鱼', '围', '赶', '蚕', '岗',
                    '贯', '积', '涛', '暂', '讳', '肾', '罗', '别', '涨', '诞', '届', '锅', '赡',
                    '韩', '鲜', '赚', '腻', '连', '减', '来', '肃', '贷', '奥', '湿', '请', '轩',
                    '签', '浇', '仪', '胧', '躯', '钉', '导', '寿', '见', '点', '筹', '渔', '饶',
                    '阴', '丧', '课', '搂', '酱', '币', '骡', '欧', '宠', '获', '兑', '凤', '东',
                    '担', '帮', '毕', '约', '攒', '沥', '扰', '骗', '抛', '杂', '铁', '轰', '绝',
                    '闷', '踪', '虚', '灯', '驼', '颤', '荐', '节', '绞', '谜', '绩', '叹', '议',
                    '袭', '钱', '场', '码', '窑', '乔', '随', '历', '扩', '绪', '飞', '愿', '虫',
                ]
            ),
            (
                ScriptLanguage::ChineseTraditional,
                [
                    '後', '並', '併', '緖', '絶', '偽', '証', '嘆', '稅', '脱', '剝', '悅',
                    // simplified autoconverted
                    // https://www.lexilogos.com/keyboard/chinese_conversion.htm
                    // then here https://www.chineseconverter.com/en/convert/simplified-to-traditional
                    // analogue chars are from here
                    // https://www.chinese-tools.com/tools/converter-simptrad.html
                    '勞', '學', '紳', '渾', '麽', '輪', '達', '韋', '邊', '詳', '練', '養', '顔',
                    '籬', '傘', '篩', '牽', '諧', '備', '撈', '國', '廳', '爾', '級', '餅', '倉',
                    '騎', '沒', '蘊', '僅', '勢', '凍', '綱', '厲', '決', '賬', '內', '極', '靈',
                    '統', '臨', '蓮', '櫥', '塊', '戰', '礎', '廣', '蠟', '證', '勻', '爭', '騰',
                    '營', '婦', '權', '記', '夢', '頁', '鴨', '從', '鏟', '樓', '齡', '啓', '澇',
                    '簡', '訣', '撲', '資', '災', '漿', '儉', '劇', '銳', '華', '墾', '壓', '轉',
                    '蓋', '煉', '紅', '騷', '謬', '藍', '礦', '聾', '謊', '為', '棗', '爺', '霧',
                    '瞞', '臘', '徑', '紐', '掙', '緬', '輔', '貞', '詢', '賠', '嗎', '丟', '遷',
                    '麥', '窮', '魯', '貪', '窩', '設', '閘', '閑', '綜', '禍', '欄', '較', '繳',
                    '鋼', '岡', '廠', '盞', '亞', '灘', '號', '賊', '涼', '數', '敵', '偉', '監',
                    '獄', '衛', '飯', '純', '倆', '驚', '莊', '鶴', '協', '盡', '腦', '關', '濟',
                    '槳', '瀕', '邁', '獵', '續', '廂', '韌', '僑', '帳', '褲', '溫', '區', '徹',
                    '寫', '寬', '陸', '賴', '飼', '優', '綢', '慚', '謀', '睜', '愛', '噴', '滿',
                    '聰', '藥', '鈴', '韁', '賜', '譯', '馳', '誠', '絨', '脅', '廟', '竄', '樞',
                    '猶', '澤', '鵝', '簾', '傾', '頻', '饑', '訟', '綠', '納', '賽', '淪', '癩',
                    '謝', '潔', '終', '會', '舉', '業', '侶', '歸', '嬰', '虧', '輸', '傷', '瀟',
                    '繼', '軸', '憑', '棄', '彙', '燦', '嶺', '誘', '調', '敘', '緩', '鎮', '謠',
                    '爍', '闖', '螞', '獨', '據', '歡', '濁', '鈎', '貿', '碩', '陽', '壺', '緣',
                    '綽', '縛', '齊', '樂', '榮', '詛', '殲', '湯', '發', '麗', '銜', '謹', '辦',
                    '搖', '奪', '嚴', '豈', '閉', '陣', '這', '膠', '廈', '楊', '妝', '額', '毀',
                    '經', '刪', '療', '問', '陝', '憂', '無', '飽', '繪', '龜', '藝', '跡', '鑄',
                    '墜', '侖', '歲', '駛', '擇', '擺', '夠', '蘿', '縫', '園', '離', '彎', '獎',
                    '謙', '啞', '掛', '橋', '壟', '糞', '鑼', '蔭', '顆', '滯', '錄', '囂', '態',
                    '齋', '晉', '鑲', '遼', '龐', '戲', '紋', '聽', '灑', '紀', '贏', '倫', '訛',
                    '題', '說', '嘯', '攪', '顯', '淵', '踐', '氣', '艙', '賓', '堅', '凜', '攜',
                    '饞', '載', '償', '蠅', '響', '皺', '鍵', '譴', '壯', '識', '囑', '潛', '銘',
                    '凱', '誤', '譜', '勵', '懷', '峽', '類', '賀', '謂', '異', '帥', '鳩', '剎',
                    '艱', '鄭', '臉', '鬧', '絡', '贊', '貧', '檔', '鷹', '閣', '曉', '鋒', '綿',
                    '審', '蕩', '滾', '壩', '讓', '熱', '師', '線', '俠', '溝', '稈', '試', '規',
                    '讀', '淨', '買', '籃', '紉', '燙', '瀉', '實', '箏', '鋤', '纏', '襖', '時',
                    '禿', '訴', '禮', '趨', '錯', '攝', '錘', '彥', '雖', '鄉', '叢', '階', '認',
                    '攔', '須', '億', '執', '錫', '側', '鱗', '譚', '鑒', '濾', '鍛', '過', '詠',
                    '燒', '鯨', '講', '閱', '罰', '緊', '維', '義', '編', '損', '陳', '領', '糾',
                    '賈', '籮', '錦', '論', '饅', '聞', '闊', '圓', '盜', '吳', '檢', '畫', '吶',
                    '搗', '竊', '繞', '絹', '淚', '廚', '諷', '寶', '網', '罵', '擠', '譽', '囪',
                    '喲', '屢', '遞', '樁', '彈', '務', '盤', '滲', '蠻', '慮', '廬', '嗚', '諒',
                    '曬', '豬', '獻', '孫', '頃', '贈', '沖', '壞', '蘇', '體', '貴', '習', '黨',
                    '脫', '灣', '鱷', '闡', '門', '勸', '蘭', '惡', '嶄', '貼', '暈', '運', '醫',
                    '輩', '湊', '違', '專', '話', '揮', '滅', '條', '頑', '軌', '蔥', '糧', '種',
                    '際', '兩', '憲', '瓊', '賞', '覺', '塗', '墳', '狀', '雞', '斃', '慶', '狹',
                    '晝', '揚', '萊', '萬', '計', '壇', '貢', '筍', '費', '該', '攤', '鳧', '處',
                    '腸', '蘋', '鸚', '職', '壘', '對', '樣', '頓', '戀', '單', '擋', '誰', '紙',
                    '潑', '櫃', '誦', '隸', '鄰', '亂', '尋', '緯', '輿', '難', '遺', '薩', '責',
                    '辮', '軍', '嘩', '項', '訓', '聲', '鈔', '覽', '稱', '龍', '蒼', '駐', '館',
                    '飄', '輝', '電', '鐮', '憤', '鄧', '織', '販', '鹽', '蕭', '甯', '驢', '親',
                    '撐', '現', '鴉', '攏', '顛', '繭', '則', '賤', '嗆', '視', '釋', '鑽', '興',
                    '鳥', '擁', '鴿', '礙', '擬', '棧', '嶼', '釣', '濃', '開', '瑪', '繩', '書',
                    '頰', '掃', '劉', '暢', '鵲', '膚', '漸', '餓', '曠', '翹', '煩', '掄', '複',
                    '勝', '購', '靜', '莖', '護', '緞', '農', '辯', '駁', '穩', '組', '勁', '蟻',
                    '犧', '鏈', '脹', '縷', '斬', '筆', '獸', '蕪', '討', '漢', '郵', '幟', '鬢',
                    '懼', '籲', '閃', '諾', '虜', '詩', '韻', '燭', '驗', '觸', '擲', '嬌', '順',
                    '脈', '鴻', '卻', '濱', '膽', '飾', '滬', '穌', '綁', '鮑', '烏', '縮', '訝',
                    '剛', '兒', '畝', '細', '躍', '殘', '鏡', '還', '葉', '淺', '瘡', '馬', '應',
                    '産', '換', '預', '襯', '殼', '撓', '鏽', '狽', '驟', '員', '頂', '們', '艦',
                    '齒', '賢', '撿', '構', '夾', '輕', '疊', '濫', '遜', '鋪', '劍', '報', '頸',
                    '鉛', '鼈', '癮', '紛', '競', '冊', '覓', '餃', '賭', '爛', '庫', '懶', '懲',
                    '惱', '擱', '駕', '蔣', '蝕', '腫', '獅', '趙', '媽', '給', '風', '張', '槍',
                    '總', '馱', '纜', '遠', '與', '閥', '籠', '遙', '間', '雙', '紹', '憶', '標',
                    '誼', '癢', '撥', '轍', '圖', '擊', '許', '彌', '厭', '樸', '偵', '鍾', '貓',
                    '煙', '鏢', '劑', '質', '險', '頭', '懸', '縣', '軟', '診', '恥', '慣', '釀',
                    '戶', '殺', '環', '貝', '顧', '宮', '頒', '貨', '強', '嘔', '廁', '頌', '盧',
                    '遲', '閏', '針', '敗', '磚', '進', '鍬', '參', '縱', '豔', '馴', '頗', '詐',
                    '訊', '創', '賣', '個', '語', '娛', '層', '適', '賄', '銷', '飲', '團', '癬',
                    '驕', '躪', '牆', '鵬', '潰', '測', '蘆', '辭', '訂', '評', '搶', '訪', '當',
                    '車', '餡', '鹵', '變', '喚', '財', '諸', '繡', '隱', '舊', '鎖', '橫', '鰭',
                    '瘋', '補', '滄', '墮', '長', '紗', '衆', '聳', '屬', '髒', '欽', '樹', '罷',
                    '慘', '軋', '島', '憐', '嬸', '傳', '葦', '詞', '帶', '負', '確', '懇', '廢',
                    '纖', '鳴', '櫻', '紡', '馮', '驅', '塢', '嶇', '賦', '穎', '邏', '價', '駱',
                    '輛', '黃', '嘗', '動', '鉗', '洶', '儲', '譏', '奮', '楓', '蝦', '豎', '摳',
                    '棟', '潤', '談', '鑰', '況', '斷', '聯', '裝', '債', '禪', '撫', '絲', '將',
                    '聖', '墊', '銀', '術', '爐', '襪', '轎', '揀', '選', '結', '輯', '銅', '茲',
                    '隊', '鋸', '呂', '僞', '觀', '塵', '鞏', '噸', '魚', '圍', '趕', '蠶', '崗',
                    '貫', '積', '濤', '暫', '諱', '腎', '羅', '別', '漲', '誕', '屆', '鍋', '贍',
                    '韓', '鮮', '賺', '膩', '連', '減', '來', '肅', '貸', '奧', '濕', '請', '軒',
                    '簽', '澆', '儀', '朧', '軀', '釘', '導', '壽', '見', '點', '籌', '漁', '饒',
                    '陰', '喪', '課', '摟', '醬', '幣', '騾', '歐', '寵', '獲', '兌', '鳳', '東',
                    '擔', '幫', '畢', '約', '攢', '瀝', '擾', '騙', '拋', '雜', '鐵', '轟', '絕',
                    '悶', '蹤', '虛', '燈', '駝', '顫', '薦', '節', '絞', '謎', '績', '歎', '議',
                    '襲', '錢', '場', '碼', '窯', '喬', '隨', '曆', '擴', '緒', '飛', '願', '蟲',
                    '麼', '瞇', '隻', '顚',
                ]
            ),
            (
                ScriptLanguage::ChineseCantoneseTraditional,
                [
                    '晒', '响', '温', '畀', //
                    // http://whitey.net/en/typing-cantonese-characters.htm
                    // invalid '傾','緊','呢',
                    '呃', '啤', '俾', '煲', '噚', '哋', '啲', '瞓', '噉', '咁', '梗', '嘅', '嗰',
                    '癐', '吓', '喺', '係', '唨', '咗', '咭', '佢', '姖', '嘞', '叻', '靚', '嚟',
                    '唔', '乜', '咩', '冇', '嬲', '啱', '嗮', '閂', '呔', '睇', '唞', '搵', '嘢',
                    // DO NOT EDIT BELOW (copy from ChineseTraditional from above)
                    '後', '並', '併', '緖', '絶', '偽', '証', '嘆', '稅', '脱', '剝', '悅',
                    // simplified autoconverted
                    '勞', '學', '紳', '渾', '麽', '輪', '達', '韋', '邊', '詳', '練', '養', '顔',
                    '籬', '傘', '篩', '牽', '諧', '備', '撈', '國', '廳', '爾', '級', '餅', '倉',
                    '騎', '沒', '蘊', '僅', '勢', '凍', '綱', '厲', '決', '賬', '內', '極', '靈',
                    '統', '臨', '蓮', '櫥', '塊', '戰', '礎', '廣', '蠟', '證', '勻', '爭', '騰',
                    '營', '婦', '權', '記', '夢', '頁', '鴨', '從', '鏟', '樓', '齡', '啓', '澇',
                    '簡', '訣', '撲', '資', '災', '漿', '儉', '劇', '銳', '華', '墾', '壓', '轉',
                    '蓋', '煉', '紅', '騷', '謬', '藍', '礦', '聾', '謊', '為', '棗', '爺', '霧',
                    '瞞', '臘', '徑', '紐', '掙', '緬', '輔', '貞', '詢', '賠', '嗎', '丟', '遷',
                    '麥', '窮', '魯', '貪', '窩', '設', '閘', '閑', '綜', '禍', '欄', '較', '繳',
                    '鋼', '岡', '廠', '盞', '亞', '灘', '號', '賊', '涼', '數', '敵', '偉', '監',
                    '獄', '衛', '飯', '純', '倆', '驚', '莊', '鶴', '協', '盡', '腦', '關', '濟',
                    '槳', '瀕', '邁', '獵', '續', '廂', '韌', '僑', '帳', '褲', '溫', '區', '徹',
                    '寫', '寬', '陸', '賴', '飼', '優', '綢', '慚', '謀', '睜', '愛', '噴', '滿',
                    '聰', '藥', '鈴', '韁', '賜', '譯', '馳', '誠', '絨', '脅', '廟', '竄', '樞',
                    '猶', '澤', '鵝', '簾', '傾', '頻', '饑', '訟', '綠', '納', '賽', '淪', '癩',
                    '謝', '潔', '終', '會', '舉', '業', '侶', '歸', '嬰', '虧', '輸', '傷', '瀟',
                    '繼', '軸', '憑', '棄', '彙', '燦', '嶺', '誘', '調', '敘', '緩', '鎮', '謠',
                    '爍', '闖', '螞', '獨', '據', '歡', '濁', '鈎', '貿', '碩', '陽', '壺', '緣',
                    '綽', '縛', '齊', '樂', '榮', '詛', '殲', '湯', '發', '麗', '銜', '謹', '辦',
                    '搖', '奪', '嚴', '豈', '閉', '陣', '這', '膠', '廈', '楊', '妝', '額', '毀',
                    '經', '刪', '療', '問', '陝', '憂', '無', '飽', '繪', '龜', '藝', '跡', '鑄',
                    '墜', '侖', '歲', '駛', '擇', '擺', '夠', '蘿', '縫', '園', '離', '彎', '獎',
                    '謙', '啞', '掛', '橋', '壟', '糞', '鑼', '蔭', '顆', '滯', '錄', '囂', '態',
                    '齋', '晉', '鑲', '遼', '龐', '戲', '紋', '聽', '灑', '紀', '贏', '倫', '訛',
                    '題', '說', '嘯', '攪', '顯', '淵', '踐', '氣', '艙', '賓', '堅', '凜', '攜',
                    '饞', '載', '償', '蠅', '響', '皺', '鍵', '譴', '壯', '識', '囑', '潛', '銘',
                    '凱', '誤', '譜', '勵', '懷', '峽', '類', '賀', '謂', '異', '帥', '鳩', '剎',
                    '艱', '鄭', '臉', '鬧', '絡', '贊', '貧', '檔', '鷹', '閣', '曉', '鋒', '綿',
                    '審', '蕩', '滾', '壩', '讓', '熱', '師', '線', '俠', '溝', '稈', '試', '規',
                    '讀', '淨', '買', '籃', '紉', '燙', '瀉', '實', '箏', '鋤', '纏', '襖', '時',
                    '禿', '訴', '禮', '趨', '錯', '攝', '錘', '彥', '雖', '鄉', '叢', '階', '認',
                    '攔', '須', '億', '執', '錫', '側', '鱗', '譚', '鑒', '濾', '鍛', '過', '詠',
                    '燒', '鯨', '講', '閱', '罰', '緊', '維', '義', '編', '損', '陳', '領', '糾',
                    '賈', '籮', '錦', '論', '饅', '聞', '闊', '圓', '盜', '吳', '檢', '畫', '吶',
                    '搗', '竊', '繞', '絹', '淚', '廚', '諷', '寶', '網', '罵', '擠', '譽', '囪',
                    '喲', '屢', '遞', '樁', '彈', '務', '盤', '滲', '蠻', '慮', '廬', '嗚', '諒',
                    '曬', '豬', '獻', '孫', '頃', '贈', '沖', '壞', '蘇', '體', '貴', '習', '黨',
                    '脫', '灣', '鱷', '闡', '門', '勸', '蘭', '惡', '嶄', '貼', '暈', '運', '醫',
                    '輩', '湊', '違', '專', '話', '揮', '滅', '條', '頑', '軌', '蔥', '糧', '種',
                    '際', '兩', '憲', '瓊', '賞', '覺', '塗', '墳', '狀', '雞', '斃', '慶', '狹',
                    '晝', '揚', '萊', '萬', '計', '壇', '貢', '筍', '費', '該', '攤', '鳧', '處',
                    '腸', '蘋', '鸚', '職', '壘', '對', '樣', '頓', '戀', '單', '擋', '誰', '紙',
                    '潑', '櫃', '誦', '隸', '鄰', '亂', '尋', '緯', '輿', '難', '遺', '薩', '責',
                    '辮', '軍', '嘩', '項', '訓', '聲', '鈔', '覽', '稱', '龍', '蒼', '駐', '館',
                    '飄', '輝', '電', '鐮', '憤', '鄧', '織', '販', '鹽', '蕭', '甯', '驢', '親',
                    '撐', '現', '鴉', '攏', '顛', '繭', '則', '賤', '嗆', '視', '釋', '鑽', '興',
                    '鳥', '擁', '鴿', '礙', '擬', '棧', '嶼', '釣', '濃', '開', '瑪', '繩', '書',
                    '頰', '掃', '劉', '暢', '鵲', '膚', '漸', '餓', '曠', '翹', '煩', '掄', '複',
                    '勝', '購', '靜', '莖', '護', '緞', '農', '辯', '駁', '穩', '組', '勁', '蟻',
                    '犧', '鏈', '脹', '縷', '斬', '筆', '獸', '蕪', '討', '漢', '郵', '幟', '鬢',
                    '懼', '籲', '閃', '諾', '虜', '詩', '韻', '燭', '驗', '觸', '擲', '嬌', '順',
                    '脈', '鴻', '卻', '濱', '膽', '飾', '滬', '穌', '綁', '鮑', '烏', '縮', '訝',
                    '剛', '兒', '畝', '細', '躍', '殘', '鏡', '還', '葉', '淺', '瘡', '馬', '應',
                    '産', '換', '預', '襯', '殼', '撓', '鏽', '狽', '驟', '員', '頂', '們', '艦',
                    '齒', '賢', '撿', '構', '夾', '輕', '疊', '濫', '遜', '鋪', '劍', '報', '頸',
                    '鉛', '鼈', '癮', '紛', '競', '冊', '覓', '餃', '賭', '爛', '庫', '懶', '懲',
                    '惱', '擱', '駕', '蔣', '蝕', '腫', '獅', '趙', '媽', '給', '風', '張', '槍',
                    '總', '馱', '纜', '遠', '與', '閥', '籠', '遙', '間', '雙', '紹', '憶', '標',
                    '誼', '癢', '撥', '轍', '圖', '擊', '許', '彌', '厭', '樸', '偵', '鍾', '貓',
                    '煙', '鏢', '劑', '質', '險', '頭', '懸', '縣', '軟', '診', '恥', '慣', '釀',
                    '戶', '殺', '環', '貝', '顧', '宮', '頒', '貨', '強', '嘔', '廁', '頌', '盧',
                    '遲', '閏', '針', '敗', '磚', '進', '鍬', '參', '縱', '豔', '馴', '頗', '詐',
                    '訊', '創', '賣', '個', '語', '娛', '層', '適', '賄', '銷', '飲', '團', '癬',
                    '驕', '躪', '牆', '鵬', '潰', '測', '蘆', '辭', '訂', '評', '搶', '訪', '當',
                    '車', '餡', '鹵', '變', '喚', '財', '諸', '繡', '隱', '舊', '鎖', '橫', '鰭',
                    '瘋', '補', '滄', '墮', '長', '紗', '衆', '聳', '屬', '髒', '欽', '樹', '罷',
                    '慘', '軋', '島', '憐', '嬸', '傳', '葦', '詞', '帶', '負', '確', '懇', '廢',
                    '纖', '鳴', '櫻', '紡', '馮', '驅', '塢', '嶇', '賦', '穎', '邏', '價', '駱',
                    '輛', '黃', '嘗', '動', '鉗', '洶', '儲', '譏', '奮', '楓', '蝦', '豎', '摳',
                    '棟', '潤', '談', '鑰', '況', '斷', '聯', '裝', '債', '禪', '撫', '絲', '將',
                    '聖', '墊', '銀', '術', '爐', '襪', '轎', '揀', '選', '結', '輯', '銅', '茲',
                    '隊', '鋸', '呂', '僞', '觀', '塵', '鞏', '噸', '魚', '圍', '趕', '蠶', '崗',
                    '貫', '積', '濤', '暫', '諱', '腎', '羅', '別', '漲', '誕', '屆', '鍋', '贍',
                    '韓', '鮮', '賺', '膩', '連', '減', '來', '肅', '貸', '奧', '濕', '請', '軒',
                    '簽', '澆', '儀', '朧', '軀', '釘', '導', '壽', '見', '點', '籌', '漁', '饒',
                    '陰', '喪', '課', '摟', '醬', '幣', '騾', '歐', '寵', '獲', '兌', '鳳', '東',
                    '擔', '幫', '畢', '約', '攢', '瀝', '擾', '騙', '拋', '雜', '鐵', '轟', '絕',
                    '悶', '蹤', '虛', '燈', '駝', '顫', '薦', '節', '絞', '謎', '績', '歎', '議',
                    '襲', '錢', '場', '碼', '窯', '喬', '隨', '曆', '擴', '緒', '飛', '願', '蟲',
                    '麼', '瞇', '隻', '顚',
                ]
            ),
            (
                ScriptLanguage::Korean,
                [
                    '難', '漢', '衛', '賓', '為', '頻', '併', '旣', '汚', '緖', '緒', '懲', '脹',
                    '謹', '証', '顛', '後', '凜', '顚', '來', '业', '内', '携', '彦', '庄', '猫',
                    '却', '悅', //
                    // unverified
                    '亜', '悪', '圧', '囲', '医', '栄', '駅', '謁', '応', '欧', '奥', '横',
                    /* '温', */ '仮', '価', '禍', '画', '会', '絵', '覚', '学', '楽', '渇',
                    '寛', '関', '歓', '観', '気', '帰', '亀', '偽', '戯', '旧', '挙', '虚', '郷',
                    '響', '区', '勲', '径', '恵', '掲', '経', '蛍', '軽', '継', '鶏', '芸', '撃',
                    '県', '倹', '剣', '圏', '検', '権', '顕', '験', '厳', '広', '黄', '鉱', '号',
                    '国', '黒', '砕', '済', '斎', '剤', '雑', '参', '賛', '残', '祉', '視', '児',
                    '実', '写', '舎', '釈', '収', '従', '渋', '獣', '粛', '処', '叙', '将', '称',
                    '渉', '焼', '奨', '条', '状', '乗', '浄', '剰', '縄', '壌', '譲', '触', '図',
                    '粋', '酔', '穂', '数', '声', '斉', '静', '窃', '摂', '節', '絶', '専', '浅',
                    '戦', '践', '銭', '潜', '双', '壮', '争', '荘', '捜', '巣', '装', '総', '増',
                    '蔵', '属', '続', '堕', '対', '体', '帯', '滞', '滝', '沢', '担', '単', '胆',
                    '嘆', '団', '断', '弾', '遅', '痴', '虫', '庁', '徴', '聴', '勅', '鎮', '逓',
                    '鉄', '点', '転', '伝', '灯', '当', '盗', '稲', '闘', '徳', '独', '読', '届',
                    '弐', '脳', '覇', '廃', '売', '発', '抜', '晩', '払', '仏', '並', '塀', '餅',
                    '辺', '変', '弁', '歩', '宝', '豊', '没', '毎', '万', '満', '弥', '訳', '薬',
                    '与', '誉', '揺', '様', /* '来', */ '頼', '乱', '欄', '竜', '虜', '両',
                    '猟', '緑', '涙', '塁', '類', '礼', '励', '戻', '霊', '齢', '暦', '歴', '恋',
                    '練', '錬', '炉', '労', '楼', '録', '湾',
                ]
            ),
            (
                // it also uses all Traditional Chinese characters
                ScriptLanguage::Japanese,
                [
                    // Jōyō Shinjitai kanji (https://en.wikipedia.org/wiki/List_of_j%C5%8Dy%C5%8D_kanji)
                    // invalid '台','海','者','即','都','著','社','真','研','神','免','器','層',
                    // '朗','殺','祖','福','突','署','梅','秘','概','翻','余','既','敏', 祈 欠 糸
                    // '祝','繁','隆','諸','墨','郎','廊','祥','煮','勤','碑','予','卑','憎','暑',
                    // '恒','喝','岳','臭','穀','慎','逸','僧','贈','瓶','慨','悔','勉','侮',
                    // alternative forms? 恵 掲 黒
                    '亜', '悪', '圧', '囲', '医', '壱', '隠', '栄', '営', '駅', '謁', '円', '塩',
                    '縁', '艶', '応', '欧', '殴', '桜', '奥', '横', '温', '穏', '仮', '価', '画',
                    '会', '絵', '壊', '懐', '拡', '殻', '覚', '学', '楽', '渇', '褐', '缶', '巻',
                    '陥', '勧', '寛', '関', '歓', '観', '気', '帰', '亀', '戯', '犠', '旧', '拠',
                    '挙', '虚', '峡', '挟', '狭', '郷', '暁', '区', '駆', '勲', '薫', '径', '茎',
                    '恵', '掲', '渓', '経', '蛍', '軽', '継', '鶏', '芸', '撃', '県', '倹', '剣',
                    '険', '圏', '検', '献', '権', '顕', '験', '厳', '広', '効', '黄', '鉱', '号',
                    '国', '黒', '砕', '済', '斎', '剤', '雑', '参', '桟', '蚕', '惨', '賛', '残',
                    '祉', '歯', '児', '辞', '湿', '実', '写', '舎', '釈', '寿', '収', '従', '渋',
                    '獣', '縦', '粛', '処', '叙', '将', '称', '渉', '焼', '奨', '条', '状', '乗',
                    '浄', '剰', '畳', '縄', '壌', '嬢', '譲', '醸', '触', '嘱', '寝', '尽', '図',
                    '粋', '酔', '穂', '随', '髄', '枢', '数', '瀬', '声', '斉', '静', '窃', '摂',
                    '専', '浅', '戦', '践', '銭', '潜', '繊', '禅', '双', '壮', '争', '荘', '捜',
                    '挿', '巣', '曽', '痩', '装', '総', '騒', '増', '蔵', '臓', '属', '続', '堕',
                    '対', '体', '帯', '滞', '滝', '択', '沢', '担', '単', '胆', '団', '断', '弾',
                    '遅', '痴', '虫', '昼', '鋳', '庁', '徴', '聴', '勅', '塚', '逓', '鉄', '点',
                    '転', '伝', '灯', '当', '党', '盗', '稲', '闘', '徳', '独', '読', '届', '弐',
                    '悩', '脳', '覇', '拝', '廃', '売', '麦', '発', '髪', '抜', '晩', '蛮', '浜',
                    '払', '仏', '塀', '辺', '変', '弁', '歩', '宝', '豊', '褒', '没', '毎', '万',
                    '満', '麺', '黙', '弥', '訳', '薬', '与', '誉', '揺', '様', '謡', '来', '頼',
                    '乱', '覧', '竜', '両', '猟', '緑', '涙', '塁', '礼', '励', '戻', '霊', '齢',
                    '暦', '歴', '恋', '錬', '炉', '労', '楼', '録', '湾',
                    // Jinmeiyō Kanji (https://en.wikipedia.org/wiki/Ky%C5%ABjitai)
                    '亘', /* '凜', */ '尭', '巌', '晃', '桧', '槙', '渚', '猪', '琢', '祢',
                    '祐', '祷', '禄', '禎', '穣', '萌', '遥',
                    // Hyōgai kanji (https://en.wikipedia.org/wiki/Ky%C5%ABjitai)
                    // invalid '并', '屏',
                    '唖', '頴', '鴎', '躯', '撹', '麹', '鹸', '噛', '繍', '蒋', '醤', '掻', '沪',
                    '芦', '蝋', '弯', '焔', '砿', '讃', /* '顛', */ '醗', '溌', '輌', '繋',
                    // Kokuji (https://en.wikipedia.org/wiki/Kokuji)
                    // Jōyō Kanji
                    // invalid '腺',
                    '働', '匂', '峠', '枠', '栃', '畑', '込', '搾',
                    // Jinmeiyō Kanji (https://en.wikipedia.org/wiki/Ky%C5%ABjitai)
                    '勺', '銑', /* '脹', */ /* '錘', */ '匁', '俣', '凧', '凪', '喰', '柾',
                    '椛', '榊', '樫', '畠', '笹', '籾', '辻', '雫', '鰯', '麿', //
                    // Hyōgaiji
                    '躾', //
                    // https://en.wikipedia.org/wiki/Extended_shinjitai
                    '鴬', '掴', '箪', '涜', '侭', '薮', '篭', //
                    // others
                    '醖', '晒', '旣', '汚', '响', '业', '内', '携', '彦', '庄', '猫', '却', '厨',
                    '吓',
                    // Shinjitai dupes from copy below '緒','節','懲','虜','漢','視','難','類',
                    // '響','賓','謹','鎮','頻','衛','欄','禍','為','餅','練','嘆','証','偽','絶',
                    // '併','並',
                    // others '緖','後',
                    // DO NOT EDIT BELOW (copy from ChineseTraditional from above)
                    '後', '並', '併', '緖', '絶', '偽', '証', '嘆', '稅', '脱', '剝', '悅',
                    // simplified autoconverted
                    '勞', '學', '紳', '渾', '麽', '輪', '達', '韋', '邊', '詳', '練', '養', '顔',
                    '籬', '傘', '篩', '牽', '諧', '備', '撈', '國', '廳', '爾', '級', '餅', '倉',
                    '騎', '沒', '蘊', '僅', '勢', '凍', '綱', '厲', '決', '賬', '內', '極', '靈',
                    '統', '臨', '蓮', '櫥', '塊', '戰', '礎', '廣', '蠟', '證', '勻', '爭', '騰',
                    '營', '婦', '權', '記', '夢', '頁', '鴨', '從', '鏟', '樓', '齡', '啓', '澇',
                    '簡', '訣', '撲', '資', '災', '漿', '儉', '劇', '銳', '華', '墾', '壓', '轉',
                    '蓋', '煉', '紅', '騷', '謬', '藍', '礦', '聾', '謊', '為', '棗', '爺', '霧',
                    '瞞', '臘', '徑', '紐', '掙', '緬', '輔', '貞', '詢', '賠', '嗎', '丟', '遷',
                    '麥', '窮', '魯', '貪', '窩', '設', '閘', '閑', '綜', '禍', '欄', '較', '繳',
                    '鋼', '岡', '廠', '盞', '亞', '灘', '號', '賊', '涼', '數', '敵', '偉', '監',
                    '獄', '衛', '飯', '純', '倆', '驚', '莊', '鶴', '協', '盡', '腦', '關', '濟',
                    '槳', '瀕', '邁', '獵', '續', '廂', '韌', '僑', '帳', '褲', '溫', '區', '徹',
                    '寫', '寬', '陸', '賴', '飼', '優', '綢', '慚', '謀', '睜', '愛', '噴', '滿',
                    '聰', '藥', '鈴', '韁', '賜', '譯', '馳', '誠', '絨', '脅', '廟', '竄', '樞',
                    '猶', '澤', '鵝', '簾', '傾', '頻', '饑', '訟', '綠', '納', '賽', '淪', '癩',
                    '謝', '潔', '終', '會', '舉', '業', '侶', '歸', '嬰', '虧', '輸', '傷', '瀟',
                    '繼', '軸', '憑', '棄', '彙', '燦', '嶺', '誘', '調', '敘', '緩', '鎮', '謠',
                    '爍', '闖', '螞', '獨', '據', '歡', '濁', '鈎', '貿', '碩', '陽', '壺', '緣',
                    '綽', '縛', '齊', '樂', '榮', '詛', '殲', '湯', '發', '麗', '銜', '謹', '辦',
                    '搖', '奪', '嚴', '豈', '閉', '陣', '這', '膠', '廈', '楊', '妝', '額', '毀',
                    '經', '刪', '療', '問', '陝', '憂', '無', '飽', '繪', '龜', '藝', '跡', '鑄',
                    '墜', '侖', '歲', '駛', '擇', '擺', '夠', '蘿', '縫', '園', '離', '彎', '獎',
                    '謙', '啞', '掛', '橋', '壟', '糞', '鑼', '蔭', '顆', '滯', '錄', '囂', '態',
                    '齋', '晉', '鑲', '遼', '龐', '戲', '紋', '聽', '灑', '紀', '贏', '倫', '訛',
                    '題', '說', '嘯', '攪', '顯', '淵', '踐', '氣', '艙', '賓', '堅', '凜', '攜',
                    '饞', '載', '償', '蠅', '響', '皺', '鍵', '譴', '壯', '識', '囑', '潛', '銘',
                    '凱', '誤', '譜', '勵', '懷', '峽', '類', '賀', '謂', '異', '帥', '鳩', '剎',
                    '艱', '鄭', '臉', '鬧', '絡', '贊', '貧', '檔', '鷹', '閣', '曉', '鋒', '綿',
                    '審', '蕩', '滾', '壩', '讓', '熱', '師', '線', '俠', '溝', '稈', '試', '規',
                    '讀', '淨', '買', '籃', '紉', '燙', '瀉', '實', '箏', '鋤', '纏', '襖', '時',
                    '禿', '訴', '禮', '趨', '錯', '攝', '錘', '彥', '雖', '鄉', '叢', '階', '認',
                    '攔', '須', '億', '執', '錫', '側', '鱗', '譚', '鑒', '濾', '鍛', '過', '詠',
                    '燒', '鯨', '講', '閱', '罰', '緊', '維', '義', '編', '損', '陳', '領', '糾',
                    '賈', '籮', '錦', '論', '饅', '聞', '闊', '圓', '盜', '吳', '檢', '畫', '吶',
                    '搗', '竊', '繞', '絹', '淚', '廚', '諷', '寶', '網', '罵', '擠', '譽', '囪',
                    '喲', '屢', '遞', '樁', '彈', '務', '盤', '滲', '蠻', '慮', '廬', '嗚', '諒',
                    '曬', '豬', '獻', '孫', '頃', '贈', '沖', '壞', '蘇', '體', '貴', '習', '黨',
                    '脫', '灣', '鱷', '闡', '門', '勸', '蘭', '惡', '嶄', '貼', '暈', '運', '醫',
                    '輩', '湊', '違', '專', '話', '揮', '滅', '條', '頑', '軌', '蔥', '糧', '種',
                    '際', '兩', '憲', '瓊', '賞', '覺', '塗', '墳', '狀', '雞', '斃', '慶', '狹',
                    '晝', '揚', '萊', '萬', '計', '壇', '貢', '筍', '費', '該', '攤', '鳧', '處',
                    '腸', '蘋', '鸚', '職', '壘', '對', '樣', '頓', '戀', '單', '擋', '誰', '紙',
                    '潑', '櫃', '誦', '隸', '鄰', '亂', '尋', '緯', '輿', '難', '遺', '薩', '責',
                    '辮', '軍', '嘩', '項', '訓', '聲', '鈔', '覽', '稱', '龍', '蒼', '駐', '館',
                    '飄', '輝', '電', '鐮', '憤', '鄧', '織', '販', '鹽', '蕭', '甯', '驢', '親',
                    '撐', '現', '鴉', '攏', '顛', '繭', '則', '賤', '嗆', '視', '釋', '鑽', '興',
                    '鳥', '擁', '鴿', '礙', '擬', '棧', '嶼', '釣', '濃', '開', '瑪', '繩', '書',
                    '頰', '掃', '劉', '暢', '鵲', '膚', '漸', '餓', '曠', '翹', '煩', '掄', '複',
                    '勝', '購', '靜', '莖', '護', '緞', '農', '辯', '駁', '穩', '組', '勁', '蟻',
                    '犧', '鏈', '脹', '縷', '斬', '筆', '獸', '蕪', '討', '漢', '郵', '幟', '鬢',
                    '懼', '籲', '閃', '諾', '虜', '詩', '韻', '燭', '驗', '觸', '擲', '嬌', '順',
                    '脈', '鴻', '卻', '濱', '膽', '飾', '滬', '穌', '綁', '鮑', '烏', '縮', '訝',
                    '剛', '兒', '畝', '細', '躍', '殘', '鏡', '還', '葉', '淺', '瘡', '馬', '應',
                    '産', '換', '預', '襯', '殼', '撓', '鏽', '狽', '驟', '員', '頂', '們', '艦',
                    '齒', '賢', '撿', '構', '夾', '輕', '疊', '濫', '遜', '鋪', '劍', '報', '頸',
                    '鉛', '鼈', '癮', '紛', '競', '冊', '覓', '餃', '賭', '爛', '庫', '懶', '懲',
                    '惱', '擱', '駕', '蔣', '蝕', '腫', '獅', '趙', '媽', '給', '風', '張', '槍',
                    '總', '馱', '纜', '遠', '與', '閥', '籠', '遙', '間', '雙', '紹', '憶', '標',
                    '誼', '癢', '撥', '轍', '圖', '擊', '許', '彌', '厭', '樸', '偵', '鍾', '貓',
                    '煙', '鏢', '劑', '質', '險', '頭', '懸', '縣', '軟', '診', '恥', '慣', '釀',
                    '戶', '殺', '環', '貝', '顧', '宮', '頒', '貨', '強', '嘔', '廁', '頌', '盧',
                    '遲', '閏', '針', '敗', '磚', '進', '鍬', '參', '縱', '豔', '馴', '頗', '詐',
                    '訊', '創', '賣', '個', '語', '娛', '層', '適', '賄', '銷', '飲', '團', '癬',
                    '驕', '躪', '牆', '鵬', '潰', '測', '蘆', '辭', '訂', '評', '搶', '訪', '當',
                    '車', '餡', '鹵', '變', '喚', '財', '諸', '繡', '隱', '舊', '鎖', '橫', '鰭',
                    '瘋', '補', '滄', '墮', '長', '紗', '衆', '聳', '屬', '髒', '欽', '樹', '罷',
                    '慘', '軋', '島', '憐', '嬸', '傳', '葦', '詞', '帶', '負', '確', '懇', '廢',
                    '纖', '鳴', '櫻', '紡', '馮', '驅', '塢', '嶇', '賦', '穎', '邏', '價', '駱',
                    '輛', '黃', '嘗', '動', '鉗', '洶', '儲', '譏', '奮', '楓', '蝦', '豎', '摳',
                    '棟', '潤', '談', '鑰', '況', '斷', '聯', '裝', '債', '禪', '撫', '絲', '將',
                    '聖', '墊', '銀', '術', '爐', '襪', '轎', '揀', '選', '結', '輯', '銅', '茲',
                    '隊', '鋸', '呂', '僞', '觀', '塵', '鞏', '噸', '魚', '圍', '趕', '蠶', '崗',
                    '貫', '積', '濤', '暫', '諱', '腎', '羅', '別', '漲', '誕', '屆', '鍋', '贍',
                    '韓', '鮮', '賺', '膩', '連', '減', '來', '肅', '貸', '奧', '濕', '請', '軒',
                    '簽', '澆', '儀', '朧', '軀', '釘', '導', '壽', '見', '點', '籌', '漁', '饒',
                    '陰', '喪', '課', '摟', '醬', '幣', '騾', '歐', '寵', '獲', '兌', '鳳', '東',
                    '擔', '幫', '畢', '約', '攢', '瀝', '擾', '騙', '拋', '雜', '鐵', '轟', '絕',
                    '悶', '蹤', '虛', '燈', '駝', '顫', '薦', '節', '絞', '謎', '績', '歎', '議',
                    '襲', '錢', '場', '碼', '窯', '喬', '隨', '曆', '擴', '緒', '飛', '願', '蟲',
                    '麼', '瞇', '隻', '顚',
                ]
            ),
        ]),
        Hangul => &[ScriptLanguage::Korean],
        HanifiRohingya => &[ScriptLanguage::Rohingya],
        Hanunoo => &[ScriptLanguage::Hanunoo],
        Hatran => &[ScriptLanguage::AramaicHatran],
        Hebrew => alphabet_match!([
            (
                ScriptLanguage::Hebrew,
                [
                    'א', 'ב', 'ג', 'ד', 'ה', 'ו', 'ז', 'ח', 'ט', 'י', 'כ', 'ל', 'מ', 'נ', 'ס', 'ע',
                    'פ', 'צ', 'ק', 'ר', 'ש', 'ת'
                ]
            ),
            (
                ScriptLanguage::YiddishEastern,
                [
                    'א', 'ב', 'ג', 'ד', 'ה', 'ו', 'ז', 'ח', 'ט', 'י', 'ײ', 'כ', 'ל', 'מ', 'נ', 'ס',
                    'ע', 'פ', /* 'פּ' */ '\u{5bc}', /* 'פֿ' */ '\u{5bf}', 'צ', 'ק',
                    'ר', 'ש', 'ת', 'װ'
                ]
            ),
        ]),
        Hiragana => &[ScriptLanguage::Japanese],
        ImperialAramaic => &[ScriptLanguage::AramaicImperial],
        Inherited => &[], // must be always empty
        InscriptionalPahlavi => &[ScriptLanguage::MiddlePersianPahlaviInscriptional],
        InscriptionalParthian => &[ScriptLanguage::Parthian],
        Javanese => &[ScriptLanguage::JavaneseJavanese],
        Kaithi => &[
            ScriptLanguage::BhojpuriKaithi,
            ScriptLanguage::HindiKaithi,
            ScriptLanguage::MagahiKaithi,
            ScriptLanguage::MaithiliKaithi,
        ],
        Kannada => &[ScriptLanguage::Kannada, ScriptLanguage::Tulu],
        Katakana => &[ScriptLanguage::Japanese],
        Kawi => &[ScriptLanguage::OldJavanese, ScriptLanguage::SanskritKawi],
        KayahLi => &[ScriptLanguage::KayahEastern, ScriptLanguage::KayahWestern],
        Kharoshthi => &[ScriptLanguage::Gandhari],
        KhitanSmallScript => &[ScriptLanguage::Khitan],
        Khmer => &[ScriptLanguage::Khmer],
        Khojki => &[ScriptLanguage::KutchiKhojki, ScriptLanguage::SindhiKhojki],
        Khudawadi => &[ScriptLanguage::SindhiKhudawadi],
        KiratRai => &[ScriptLanguage::Bantawa],
        Lao => &[ScriptLanguage::Lao],
        Latin => alphabet_match!([
            (
                ScriptLanguage::Acehnese, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'É', 'é', 'È', 'è', 'Ë', 'ë',
                    'Ô', 'ô', 'Ö', 'ö' // loanwords F, Q, V, X, Z
                ]
            ),
            (
                ScriptLanguage::Afrikaans, //++
                [
                    'Á', 'á', 'A', 'a', 'Ä', 'ä', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'É', 'é',
                    'È', 'è', 'Ê', 'ê', 'Ë', 'ë', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'Í', 'í',
                    'Î', 'î', 'Ï', 'ï', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'ŉ', 'O',
                    'o', 'Ó', 'ó', 'Ô', 'ô', 'Ö', 'ö', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T',
                    't', 'Û', 'û', 'U', 'u', 'Ú', 'ú', 'Ü', 'ü', 'V', 'v', 'W', 'w', 'X', 'x', 'Ý',
                    'ý', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::Akan, //+
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'Ɔ', 'ɔ', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y',
                    'y', // loanwords C, J, Q, V, X, Z
                ]
            ),
            (
                ScriptLanguage::AlbanianTosk, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', /* 'Dh', 'dh', */ 'E',
                    'e', 'Ë', 'ë', 'F', 'f', 'G', 'g', /* 'Gj', 'gj', */ 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', /* 'Ll', 'll', */ 'M', 'm', 'N', 'n',
                    /* 'Nj', 'nj', */ 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r',
                    /* 'Rr', 'rr', */ 'S', 's', /* 'Sh', 'sh', */ 'T', 't',
                    /* 'Th', 'th', */ 'U', 'u', 'V', 'v', 'X', 'x',
                    /* 'Xh', 'xh', */ 'Y', 'y', 'Z', 'z', /* 'Zh', 'zh', */
                ]
            ),
            (
                ScriptLanguage::Asturian, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'O', 'o',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Z', 'z', 'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú',
                    'Ü', 'ü',
                ]
            ),
            (
                ScriptLanguage::AymaraCentral, //+?
                [
                    'A', 'a', 'B', 'b', /* 'Ch', 'ch', */ 'C', 'c', 'D', 'd', 'E', 'e', 'F',
                    'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l',
                    /* 'Ll', 'll', */ 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'O', 'o', 'P', 'p', 'Q',
                    'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Ä',
                    'ä', 'Ï', 'ï', 'Á', 'á', 'Ó', 'ó', 'É', 'é', 'Ü', 'ü', 'Í', 'í', 'Z', 'z', 'Ë',
                    'ë', 'Ú', 'ú', 'Ö', 'ö', /*unverified*/ 'X', 'x',
                ]
            ),
            (
                ScriptLanguage::AzerbaijaniNorth, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'Ə', 'ə', 'F', 'f',
                    'G', 'g', 'Ğ', 'ğ', 'H', 'h', 'X', 'x', 'I', 'ı', 'İ', 'i', 'J', 'j', 'K', 'k',
                    'Q', 'q', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'Ö', 'ö', 'P', 'p', 'R', 'r',
                    'S', 's', 'Ş', 'ş', 'T', 't', 'U', 'u', 'Ü', 'ü', 'V', 'v', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::Balinese, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'É', 'é', 'V', 'v',
                    'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::Bambara, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ɲ', 'ɲ',
                    'Ŋ', 'ŋ', 'O', 'o', 'Ɔ', 'ɔ', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                    'W', 'w', 'Y', 'y', 'Z', 'z', 'À', 'à', 'È', 'è', 'É', 'é', 'Ô', 'ô', 'Ù', 'ù',
                    'Ú',
                    'ú',
                    // any more diacritics? 'Á', 'á', 'Ì', 'ì', 'Í', 'í', 'Ò', 'ò', 'Ó', 'ó',
                ]
            ),
            (
                ScriptLanguage::Banjar, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'É', 'é'
                ]
            ),
            (
                ScriptLanguage::Basque, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'O', 'o',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Z', 'z', 'Ç', 'ç', 'Á', 'á', 'É', 'é'
                ]
            ),
            (
                ScriptLanguage::Bemba, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::Bosnian, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'Ć', 'ć', 'D', 'd', 'Đ', 'đ', 'E', 'e',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l',
                    /* 'Lj', 'lj', */ 'M', 'm', 'N', 'n', /* 'Nj', 'nj', */ 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž',
                ]
            ),
            (
                ScriptLanguage::Buginese, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                ScriptLanguage::Catalan, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'À', 'à', 'Ç', 'ç', 'É', 'é', 'È', 'è', 'Ë', 'ë', 'Í', 'í',
                    'Ï', 'ï', 'Ó', 'ó', 'Ò', 'ò', 'Ü', 'ü', 'Ú', 'ú',
                ]
            ),
            (
                ScriptLanguage::Cebuano, //++
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'G', 'g', 'H', 'h', 'I', 'i', 'K', 'k',
                    'L', 'l', 'M', 'm', 'N', 'n', /* 'Ng', 'ng', */ 'O', 'o', 'P', 'p', 'R',
                    'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y',
                    'y',
                    // loanwords c, f, j, q, v, x, z
                ]
            ),
            (
                ScriptLanguage::Chokwe, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::CreoleHaitian, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z',
                    'À', 'à', 'È', 'è', 'É', 'é', 'Ò', 'ò'
                ]
            ),
            (
                ScriptLanguage::Croatian, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'Ć', 'ć', 'D', 'd', 'Đ', 'đ', 'E', 'e',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l',
                    /* 'Lj', 'lj', */ 'M', 'm', 'N', 'n', /* 'Nj', 'nj', */ 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž'
                ]
            ),
            (
                ScriptLanguage::Czech, //+
                [
                    'A', 'a', 'Á', 'á', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'Ď', 'ď', 'E', 'e',
                    'È', 'è', 'É', 'é', 'Ě', 'ě', 'F', 'f', 'G', 'g', 'H', 'h',
                    /* 'Ch', 'ch', */ 'I', 'i', 'Ì', 'ì', 'Í', 'í', 'J', 'j', 'K', 'k', 'L',
                    'l', 'M', 'm', 'N', 'n', 'Ň', 'ň', 'O', 'o', 'Ó', 'ó', 'P', 'p', 'R', 'r', 'Ř',
                    'ř', 'S', 's', 'Š', 'š', 'T', 't', 'Ť', 'ť', 'U', 'u', 'Ù', 'ù', 'Ú', 'ú', 'Ů',
                    'ů', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Ý', 'ý', 'Z', 'z', 'Ž', 'ž', 'Ø',
                    'ø',
                ]
            ),
            (
                ScriptLanguage::Danish, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Å', 'å', 'Æ', 'æ', 'É', 'é', 'Ø', 'ø',
                ]
            ),
            (
                ScriptLanguage::Dholuo, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::DinkaSouthwestern, //++
                #[rustfmt::skip]
                [
                    'Ä', 'ä', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ë', 'ë', 'Ɛ', 'ɛ',
                    'G', 'g', 'Ɣ', 'ɣ', 'H', 'h', 'I', 'i', 'Ï', 'ï', 'J', 'j', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'Ŋ', 'ŋ', 'Ö', 'ö', 'O', 'o', 'Ɔ', 'ɔ', 'P', 'p', 'R', 'r',
                    'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y',
                    // from `char_compose_custom`
                    '\u{f0854}', // Ɔ̈ɔ̈
                    '\u{f0890}', // Ɛ̈ɛ̈
                ]
            ),
            (
                ScriptLanguage::Dutch, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Á', 'á', 'Ä', 'ä', 'È', 'è', 'Ë', 'ë', 'É', 'é', 'Ï', 'ï',
                    'Ĳ', 'ĳ', 'Ó', 'ó', 'Ö', 'ö', 'Ü', 'ü',
                ]
            ),
            (
                ScriptLanguage::Dyula, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ɲ', 'ɲ',
                    'O', 'o', 'Ɔ', 'ɔ', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v',
                    'W', 'w', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::English, //+++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::Esperanto, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ĉ', 'ĉ', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'Ĝ', 'ĝ', 'H', 'h', 'Ĥ', 'ĥ', 'I', 'i', 'J', 'j', 'Ĵ', 'ĵ', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'Ŝ', 'ŝ', 'T', 't',
                    'U', 'u', 'Ŭ', 'ŭ', 'V', 'v', 'X', 'x', 'Y', 'y', 'Z', 'z', // XY?
                ]
            ),
            (
                ScriptLanguage::Estonian, //+
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'Ä', 'ä', 'Ö', 'ö', 'Õ', 'õ', 'Š', 'š',
                    'Ü', 'ü', 'Ž', 'ž', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::Ewe, //+
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'Ð', 'Ɖ', 'ɖ', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'Ƒ',
                    'ƒ', 'G', 'g', 'Ɣ', 'ɣ', 'H', 'h', 'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N',
                    'n', 'Ŋ', 'ŋ', 'O', 'o', 'Ɔ', 'ɔ', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U',
                    'u', 'V', 'v', 'Ʋ', 'ʋ', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z', 'Ã', 'ã', 'À',
                    'à', 'È', 'è', 'Ẽ', 'ẽ', 'É', 'é', 'Ĩ', 'ĩ', 'Í', 'í', 'Ò', 'ò',
                ]
            ),
            (
                ScriptLanguage::Faroese, //+
                [
                    'A', 'a', 'Á', 'á', 'B', 'b', 'D', 'd', 'Ð', 'ð', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'Í', 'í', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'O', 'o', 'Ó', 'ó', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'Ú', 'ú',
                    'V', 'v', 'Y', 'y', 'Ý', 'ý', 'Æ', 'æ', 'Ø', 'ø'
                ]
            ),
            (
                ScriptLanguage::Fijian, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y',
                    'y',
                    // loanwords 'H', 'h',
                ]
            ),
            (
                ScriptLanguage::Finnish, //++
                [
                    'A', 'a', 'Ä', 'ä', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ö', 'ö', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v',
                    'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::Fon, //+
                [
                    'A', 'a', 'À', 'à', 'Á', 'á', 'Ǎ', 'ǎ', 'B', 'b', 'C', 'c', 'D', 'd', 'Ð', 'Ɖ',
                    'ɖ', 'Ě', 'ě', 'É', 'é', 'È', 'è', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g', 'H',
                    'h', 'Ǐ', 'ǐ', 'Í', 'í', 'Ì', 'ì', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M',
                    'm', 'N', 'n', 'Ò', 'ò', 'Ǒ', 'ǒ', 'O', 'o', 'Ó', 'ó', 'Ɔ', 'ɔ', 'P', 'p', 'R',
                    'r', 'S', 's', 'T', 't', 'Ú', 'ú', 'Ǔ', 'ǔ', 'Ù', 'ù', 'U', 'u', 'V', 'v', 'W',
                    'w', 'X', 'x', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::French, //+
                [
                    'Æ', 'æ', 'À', 'à', 'Â', 'â', 'A', 'a', 'B', 'b', 'Ç', 'ç', 'C', 'c', 'D', 'd',
                    'É', 'é', 'E', 'e', 'Ê', 'ê', 'È', 'è', 'Ë', 'ë', 'F', 'f', 'G', 'g', 'H', 'h',
                    'Î', 'î', 'Ï', 'ï', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'Ô', 'ô', 'O', 'o', 'Œ', 'œ', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't',
                    'Û', 'û', 'Ù', 'ù', 'Ü', 'ü', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y',
                    'Ÿ', 'ÿ', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::Friulian, //++
                [
                    'A', 'a', 'Â', 'â', 'À', 'à', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'Ê', 'ê',
                    'È', 'è', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'Ì', 'ì', 'Î', 'î',
                    'J', 'j', 'L', 'l', 'M', 'm', 'N', 'n', 'Ò', 'ò', 'Ô', 'ô', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'Ù', 'ù', 'Û', 'û', 'U', 'u', 'V', 'v', 'Z',
                    'z',
                    // loanwords 'K', 'k', 'Y', 'y', 'W', 'w', 'X', 'x'
                ]
            ),
            (
                ScriptLanguage::FulfuldeNigerian, //+
                [
                    'A', 'a', 'B', 'b', 'Ɓ', 'ɓ', 'C', 'c', 'D', 'd', 'Ɗ', 'ɗ', 'E', 'e', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'Ŋ', 'ŋ', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Ƴ', 'ƴ',
                ]
            ),
            (
                ScriptLanguage::GaelicScottish, //+
                [
                    'À', 'à', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'È', 'è', 'F', 'f',
                    'G', 'g', 'H', 'h', 'Ì', 'ì', 'I', 'i', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ò', 'ò', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'Ù', 'ù', 'U', 'u',
                ]
            ),
            (
                ScriptLanguage::Galician, //+
                [
                    'Á', 'á', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'É', 'é', 'E', 'e', 'F', 'f',
                    'G', 'g', 'H', 'h', 'Í', 'í', 'I', 'i', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ',
                    'Ó', 'ó', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                    'Ú', 'ú', 'V', 'v', 'X', 'x', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::Ganda, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ŋ', 'ŋ', 'O', 'o',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::German, //+
                [
                    'Ä', 'ä', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ö', 'ö', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'ẞ', 'ß', 'T', 't', 'Ü', 'ü',
                    'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::Guarani, //+
                [
                    'Ã', 'ã', 'Á', 'á', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'Ẽ', 'ẽ', 'Ê', 'ê',
                    'E', 'e', 'É', 'é', 'G', 'g', 'H', 'h', 'Í', 'í', 'Ĩ', 'ĩ', 'Î', 'î', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'Ó', 'ó', 'O', 'o',
                    'Õ', 'õ', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'Ú', 'ú', 'U', 'u', 'Ũ', 'ũ',
                    'V', 'v', 'Ý', 'ý', 'Ỹ', 'ỹ', 'Y', 'y',
                ]
            ),
            (
                ScriptLanguage::Hausa,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Ɓ', 'ɓ', 'Ɗ', 'ɗ', 'Ƙ', 'ƙ',
                    'Ƴ', 'ƴ', /*unverified*/ 'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::Hawaiian, //+
                [
                    'A', 'a', 'E', 'e', 'I', 'i', 'O', 'o', 'U', 'u', 'H', 'h', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'P', 'p', 'W', 'w', 'Ā', 'ā', 'Ē', 'ē', 'Ī', 'ī', 'Ō', 'ō',
                    'Ū', 'ū'
                ]
            ),
            (
                ScriptLanguage::Hungarian, //++
                [
                    'A', 'a', 'Á', 'á', 'B', 'b', 'C', 'c', /* 'Cs', 'cs', */ 'D', 'd',
                    /* 'Dz', 'dz', 'Dzs', 'dzs', */ 'E', 'e', 'É', 'é', 'F', 'f', 'G', 'g',
                    /* 'Gy', 'gy', */ 'H', 'h', 'I', 'i', 'Í', 'í', 'J', 'j', 'K', 'k', 'L',
                    'l', /* 'Ly', 'ly', */ 'M', 'm', 'N', 'n', /* 'Ny', 'ny', */ 'O',
                    'o', 'Ó', 'ó', 'Ö', 'ö', 'Ő', 'ő', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's',
                    /* 'Sz', 'sz', */ 'T', 't', /* 'Ty', 'ty', */ 'U', 'u', 'Ú', 'ú',
                    'Ü', 'ü', 'Ű', 'ű', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z',
                    'z', /* 'Zs', 'zs' */
                ]
            ),
            (
                ScriptLanguage::Icelandic, //++
                [
                    'A', 'a', 'Á', 'á', 'Æ', 'æ', 'B', 'b', 'D', 'd', 'Ð', 'ð', 'E', 'e', 'É', 'é',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'Í', 'í', 'J', 'j', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'O', 'o', 'Ó', 'ó', 'Ö', 'ö', 'P', 'p', 'R', 'r', 'S', 's',
                    'T', 't', 'U', 'u', 'Ú', 'ú', 'V', 'v', 'X', 'x', 'Y', 'y', 'Ý', 'ý', 'Þ',
                    'þ',
                    // obsolete 'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::Igbo,
                [
                    'A', 'a', 'B', 'b', /* 'Ch', 'ch', */ 'C', 'c', 'D', 'd', 'E', 'e', 'F',
                    'f', 'G', 'g', /* 'Gb', 'gb', 'Gh', 'gh', */ 'H', 'h', 'I', 'i', 'Ị', 'ị',
                    'J', 'j', 'K', 'k', /* 'Kw', 'kw', */ 'L', 'l', 'M', 'm', 'N', 'n',
                    /* 'Nw', 'nw', */ 'O', 'o', 'Ọ', 'ọ', 'P', 'p', 'R', 'r', 'S', 's', 'T',
                    't', 'U', 'u', 'Ụ', 'ụ', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z', 'Á', 'á', 'É',
                    'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú', 'Ṅ', 'ṅ', 'Ŋ', 'ŋ'
                ]
            ),
            (
                ScriptLanguage::Ilocano, //++
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'G', 'g', 'H', 'h', 'I', 'i', 'K', 'k',
                    'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't',
                    'U', 'u', 'W', 'w', 'Y', 'y',
                    /*'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú',*/
                    // Spanish-based
                    'C', 'c', 'F', 'f', 'J', 'j', 'Ñ', 'ñ', 'V', 'v', 'Z',
                    'z',
                    // loanwords Q, X,
                ]
            ),
            (
                ScriptLanguage::Indonesian,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::Irish, //++
                [
                    'A', 'a', 'Á', 'á', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'É', 'é', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'Í', 'í', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ó', 'ó', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'Ú', 'ú',
                ]
            ),
            (
                ScriptLanguage::Italian, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z', 'À', 'à', 'È', 'è',
                    'É', 'é', 'Ì', 'ì', 'Í', 'í', 'Ò', 'ò', 'Ù', 'ù', 'Ú',
                    'ú',
                    // loanwords 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                ScriptLanguage::Javanese,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', /*unverified*/
                    'É', 'é', 'È', 'è'
                ]
            ),
            (
                ScriptLanguage::Jingpho,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::Kabiye, //++
                #[rustfmt::skip]
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'Ð', 'Ɖ', 'ɖ', 'E', 'e', 'Ɛ', 'ɛ', 'F',
                    'f', 'G', 'g', 'H', 'h', 'I', 'i', 'Ɩ', 'ɩ', 'J', 'j', 'K', 'k', 'L', 'l', 'M',
                    'm', 'N', 'n', 'Ñ', 'ñ', 'Ŋ', 'ŋ', 'O', 'o', 'Ɔ', 'ɔ', 'P', 'p', 'R', 'r', 'S',
                    's', 'T', 't', 'U', 'u', 'V', 'v', 'Ʋ', 'ʋ', 'W', 'w', 'Y', 'y', 'Z', 'z', 'Ɣ',
                    'ɣ', 'Ɑ', 'ɑ', 'Ʊ', 'ʊ', 'É', 'é',
                    // from `char_compose_custom`
                    '\u{f046e}', // N̄n̄
                ]
            ),
            (
                ScriptLanguage::Kabuverdianu,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'X', 'x', 'Z', 'z',
                    'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú', 'Õ', 'õ', 'Ç', 'ç', 'Ê', 'ê'
                ]
            ),
            (
                ScriptLanguage::Kabyle, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'Ḍ', 'ḍ', 'E', 'e', 'F', 'f',
                    'G', 'g', /* 'Ğ', 'ğ', */ 'Ɣ', 'ɣ', 'H', 'h', 'Ḥ', 'ḥ', 'I', 'i', 'J',
                    'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Q', 'q', 'R', 'r', 'Ṛ', 'ṛ', 'S',
                    's', 'Ṣ', 'ṣ', 'T', 't', 'Ṭ', 'ṭ', 'U', 'u', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z',
                    'z', 'Ẓ', 'ẓ', 'Ɛ', 'ɛ',
                    // loanwords O, P
                ]
            ),
            (
                ScriptLanguage::Kamba, //++
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'G', 'g', 'H', 'h', 'Ĩ', 'ĩ', 'I', 'i',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'S', 's', 'T', 't', 'U', 'u',
                    'Ũ', 'ũ', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::KanuriCentral,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y',
                    /*unverified*/ 'Z', 'z', 'Ə', 'ə'
                ]
            ),
            (
                ScriptLanguage::Kikongo,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', /*unverified*/
                    'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::Kikuyu, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'G', 'g', 'H', 'h', 'I', 'i',
                    'Ĩ', 'ĩ', 'J', 'j', 'K', 'k', 'M', 'm', 'N', 'n', 'O', 'o', 'R', 'r', 'T', 't',
                    'U', 'u', 'Ũ', 'ũ', 'W', 'w', 'Y', 'y', //
                    // extra
                    'F', 'f', 'L', 'l', 'S', 's',
                    // must be absent P, Q, V, X, Z
                ]
            ),
            (
                ScriptLanguage::Kimbundu, //++
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'S', 's',
                    'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z',
                    // not sure about diacritics
                    'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú', 'À', 'à', 'È', 'è', 'Â', 'â',
                    'Ê', 'ê', 'Ô', 'ô'
                ]
            ),
            (
                ScriptLanguage::Kinyarwanda,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Í', 'í', 'Ú',
                    'ú', /*unverified*/
                    'V', 'v', 'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::KurdishNorthern,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'Ê', 'ê', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'Î', 'î', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'Ş', 'ş', 'T', 't',
                    'U', 'u', 'Û', 'û', 'V', 'v', 'X', 'x', 'Y', 'y', 'Z', 'z',
                    /*unverified*/ 'W', 'w'
                ]
            ),
            (
                ScriptLanguage::Latgalian, //++
                [
                    'A', 'a', 'Ā', 'ā', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'Ē', 'ē',
                    'F', 'f', 'G', 'g', 'Ģ', 'ģ', 'H', 'h', 'I', 'i', 'Y', 'y', 'Ī', 'ī', 'J', 'j',
                    'K', 'k', 'Ķ', 'ķ', 'L', 'l', 'Ļ', 'ļ', 'M', 'm', 'N', 'n', 'Ņ', 'ņ', 'O', 'o',
                    'Ō', 'ō', 'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'Ū', 'ū',
                    'V', 'v', 'Z', 'z', 'Ž', 'ž',
                ]
            ),
            (
                ScriptLanguage::Latin, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'X', 'x', 'Y', 'y', 'Z', 'z',
                    'Ā', 'ā', 'Ē', 'ē', 'Ī', 'ī', 'Ō', 'ō', 'Ū', 'ū', 'Æ', 'æ', 'Œ', 'œ'
                ]
            ),
            (
                ScriptLanguage::Latvian, //++
                [
                    'A', 'a', 'Ā', 'ā', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'Ē', 'ē',
                    'F', 'f', 'G', 'g', 'Ģ', 'ģ', 'H', 'h', 'I', 'i', 'Ī', 'ī', 'J', 'j', 'K', 'k',
                    'Ķ', 'ķ', 'L', 'l', 'Ļ', 'ļ', 'M', 'm', 'N', 'n', 'Ņ', 'ņ', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'Ū', 'ū', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž',
                ]
            ),
            (
                ScriptLanguage::Ligurian, //++
                [
                    'A', 'a', 'Æ', 'æ', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'X', 'x', 'Z', 'z',
                    'À', 'à', 'Â', 'â', 'É', 'é', 'È', 'è', 'Ê', 'ê', 'Ì', 'ì', 'Î', 'î', 'Ò', 'ò',
                    'Ô', 'ô', 'Ö', 'ö', 'Ù', 'ù', 'Û', 'û',
                    // loanwords 'J', 'j', 'Ó', 'ó',
                ]
            ),
            (
                ScriptLanguage::Limburgish,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Á', 'á', 'É', 'é', 'È', 'è', 'Ë', 'ë', 'Ì', 'ì', 'Í', 'í',
                    'Î', 'î', 'Ó', 'ó', 'Ô', 'ô', 'Ò', 'ò', 'Ú', 'ú', 'Ù', 'ù', 'Û', 'û', 'Ä', 'ä',
                    'Ö', 'ö', 'Ü', 'ü'
                ]
            ),
            (
                ScriptLanguage::Lingala, //++
                #[rustfmt::skip]
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'Ɔ', 'ɔ',
                    'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y',
                    'Z', 'z', 'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú', 'Â', 'â', 'Ê', 'ê',
                    'Î', 'î', 'Ô', 'ô', 'Ǎ', 'ǎ', 'Ě', 'ě', 'Ǐ', 'ǐ', 'Ǒ', 'ǒ',
                    // from `char_compose_custom`
                    '\u{f0154}', // Ɔ́ɔ́
                    '\u{f015b}', // Ɛ́ɛ́
                    '\u{f0254}', // Ɔ̂ɔ̂
                    '\u{f025b}', // Ɛ̂ɛ̂
                    '\u{f0c54}', // Ɔ̌ɔ̌
                    '\u{f0c5b}', // Ɛ̌ɛ̌
                    // loanwords J, X
                ]
            ),
            (
                ScriptLanguage::Lithuanian, //++
                [
                    'A', 'a', 'Ą', 'ą', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'Ę', 'ę',
                    'Ė', 'ė', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'Į', 'į', 'Y', 'y', 'J', 'j',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'Š', 'š', 'T', 't', 'U', 'u', 'Ų', 'ų', 'Ū', 'ū', 'V', 'v', 'Z', 'z', 'Ž', 'ž',
                ]
            ),
            (
                ScriptLanguage::Lombard, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z', 'À', 'à', 'Á', 'á',
                    'È', 'è', 'É', 'é', 'Ì', 'ì', 'Í', 'í', 'Ò', 'ò', 'Ó', 'ó', 'Ô', 'ô', 'Ø', 'ø',
                    'Ö', 'ö', 'Ù', 'ù', 'Ü', 'ü', 'Œ', 'œ',
                ]
            ),
            (
                ScriptLanguage::LubaKasai,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y',
                    'y', /*unverified*/
                    'Z', 'z', 'Î', 'î', 'Ê', 'ê',
                ]
            ),
            (
                ScriptLanguage::Luxembourgish,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Ä', 'ä', 'É', 'é', 'Ë', 'ë', 'Ó', 'ó', 'Ö', 'ö', 'Ü',
                    'ü', /*unverified*/
                    'È', 'è'
                ]
            ),
            (
                ScriptLanguage::Malay, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::MalgasyPlateau,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'V', 'v', 'Y', 'y', 'Z', 'z', /*unverified*/ 'À', 'à',
                    'Ô', 'ô'
                ]
            ),
            (
                ScriptLanguage::Maltese,
                [
                    'A', 'a', 'B', 'b', 'Ċ', 'ċ', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    /* 'Għ', 'għ', */ 'H', 'h', 'Ħ', 'ħ', 'I', 'i',
                    /* 'Ie', 'ie', */ 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O',
                    'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W',
                    'w', 'X', 'x', 'Ż', 'ż', 'Z', 'z', 'Ġ', 'ġ', /*unverified*/ 'À', 'à', 'C',
                    'c',
                ]
            ),
            (
                ScriptLanguage::Mandailing, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y',
                ]
            ),
            (
                ScriptLanguage::Maori, //++
                [
                    'A', 'a', 'E', 'e', 'H', 'h', 'I', 'i', 'K', 'k', 'M', 'm', 'N', 'n', 'O', 'o',
                    'P', 'p', 'R', 'r', 'T', 't', 'U', 'u', 'W', 'w', 'G', 'g', 'Ā', 'ā', 'Ē', 'ē',
                    'Ī', 'ī', 'Ō', 'ō', 'Ū', 'ū', //
                    // Southern dialect
                    'L', 'l', 'Ḵ', 'ḵ',
                ]
            ),
            (
                ScriptLanguage::Minangkabau,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::Mizo, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'Ṭ', 'ṭ', 'U', 'u', 'V', 'v', 'Z', 'z', 'W', 'w',
                    'Ǎ', 'ǎ', 'Ă', 'ă', 'Ả', 'ả', 'Ȧ', 'ȧ', 'Ã', 'ã', 'Ą', 'ą', 'Ạ', 'ạ', 'Â', 'â',
                    'Á', 'á', 'Ä', 'ä', 'À', 'à', 'Ǒ', 'ǒ', 'Ŏ', 'ŏ', 'Ỏ', 'ỏ', 'Ó', 'ó', 'Ọ', 'ọ',
                    'Ò', 'ò', 'Ǔ', 'ǔ', 'Ŭ', 'ŭ', 'Ủ', 'ủ', 'Ů', 'ů', 'Ũ', 'ũ', 'Ų', 'ų', 'Ụ', 'ụ',
                    'Û', 'û', 'Ú', 'ú', 'Ü', 'ü', 'Ù', 'ù', 'Ě', 'ě', 'Ĕ', 'ĕ', 'Ẻ', 'ẻ', 'Ė', 'ė',
                    'Ẽ', 'ẽ', 'Ę', 'ę', 'Ẹ', 'ẹ', 'Ê', 'ê', 'É', 'é', 'Ë', 'ë', 'È', 'è', 'Ǐ', 'ǐ',
                    'Ĭ', 'ĭ', 'Ỉ', 'ỉ', 'Ĩ', 'ĩ', 'Į', 'į', 'Ị', 'ị', 'Î', 'î', 'Í', 'í', 'Ï', 'ï',
                    'Ì', 'ì',
                ]
            ),
            (
                ScriptLanguage::Mossi, //++
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'Ɩ', 'ɩ', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'Ʋ', 'ʋ', 'V', 'v', 'W', 'w', 'Y', 'y',
                    'Z', 'z', 'Ã', 'ã', 'Ẽ', 'ẽ', 'Ĩ', 'ĩ', 'Õ', 'õ', 'Ũ', 'ũ', 'É',
                    'é',
                    // any more diacritics?
                ]
            ),
            (
                ScriptLanguage::NorwegianBokmal, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Æ', 'æ', 'Ø', 'ø', 'Å', 'å', 'É', 'é', 'Ó', 'ó', 'Ô', 'ô',
                ]
            ),
            (
                ScriptLanguage::NorwegianNynorsk, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Æ', 'æ', 'Ø', 'ø', 'Å', 'å', 'É', 'é', 'Ó', 'ó', 'Ò', 'ò',
                    'Ô', 'ô', 'Ê', 'ê',
                ]
            ),
            (
                ScriptLanguage::Nuer, //++
                #[rustfmt::skip]
                [
                    'A', 'a', 'Ä', 'ä', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ë', 'ë', 'Ɛ', 'ɛ',
                    'G', 'g', 'Ɣ', 'ɣ', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'Ŋ', 'ŋ', 'O', 'o', 'Ö', 'ö', 'Ɔ', 'ɔ', 'P', 'p', 'R', 'r', 'T', 't',
                    'U', 'u', 'W', 'w', 'Y', 'y',
                    // from `char_compose_custom`
                    '\u{f0890}', // Ɛ̈ɛ̈
                    '\u{f3154}', // Ɔ̱ɔ̱
                    '\u{f3161}', // A̱a̱
                    '\u{f3165}', // E̱e̱
                    '\u{f3169}', // I̱i̱
                    '\u{f316f}', // O̱o̱
                    '\u{f3190}', // Ɛ̱̈ɛ̱̈
                ]
            ),
            (
                ScriptLanguage::Nyanja,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Z',
                    'z', /*unverified*/
                    'V', 'v', 'Ŵ', 'ŵ'
                ]
            ),
            (
                ScriptLanguage::Occitan,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Á', 'á', 'È', 'è', 'É', 'é', 'Í', 'í', 'Ò', 'ò', 'Ó', 'ó',
                    'Ú', 'ú', 'Ç', 'ç'
                ]
            ),
            (
                ScriptLanguage::OromoSouthern, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::OromoWestCentral,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'X', 'x', 'Y',
                    'y', /*unverified*/
                    'Z', 'z', 'V', 'v'
                ]
            ),
            (
                ScriptLanguage::Pangasinan, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::Papiamento,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y',
                    'Z', 'z', 'Á', 'á', 'É', 'é', 'È', 'è', 'Ó', 'ó', 'Ú', 'ú', 'Ü', 'ü', 'Ñ',
                    'ñ', /*unverified*/
                    'Í', 'í', 'Ò', 'ò'
                ]
            ),
            (
                ScriptLanguage::Polish,
                [
                    'A', 'a', 'Ą', 'ą', 'B', 'b', 'C', 'c', 'Ć', 'ć', 'D', 'd', 'E', 'e', 'Ę', 'ę',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'Ł', 'ł',
                    'M', 'm', 'N', 'n', 'Ń', 'ń', 'O', 'o', 'Ó', 'ó', 'P', 'p', 'R', 'r', 'S', 's',
                    'Ś', 'ś', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Z', 'z', 'Ź', 'ź', 'Ż', 'ż',
                ]
            ),
            (
                ScriptLanguage::Portuguese, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Á', 'á', 'Â', 'â', 'Ã', 'ã', 'À', 'à', 'Ç', 'ç', 'É', 'é',
                    'Ê', 'ê', 'Í', 'í', 'Ó', 'ó', 'Ô', 'ô', 'Õ', 'õ', 'Ò', 'ò', 'Ú',
                    'ú',
                    // formerly: 'Ï', 'ï', 'Ü', 'ü', 'È', 'è', 'Ì', 'ì', 'Ù', 'ù',
                ]
            ),
            (
                ScriptLanguage::Pular, //+
                [
                    'A', 'a', 'B', 'b', 'Ɓ', 'ɓ', 'C', 'c', 'D', 'd', 'Ɗ', 'ɗ', 'E', 'e', 'F', 'f',
                    'G', 'g', 'Ɠ', 'ɠ', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'Ñ', 'ñ', 'Ŋ', 'ŋ', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't',
                    'U', 'u', 'W', 'w', 'Y', 'y', 'Ƴ', 'ƴ',
                ]
            ),
            (
                ScriptLanguage::QuechuaAyacucho, //++
                [
                    'A', 'a', 'C', 'c', 'H', 'h', 'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'Ñ', 'ñ', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w',
                    'Y', 'y', 'B', 'b', 'D', 'd', 'E', 'e', 'J', 'j', 'O', 'o', 'V', 'v',
                ]
            ),
            (
                ScriptLanguage::Romanian,
                [
                    'A', 'a', 'Ă', 'ă', 'Â', 'â', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'Î', 'î', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'Ș', 'ș', 'T', 't',
                    'Ț', 'ț', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z', 'Ş', 'ş',
                    'Ţ', 'ţ',
                ]
            ),
            (
                ScriptLanguage::Rundi,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y',
                    'y', /*unverified*/
                    'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::Samoan,
                [
                    'A', 'a', 'E', 'e', 'F', 'f', 'G', 'g', 'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'P', 'p', 'S', 's', 'T', 't', 'U', 'u', 'V',
                    'v', /*unverified*/
                    'Ā', 'ā', 'Ē', 'ē', 'R', 'r'
                ]
            ),
            (
                ScriptLanguage::Sango, //++
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z', 'Â', 'â', 'Ê', 'ê',
                    'Î', 'î', 'Ô', 'ô', 'Û', 'û', 'Ä', 'ä', 'Ë', 'ë', 'Ï', 'ï', 'Ö', 'ö', 'Ü', 'ü',
                    'É', 'é',
                ]
            ),
            (
                ScriptLanguage::Sardinian,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'À', 'à', 'É', 'é', 'È', 'è', 'Í', 'í', 'Ì', 'ì', 'Ó', 'ó',
                    'Ò', 'ò', 'Ú', 'ú', 'Ù', 'ù'
                ]
            ),
            (
                ScriptLanguage::Sepedi,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ê', 'ê', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ô', 'ô', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Z', 'z', 'Š', 'š'
                ]
            ),
            (
                ScriptLanguage::Sesotho, //+ including Lesotho
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y',
                    'Z', 'z', 'Š', 'š', 'Ò', 'ò', 'Ō', 'ō', 'È', 'è', 'Ē', 'ē'
                ]
            ),
            (
                ScriptLanguage::Shona,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z',
                    'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú'
                ]
            ),
            (
                ScriptLanguage::Sicilian, //++
                [
                    'A', 'a', 'À', 'à', 'Â', 'â', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'Ḍ', 'ḍ',
                    'Đ', 'đ', 'E', 'e', 'È', 'è', 'Ê', 'ê', 'Ë', 'ë', 'F', 'f', 'G', 'g', 'Ġ', 'ġ',
                    'H', 'h', 'Ḥ', 'ḥ', 'I', 'i', 'Ì', 'ì', 'Í', 'í', 'Î', 'î', 'Ï', 'ï', 'J', 'j',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ṅ', 'ṅ', 'O', 'o', 'Ò', 'ò', 'Ô', 'ô',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'Ṛ', 'ṛ', 'S', 's', 'Š', 'š', 'Ṣ', 'ṣ', 'T', 't',
                    'Ṭ', 'ṭ', 'U', 'u', 'Ù', 'ù', 'Ú', 'ú', 'Û', 'û', 'V', 'v', 'X', 'x', 'Y', 'y',
                    'Z', 'z', 'Ż',
                    'ż',
                    /* 'É', 'é' ?*/
                    // loanwords 'W', 'w',
                ]
            ),
            (
                ScriptLanguage::Silesian, //++
                [
                    'A', 'a', 'Ã', 'ã', 'B', 'b', 'C', 'c', 'Ć', 'ć', 'Č', 'č', 'D', 'd', 'E', 'e',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'Ł', 'ł',
                    'M', 'm', 'N', 'n', 'Ń', 'ń', 'O', 'o', 'Ŏ', 'ŏ', 'Ō', 'ō', 'Ô', 'ô', 'Õ', 'õ',
                    'P', 'p', 'R', 'r', 'Ř', 'ř', 'S', 's', 'Ś', 'ś', 'Š', 'š', 'T', 't', 'U', 'u',
                    'Ů', 'ů', 'W', 'w', 'Y', 'y', 'Z', 'z', 'Ź', 'ź', 'Ż', 'ż', 'Ž', 'ž',
                ]
            ),
            (
                ScriptLanguage::Slovak, //++
                [
                    'A', 'a', 'Á', 'á', 'Ä', 'ä', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'Ď', 'ď',
                    'E', 'e', 'É', 'é', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'Í', 'í', 'J', 'j',
                    'K', 'k', 'L', 'l', 'Ĺ', 'ĺ', 'Ľ', 'ľ', 'M', 'm', 'N', 'n', 'Ň', 'ň', 'O', 'o',
                    'Ó', 'ó', 'Ô', 'ô', 'P', 'p', 'R', 'r', 'Ŕ', 'ŕ', 'S', 's', 'Š', 'š', 'T', 't',
                    'Ť', 'ť', 'U', 'u', 'Ú', 'ú', 'V', 'v', 'X', 'x', 'Y', 'y', 'Ý', 'ý', 'Z', 'z',
                    'Ž', 'ž',
                    // loanwords Q, W
                ]
            ),
            (
                ScriptLanguage::Slovene,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž',
                ]
            ),
            (
                ScriptLanguage::Somali,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'Q', 'q',
                    'R', 'r', 'S', 's', /* 'Sh', 'sh', */ 'T', 't', 'U', 'u', 'W', 'w', 'X',
                    'x', 'Y', 'y', /*unverified*/
                    'P', 'p', 'V', 'v'
                ]
            ),
            (
                ScriptLanguage::Spanish,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'O', 'o',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Z', 'z', 'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú',
                    'Ü', 'ü'
                ]
            ),
            (
                ScriptLanguage::Sundanese,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', /*unverified*/ 'É', 'é'
                ]
            ),
            (
                ScriptLanguage::Swahili,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::Swati,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', /*unverified*/
                    'Z', 'z', 'R', 'r'
                ]
            ),
            (
                ScriptLanguage::Swedish,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Å', 'å', 'Ä', 'ä', 'Ö', 'ö',
                ]
            ),
            (
                ScriptLanguage::Tagalog,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    /* 'Ng', 'ng', */ 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T',
                    't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::TamasheqLatin, //++
                [
                    'A', 'a', 'Ă', 'ă', 'Ǎ', 'ǎ', 'Ə', 'ə', 'B', 'b', 'C', 'c', 'D', 'd', 'Ḍ', 'ḍ',
                    'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g', 'Ɣ', 'ɣ', 'H', 'h', 'Ḥ', 'ḥ', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'Ḷ', 'ḷ', 'M', 'm', 'N', 'n', 'Ŋ', 'ŋ', 'O', 'o',
                    'Q', 'q', 'R', 'r', 'S', 's', 'Ṣ', 'ṣ', 'Š', 'š', 'T', 't', 'Ṭ', 'ṭ', 'U', 'u',
                    'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z', 'Ẓ', 'ẓ', 'Ž', 'ž', 'Ɗ',
                    'ɗ',
                    /* 'P', 'p' ?*/
                ]
            ),
            (
                ScriptLanguage::TatarCrimean,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'ı', 'İ', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'O', 'o', 'Ö', 'ö', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'Ş', 'ş', 'T', 't',
                    'U', 'u', 'Ü', 'ü', 'V', 'v', 'Y', 'y', 'Z', 'z', 'Ğ', 'ğ', 'Ə',
                    'ə', /*unverified*/
                    'Ñ', 'ñ', 'Â', 'â'
                ]
            ),
            (
                ScriptLanguage::TokPisin,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z',
                    'z', /*unverified*/
                    'C', 'c',
                ]
            ),
            (
                ScriptLanguage::Tsonga, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::Tswana,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                ScriptLanguage::Tumbuka,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z',
                    'z', /*unverified*/
                    'Ŵ', 'ŵ'
                ]
            ),
            (
                ScriptLanguage::Turkish, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'Ğ', 'ğ', 'H', 'h', 'I', 'ı', 'İ', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'Ö', 'ö', 'P', 'p', 'R', 'r', 'S', 's', 'Ş', 'ş', 'T', 't',
                    'U', 'u', 'Ü', 'ü', 'V', 'v', 'Y', 'y', 'Z', 'z', 'Q', 'q', 'W', 'w', 'X', 'x',
                    'Â', 'â', 'Î', 'î', 'Û', 'û'
                ]
            ),
            (
                ScriptLanguage::Turkmen, //++
                [
                    'A', 'a', 'B', 'b', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ň', 'ň', 'O', 'o',
                    'Ö', 'ö', 'P', 'p', 'R', 'r', 'S', 's', 'Ş', 'ş', 'T', 't', 'U', 'u', 'Ü', 'ü',
                    'W', 'w', 'Y', 'y', 'Ý', 'ý', 'Z', 'z', 'Ä', 'ä', 'Ž', 'ž',
                ]
            ),
            (
                ScriptLanguage::Twi, //++
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'Ɔ', 'ɔ', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Ã', 'ã', 'Á', 'á',
                    'Ẽ', 'ẽ', 'Ĩ', 'ĩ', 'Õ', 'õ', 'Ũ', 'ũ', // any more diacritics?
                ]
            ),
            (
                ScriptLanguage::Umbundu, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Ã', 'ã',
                    'Ẽ', 'ẽ', 'Ĩ', 'ĩ', 'Õ', 'õ', 'Ũ', 'ũ', 'Ñ', 'ñ', // any more diacritics?
                ]
            ),
            (
                ScriptLanguage::UzbekNorthern,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'X', 'x', 'Y', 'y', 'Z',
                    'z', /*unverified*/
                    'C', 'c'
                ]
            ),
            (
                ScriptLanguage::Venetian,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'À', 'à', 'È', 'è', 'É', 'é', 'Ì', 'ì', 'Ó', 'ó', 'Ù', 'ù',
                    'Ç', 'ç', /*unverified*/ 'Ł', 'ł'
                ]
            ),
            (
                ScriptLanguage::Vietnamese,
                [
                    'A', 'a', 'Ă', 'ă', 'Â', 'â', 'B', 'b', 'C', 'c', 'D', 'd', 'Đ', 'đ', 'E', 'e',
                    'Ê', 'ê', 'G', 'g', 'H', 'h', 'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'O', 'o', 'Ô', 'ô', 'Ơ', 'ơ', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't',
                    'U', 'u', 'Ư', 'ư', 'V', 'v', 'X', 'x', 'Y', 'y', 'Á', 'á', 'À', 'à', 'Ả', 'ả',
                    'Ã', 'ã', 'Ạ', 'ạ', 'Ắ', 'ắ', 'Ằ', 'ằ', 'Ẳ', 'ẳ', 'Ẵ', 'ẵ', 'Ặ', 'ặ', 'Ấ', 'ấ',
                    'Ầ', 'ầ', 'Ẩ', 'ẩ', 'Ẫ', 'ẫ', 'Ậ', 'ậ', 'É', 'é', 'È', 'è', 'Ẻ', 'ẻ', 'Ẽ', 'ẽ',
                    'Ẹ', 'ẹ', 'Ế', 'ế', 'Ề', 'ề', 'Ể', 'ể', 'Ễ', 'ễ', 'Ệ', 'ệ', 'Í', 'í', 'Ì', 'ì',
                    'Ỉ', 'ỉ', 'Ĩ', 'ĩ', 'Ị', 'ị', 'Ó', 'ó', 'Ò', 'ò', 'Ỏ', 'ỏ', 'Õ', 'õ', 'Ọ', 'ọ',
                    'Ố', 'ố', 'Ồ', 'ồ', 'Ổ', 'ổ', 'Ỗ', 'ỗ', 'Ộ', 'ộ', 'Ớ', 'ớ', 'Ờ', 'ờ', 'Ở', 'ở',
                    'Ỡ', 'ỡ', 'Ợ', 'ợ', 'Ú', 'ú', 'Ù', 'ù', 'Ủ', 'ủ', 'Ũ', 'ũ', 'Ụ', 'ụ', 'Ứ', 'ứ',
                    'Ừ', 'ừ', 'Ử', 'ử', 'Ữ', 'ữ', 'Ự', 'ự', 'Ý', 'ý', 'Ỳ', 'ỳ', 'Ỷ', 'ỷ', 'Ỹ', 'ỹ',
                    'Ỵ', 'ỵ'
                ]
            ),
            (
                ScriptLanguage::Waray, //++
                [
                    'A', 'a', 'B', 'b', 'K', 'k', 'D', 'd', 'G', 'g', 'H', 'h', 'I', 'i', 'L', 'l',
                    'M', 'm', 'N', 'n', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w',
                    'Y', 'y', 'E', 'e', 'O', 'o',
                    // loanwords J, V, C, F and others
                ]
            ),
            (
                ScriptLanguage::Welsh, //++
                [
                    'A', 'a', 'B', 'b', 'C', 'c', /* 'Ch', 'ch', */ 'D', 'd',
                    /* 'Dd', 'dd', */ 'E', 'e', 'F', 'f', /* 'Ff', 'ff', */ 'G', 'g',
                    /* 'Ng', 'ng', */ 'H', 'h', 'I', 'i', 'J', 'j', 'L', 'l',
                    /* 'Ll', 'll', */ 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    /* 'Rh', 'rh', */ 'S', 's', 'T', 't', /* 'Th', 'th', */ 'U', 'u',
                    'W', 'w', 'Y', 'y', 'Â', 'â', 'Ê', 'ê', 'Î', 'î', 'Ô', 'ô', 'Û', 'û', 'Ŵ', 'ŵ',
                    'Ŷ', 'ŷ', 'À', 'à', 'È', 'è', 'Ì', 'ì', 'Ò', 'ò', 'Ù', 'ù', 'Ẁ', 'ẁ', 'Ỳ', 'ỳ',
                    'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú', 'Ẃ', 'ẃ', 'Ý', 'ý', 'Ë', 'ë',
                    'Ï', 'ï', 'Ü', 'ü', 'Ÿ',
                    'ÿ',
                    // loanwords 'K', 'k', 'Q', 'q', 'V', 'v', 'X', 'x', 'Z', 'z',
                ]
            ),
            (
                ScriptLanguage::Wolof, //++
                [
                    'A', 'a', 'À', 'à', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'É', 'é', 'Ë', 'ë',
                    'F', 'f', 'G', 'g', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'Ñ', 'ñ', 'Ŋ', 'ŋ', 'O', 'o', 'Ó', 'ó', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's',
                    'T', 't', 'U', 'u', 'W', 'w', 'X', 'x', 'Y', 'y',
                    // loanwords H, V, Z
                ]
            ),
            (
                ScriptLanguage::Xhosa, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                ScriptLanguage::Yoruba, //++
                #[rustfmt::skip]
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'Ẹ', 'ẹ', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'Ọ', 'ọ',
                    'P', 'p', 'R', 'r', 'S', 's', 'Ṣ', 'ṣ', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y',
                    'Á', 'á', 'À', 'à', 'Ā', 'ā', 'É', 'é', 'È', 'è', 'Ē', 'ē', 'Í', 'í', 'Ì', 'ì',
                    'Ī', 'ī', 'Ń', 'ń', 'Ǹ', 'ǹ', 'Ḿ', 'ḿ', 'Ó', 'ó', 'Ò', 'ò', 'Ō', 'ō', 'Ú', 'ú',
                    'Ù', 'ù', 'Ū', 'ū',
                    // from `char_compose_custom`
                    '\u{f006d}', // M̀m̀
                    '\u{f00b9}', // Ẹ̀ẹ̀
                    '\u{f00cd}', // Ọ̀ọ̀
                    '\u{f01b9}', // Ẹ́ẹ́
                    '\u{f01cd}', // Ọ́ọ́
                    '\u{f046d}', // M̄m̄
                    '\u{f046e}', // N̄n̄
                    '\u{f04b9}', // Ẹ̄ẹ̄
                    '\u{f04cd}', // Ọ̄ọ̄
                ]
            ),
            (
                ScriptLanguage::Zulu,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
        ]),
        Lepcha => &[ScriptLanguage::Lepcha],
        Limbu => &[ScriptLanguage::Limbu],
        LinearA => &[ScriptLanguage::MinoanLinearA],
        LinearB => &[ScriptLanguage::MycenaeanGreek],
        Lisu => &[ScriptLanguage::Lisu],
        Lycian => &[ScriptLanguage::Lycian],
        Lydian => &[ScriptLanguage::Lydian],
        Mahajani => &[
            ScriptLanguage::HindiMahajani,
            ScriptLanguage::Marwari,
            ScriptLanguage::PunjabiEasternMahajani,
        ],
        Makasar => &[ScriptLanguage::MakassareseMakasar],
        Malayalam => &[ScriptLanguage::Malayalam],
        Mandaic => &[ScriptLanguage::AramaicMandaic],
        Manichaean => &[
            ScriptLanguage::MiddlePersianManichaean,
            ScriptLanguage::SogdianManichaean,
        ],
        Marchen => &[ScriptLanguage::Zhangzhung],
        MasaramGondi => &[ScriptLanguage::GondiMasaram],
        Medefaidrin => &[ScriptLanguage::Medefaidrin],
        MeeteiMayek => &[ScriptLanguage::MeiteiMeeteiMayek],
        MendeKikakui => &[ScriptLanguage::Mende],
        MeroiticCursive => &[ScriptLanguage::MeroiticCursive],
        MeroiticHieroglyphs => &[ScriptLanguage::MeroiticHieroglyphs],
        Miao => &[ScriptLanguage::HmongMiao],
        Modi => &[ScriptLanguage::MarathiModi],
        Mongolian => &[ScriptLanguage::MongolianHalhMongolian],
        Mro => &[ScriptLanguage::Mro],
        Multani => &[ScriptLanguage::SaraikiMultani],
        NagMundari => &[ScriptLanguage::Mundari],
        Myanmar => &[ScriptLanguage::Burmese, ScriptLanguage::Shan],
        Nabataean => &[ScriptLanguage::AramaicNabataean],
        Nandinagari => &[ScriptLanguage::SanskritNandinagari],
        // maybe add more?
        Newa => &[ScriptLanguage::Newar],
        NewTaiLue => &[ScriptLanguage::TaiLueNew],
        Nko => &[ScriptLanguage::Manding],
        Nushu => &[ScriptLanguage::ChineseTuhua],
        NyiakengPuachueHmong => &[ScriptLanguage::HmongNyiakengPuachue],
        Ogham => &[ScriptLanguage::OldIrish],
        OlChiki => &[ScriptLanguage::Santali],
        OldHungarian => &[ScriptLanguage::HungarianOld],
        OldItalic => &[
            ScriptLanguage::Etruscan,
            ScriptLanguage::Oscan,
            ScriptLanguage::Umbrian,
        ],
        OldPermic => &[ScriptLanguage::KomiOldPermic],
        OldNorthArabian => &[ScriptLanguage::AncientNorthArabian],
        OldPersian => &[ScriptLanguage::OldPersian],
        OldSogdian => &[ScriptLanguage::SogdianOld],
        OldSouthArabian => &[ScriptLanguage::AncientSouthArabian],
        OldTurkic => &[ScriptLanguage::OldTurkic],
        OldUyghur => &[ScriptLanguage::OldUyghur],
        OlOnal => &[ScriptLanguage::Bhumij],
        Oriya => &[ScriptLanguage::Odia],
        Osage => &[ScriptLanguage::Osage],
        Osmanya => &[ScriptLanguage::SomaliOsmanya],
        PahawhHmong => &[ScriptLanguage::HmongPahawh],
        Palmyrene => &[ScriptLanguage::AramaicPalmyrene],
        PauCinHau => &[ScriptLanguage::Tedim],
        PhagsPa => &[
            ScriptLanguage::MongolianHalhPhagsPa,
            ScriptLanguage::TibetanPhagsPa,
        ],
        // maybe add more?
        Phoenician => &[ScriptLanguage::Phoenician],
        PsalterPahlavi => &[ScriptLanguage::MiddlePersianPahlaviPsalter],
        Rejang => &[ScriptLanguage::RejangRejang],
        Runic => &[ScriptLanguage::OldEnglish, ScriptLanguage::OldNorse],
        Samaritan => &[
            ScriptLanguage::AramaicSamaritan,
            ScriptLanguage::HebrewSamaritan,
        ],
        Saurashtra => &[ScriptLanguage::SaurashtraSaurashtra],
        Sharada => &[
            ScriptLanguage::SanskritSharada,
            ScriptLanguage::KashmiriSharada,
        ],
        Shavian => &[ScriptLanguage::EnglishShavian],
        Siddham => &[ScriptLanguage::SanskritSiddham],
        SignWriting => &[ScriptLanguage::SignLanguages],
        Sinhala => &[ScriptLanguage::Sinhala],
        Sogdian => &[ScriptLanguage::Sogdian],
        SoraSompeng => &[ScriptLanguage::Sora],
        Soyombo => &[
            ScriptLanguage::MongolianHalhSoyombo,
            ScriptLanguage::SanskritSoyombo,
            ScriptLanguage::TibetanSoyombo,
        ],
        Sundanese => &[ScriptLanguage::SundaneseSundanese],
        Sunuwar => &[ScriptLanguage::Sunuwar],
        SylotiNagri => &[ScriptLanguage::Sylheti],
        Syriac => &[ScriptLanguage::AramaicSyriac],
        Tagalog => &[ScriptLanguage::TagalogTagalog],
        Tagbanwa => &[ScriptLanguage::Tagbanwa],
        TaiLe => &[ScriptLanguage::TaiNuea],
        TaiTham => &[
            ScriptLanguage::LaoTaiTham,
            ScriptLanguage::NorthernThai,
            ScriptLanguage::TaiLue,
        ],
        TaiViet => &[ScriptLanguage::TaiDam, ScriptLanguage::TaiDon],
        Takri => &[ScriptLanguage::DogriTakri, ScriptLanguage::KashmiriTakri],
        Tamil => &[ScriptLanguage::Tamil],
        Tangsa => &[ScriptLanguage::Tangsa],
        Tangut => &[ScriptLanguage::Tangut],
        Telugu => &[ScriptLanguage::Telugu],
        Thaana => &[ScriptLanguage::Dhivehi],
        Thai => &[ScriptLanguage::Thai],
        Tibetan => &[ScriptLanguage::Dzongkha, ScriptLanguage::Tibetan],
        Tifinagh => alphabet_match!([
            (
                ScriptLanguage::TamasheqTifinagh,
                [
                    'ⴰ', 'ⴱ', 'ⴳ', 'ⴶ', 'ⴷ', 'ⴸ', 'ⴹ', 'ⴼ', 'ⵀ', 'ⴾ', 'ⵂ', 'ⵃ', 'ⵄ', 'ⵆ', 'ⵈ', 'ⵉ',
                    'ⵊ', 'ⵌ', 'ⵋ', 'ⵍ', 'ⵎ', 'ⵏ', 'ⵑ', 'ⵓ', 'ⵔ', 'ⵗ', 'ⵙ', 'ⵚ', 'ⵛ', 'ⵜ', 'ⵟ', 'ⵡ',
                    'ⵢ', 'ⵣ', 'ⵥ', 'ⵦ', /*unverified*/ 'ⴽ', 'ⴻ', 'ⵖ',
                ]
            ),
            (
                ScriptLanguage::TamazightCentralAtlas, //+
                [
                    // Neo-Tifinagh
                    'ⴰ', 'ⴱ', 'ⴳ', 'ⵯ', 'ⴷ', 'ⴹ', 'ⴻ', 'ⴼ', 'ⴽ', 'ⵀ', 'ⵃ', 'ⵄ', 'ⵅ', 'ⵇ', 'ⵉ', 'ⵊ',
                    'ⵍ', 'ⵎ', 'ⵏ', 'ⵓ', 'ⵔ', 'ⵕ', 'ⵖ', 'ⵙ', 'ⵚ', 'ⵛ', 'ⵜ', 'ⵟ', 'ⵡ', 'ⵢ', 'ⵣ', 'ⵥ',
                    // additional
                    'ⴲ', 'ⴴ', 'ⴵ', 'ⴶ', 'ⴸ', 'ⴺ', 'ⴾ', 'ⴿ', 'ⵁ', 'ⵂ', 'ⵆ', 'ⵈ', 'ⵋ', 'ⵌ', 'ⵐ', 'ⵑ',
                    'ⵒ', 'ⵗ', 'ⵘ', 'ⵝ', 'ⵞ', 'ⵤ', 'ⵦ', 'ⵧ',
                ]
            )
        ]),

        Tirhuta => &[ScriptLanguage::MaithiliTirhuta],
        Todhri => &[ScriptLanguage::AlbanianTodhri],
        Toto => &[ScriptLanguage::Toto],
        TuluTigalari => &[
            ScriptLanguage::KannadaTuluTigalari,
            ScriptLanguage::SanskritTuluTigalari,
            ScriptLanguage::TuluTigalari,
        ],
        Ugaritic => &[ScriptLanguage::Ugaritic],
        Vai => &[ScriptLanguage::Vai],
        Vithkuqi => &[ScriptLanguage::AlbanianToskVithkuqi],
        Wancho => &[ScriptLanguage::WanchoWancho],
        WarangCiti => &[ScriptLanguage::HoWarangCiti],
        Yezidi => &[ScriptLanguage::KurdishNorthernYezidi],
        Yi => &[ScriptLanguage::Loloish],
        ZanabazarSquare => &[
            ScriptLanguage::MongolianHalhZanabazarSquare,
            ScriptLanguage::SanskritZanabazarSquare,
            ScriptLanguage::TibetanZanabazarSquare,
        ],
    }
}
