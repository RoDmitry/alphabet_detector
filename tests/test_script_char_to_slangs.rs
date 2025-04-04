use alphabet_detector::{
    script_char_to_slangs, slang_arr_default, ucd::BY_NAME, Script, ScriptLanguage,
};
use strum::EnumCount;

#[test]
fn test_empty() {
    assert!(
        script_char_to_slangs(Script::Inherited, char::default()).is_empty(),
        "Script::Inherited must be always empty"
    );
    assert!(
        script_char_to_slangs(Script::Common, char::default()).is_empty(),
        "Script::Common match other char must be always empty"
    );
}

#[test]
fn test_doubles() {
    let mut langs;
    for &(script, ranges) in BY_NAME {
        if script == Script::Inherited {
            continue;
        }
        for range in ranges {
            for ch in range.0..=range.1 {
                langs = slang_arr_default::<usize>();

                let ch_langs = script_char_to_slangs(script, ch);
                if ch_langs.is_empty() && script != Script::Common {
                    panic!("Empty langs in {:?} for char: {}", script, ch);
                }
                for &lang in ch_langs {
                    langs[lang as usize] += 1;
                }

                let langs_used: ahash::AHashSet<ScriptLanguage> = langs
                    .into_iter()
                    .enumerate()
                    .filter(|(_, cnt)| *cnt > 1)
                    .map(|(l, _)| ScriptLanguage::from(l))
                    .collect();

                if !langs_used.is_empty() {
                    panic!(
                        "{:?} are used twice in {:?} for char: {}",
                        langs_used, script, ch
                    );
                }
            }
        }
    }
}

#[test]
fn test_alphabets() {
    use strum::IntoEnumIterator;

    let mut slangs: [Vec<Script>; ScriptLanguage::COUNT] = ::core::array::from_fn(|_| Vec::new());
    for script in Script::iter() {
        if script == Script::Common || script == Script::Inherited {
            continue;
        }
        for &slang in script_char_to_slangs(script, char::default()) {
            slangs[slang as usize].push(script);
        }
    }

    let slangs_used: ahash::AHashSet<ScriptLanguage> = slangs
        .iter()
        .enumerate()
        .filter(|(_, scrs)| scrs.is_empty())
        .map(|(l, _)| ScriptLanguage::from(l))
        .collect();

    if !slangs_used.is_empty() {
        panic!("{:?} do not have alphabets", slangs_used);
    }

    let slangs_used: ahash::AHashSet<ScriptLanguage> = slangs
        .iter()
        .enumerate()
        .filter(|(_, scrs)| scrs.len() > 1)
        .map(|(sl, _)| ScriptLanguage::from(sl))
        .filter(|&sl| <Option<Script>>::from(sl).is_some())
        .collect();

    if !slangs_used.is_empty() {
        panic!("{:?} are used in multiple scripts", slangs_used);
    }

    let slangs_used: ahash::AHashSet<ScriptLanguage> = slangs
        .iter()
        .enumerate()
        .filter(|(_, scrs)| scrs.len() == 1)
        .map(|(sl, _)| ScriptLanguage::from(sl))
        .filter(|&sl| <Option<Script>>::from(sl).is_none() && sl.into_script_str().is_empty())
        .collect();

    if !slangs_used.is_empty() {
        panic!(
            "{:?} do not have a script stated in `slang` macro",
            slangs_used
        );
    }
}
