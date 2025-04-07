use alphabet_detector::Language;
use strum::IntoEnumIterator;

#[test]
fn test_order() {
    let mut lang_prev = format!("{:?}", Language::iter().next().unwrap()).to_lowercase();
    for lang in Language::iter() {
        let lang = format!("{lang:?}").to_lowercase();
        assert!(
            lang_prev <= lang,
            "Language wrong order: {lang_prev} > {lang}"
        );
        lang_prev = lang;
    }
}

#[test]
fn test_language_str() {
    for lang in Language::iter() {
        assert!(
            !lang.into_str().chars().any(|ch| !ch.is_ascii_lowercase())
                && lang.into_str().len() < 5,
            "Language {lang:?} wrong string representation: {}",
            lang.into_str(),
        );
    }
}
