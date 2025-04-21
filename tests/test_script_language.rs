use alphabet_detector::{script_char_to_slangs, Language, Script, ScriptLanguage, UcdScript};
use strum::{EnumCount, IntoEnumIterator};
use ScriptLanguage::*;

#[test]
fn test_order() {
    let mut lang_prev = format!("{:?}", ScriptLanguage::iter().next().unwrap()).to_lowercase();
    for lang in ScriptLanguage::iter() {
        let lang = format!("{lang:?}").to_lowercase();
        assert!(
            lang_prev <= lang,
            "ScriptLanguage wrong order: {lang_prev} > {lang}"
        );
        lang_prev = lang;
    }
}

#[test]
fn test_max_value() {
    for lang in ScriptLanguage::iter() {
        assert!(
            (lang as usize) < ScriptLanguage::COUNT,
            "Language value >= it's count"
        );
    }
}

#[test]
fn test_correct_map_to_lang() {
    for slang in ScriptLanguage::iter() {
        let lang = Language::from(slang);
        let lang = format!("{lang:?}");
        let slang = format!("{slang:?}");
        assert!(
            slang.starts_with(&lang),
            "ScriptLanguage {slang} wrong lang: {lang}"
        );
    }
}

#[test]
fn test_correct_map_to_script() {
    for slang in ScriptLanguage::iter() {
        let script = Script::from(slang);
        assert!(
            script_char_to_slangs(UcdScript::from(script), char::default()).contains(&slang),
            "ScriptLanguage {slang:?} wrong script: {script:?}",
        );
    }
}

#[test]
fn test_to_string() {
    assert_eq!(English.into_str(), "engLatn");
}

#[test]
fn test_from_str() {
    let language = ScriptLanguage::from_str("engLatn").unwrap();
    assert_eq!(language, English);
}

#[test]
fn test_serialize() {
    let serialized = serde_json::to_string(&English).unwrap();
    assert_eq!(serialized, "\"engLatn\"");
}

#[test]
fn test_deserialize() {
    let deserialized = serde_json::from_str::<ScriptLanguage>("\"engLatn\"").unwrap();
    assert_eq!(deserialized, English);
}
