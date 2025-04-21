use alphabet_detector::UcdScript;
use strum::IntoEnumIterator;

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
fn test_str() {
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
