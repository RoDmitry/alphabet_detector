use alphabet_detector::{Language, ScriptLanguage, UcdScript};
use strum::EnumCount;

#[test]
fn count_alphabets() {
    assert_eq!(
        ScriptLanguage::COUNT,
        416 + 2,
        "Change alphabets count in docs"
    );
}

#[test]
fn count_languages() {
    assert_eq!(Language::COUNT, 340 + 1, "Change languages count in docs");
}

#[test]
fn count_scripts() {
    assert_eq!(UcdScript::COUNT, 174, "Change scripts count in docs");
}
