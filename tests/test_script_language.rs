use ahash::AHashSet;
use alphabet_detector::{script_char_to_slangs, Language, Script, ScriptLanguage, UcdScript};
use strum::{EnumCount, IntoEnumIterator};
use ScriptLanguage::*;

#[test]
fn test_order() {
    let mut script_prev = Script::Common;
    let mut scripts_closed = AHashSet::default();
    for slang in ScriptLanguage::iter() {
        let script = Script::from(slang);
        if script != script_prev {
            if !scripts_closed.insert(script) {
                panic!("ScriptLanguage wrong order: {script:?}");
            }
            script_prev = script;
        }
    }
}

#[test]
fn test_max_value() {
    for slang in ScriptLanguage::iter() {
        assert!(
            (slang as usize) < ScriptLanguage::COUNT,
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
fn test_each_code() {
    for v in ScriptLanguage::iter() {
        let code = v.into_code();
        assert!(
            code < (1 << 30),
            "ScriptLanguage {v:?} too big code: {}",
            code
        );
    }
}

#[test]
fn test_to_code() {
    assert_eq!(English.into_code(), 5709015_u32);
}

#[test]
fn test_from_code() {
    let language = ScriptLanguage::from_code(5709015_u32).unwrap();
    assert_eq!(language, English);
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
