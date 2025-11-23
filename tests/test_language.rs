use alphabet_detector::Language;
use strum::IntoEnumIterator;
use Language::*;

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
fn test_str() {
    for lang in Language::iter() {
        assert!(
            !lang.into_str().chars().any(|ch| !ch.is_ascii_lowercase())
                && lang.into_str().len() < 5,
            "Language {lang:?} wrong string representation: {}",
            lang.into_str(),
        );
    }
}

#[test]
fn test_code() {
    for lang in Language::iter() {
        let code = lang.into_code();
        assert!(code < (1 << 20), "Language {lang:?} too big code: {}", code);
    }
}

#[test]
fn test_to_code() {
    assert_eq!(English.into_code(), 5575_u32);
}

#[test]
fn test_from_code() {
    let language = Language::from_code(5575_u32).unwrap();
    assert_eq!(language, English);
}

#[test]
fn test_to_string() {
    assert_eq!(English.into_str(), "eng");
}

#[test]
fn test_macrolangs() {
    let macrolangs = [
        "aka", "ara", "aym", "aze", "bal", "bik", "bnc", "bua", "chm", /* "cre", */ "del",
        "den", "din", "doi", "est", "fas", "ful", "gba", /* "gon", */ "grb", "grn", "hai",
        "hbs", /* "hmn", */ /* "iku", */ "ipk", "jrb", "kau", "kln", "kok", "kom", "kon",
        "kpe", "kur", "lah", "lav", "luy", /* "man", */ "mlg", "mon", "msa",
        /* "mwr", */ "nep", "nor", /* "oji", */ "ori", "orm", "pus", "que", "raj", "rom",
        "san", /* "sqi", */ /* "srd", */ "swa", "syr", "tmh", "uzb", "yid", "zap", "zha",
        "zho", "zza",
    ];

    for lang in Language::iter() {
        let name = lang.into_str();
        assert!(
            !macrolangs.contains(&name),
            "Language {lang:?} is a macrolang: {}",
            name
        );
    }
}

#[test]
fn test_from_str() {
    let language = Language::from_str("eng").unwrap();
    assert_eq!(language, English);
}

#[test]
fn test_serialize() {
    let serialized = serde_json::to_string(&English).unwrap();
    assert_eq!(serialized, "\"eng\"");
}

#[test]
fn test_deserialize() {
    let deserialized = serde_json::from_str::<Language>("\"eng\"").unwrap();
    assert_eq!(deserialized, English);
}
