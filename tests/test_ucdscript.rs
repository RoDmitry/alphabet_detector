use alphabet_detector::UcdScript;
use strum::IntoEnumIterator;
use UcdScript::*;

#[test]
fn test_order() {
    let mut scr_prev = format!("{:?}", UcdScript::iter().next().unwrap()).to_lowercase();
    for scr in UcdScript::iter() {
        let scr = format!("{scr:?}").to_lowercase();
        assert!(scr_prev <= scr, "UcdScript wrong order: {scr_prev} > {scr}");
        scr_prev = scr;
    }
}

#[test]
fn test_each_str() {
    for scr in UcdScript::iter() {
        let mut str_chars = scr.into_str().chars();
        assert!(
            str_chars.next().unwrap().is_ascii_uppercase()
                && !str_chars.any(|ch| !ch.is_ascii_lowercase())
                && scr.into_str().len() == 4,
            "UcdScript {scr:?} wrong string representation: {}",
            scr.into_str(),
        );
    }
}

#[test]
fn test_each_code() {
    for v in UcdScript::iter() {
        let code = v.into_code();
        assert!(code < (1 << 10), "UcdScript {v:?} too big code: {}", code);
    }
}

#[test]
fn test_to_code() {
    assert_eq!(Latin.into_code(), 215_u16);
}

#[test]
fn test_from_code() {
    let language = UcdScript::from_code(215_u16).unwrap();
    assert_eq!(language, Latin);
}

#[test]
fn test_to_string() {
    assert_eq!(Latin.into_str(), "Latn");
}

#[test]
fn test_from_str() {
    let language = UcdScript::from_str("Latn").unwrap();
    assert_eq!(language, Latin);
}

#[test]
fn test_serialize() {
    let serialized = serde_json::to_string(&Latin).unwrap();
    assert_eq!(serialized, "\"Latn\"");
}

#[test]
fn test_deserialize() {
    let deserialized = serde_json::from_str::<UcdScript>("\"Latn\"").unwrap();
    assert_eq!(deserialized, Latin);
}
