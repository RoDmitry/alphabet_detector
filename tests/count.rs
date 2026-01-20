use alphabet_detector::{Language, ScriptLanguage, UcdScript};
use strum::EnumCount;

#[test]
fn count_alphabets() {
    assert_eq!(
        429 + 2,
        ScriptLanguage::COUNT,
        "Change alphabets count in docs"
    );
}

#[test]
fn count_languages() {
    assert_eq!(347 + 1, Language::COUNT, "Change languages count in docs");
}

#[test]
fn count_scripts() {
    assert_eq!(174, UcdScript::COUNT, "Change scripts count in docs");
}
