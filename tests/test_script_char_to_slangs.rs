use alphabet_detector::{
    script_char_to_slangs, slang_arr_default, ucd::BY_NAME, Language, ScriptLanguage, UcdScript,
};
// use icu_normalizer::properties::CanonicalCompositionBorrowed;
use strum::EnumCount;

#[test]
fn test_empty() {
    assert!(
        script_char_to_slangs(UcdScript::Inherited, char::default()).is_empty(),
        "UcdScript::Inherited must be always empty"
    );
    assert!(
        script_char_to_slangs(UcdScript::Common, char::default()).is_empty(),
        "UcdScript::Common match other char must be always empty"
    );
}

#[test]
fn test_each_char() {
    let mut langs;
    // let decomp_nfd = icu_normalizer::DecomposingNormalizerBorrowed::new_nfd();
    // let composer = CanonicalCompositionBorrowed::new();
    for &(script, ranges) in BY_NAME {
        if script == UcdScript::Inherited {
            continue;
        }
        for ch in ranges.iter().flat_map(|range| range.0..=range.1) {
            let ch_langs = script_char_to_slangs(script, ch);
            if ch_langs.len() == 1
                && Language::from(*ch_langs.first().unwrap()) == Language::Unknown
            {
                continue;
            }
            if ch_langs.is_empty() && script != UcdScript::Common {
                panic!("Empty langs in {:?} for char: {}", script, ch);
            }

            /* if script != UcdScript::Arabic {
                let mut decomp = decomp_nfd.normalize_iter([ch].into_iter());
                let decomp_ch = decomp.next().unwrap();
                if decomp_ch != ch {
                    let decomp_ch_langs = script_char_to_slangs(script, decomp_ch);
                    if decomp_ch_langs != ch_langs {
                        let diff = ch_langs
                            .iter()
                            .filter(|l| !decomp_ch_langs.contains(l))
                            .collect::<Vec<_>>();
                        if !diff.is_empty() {
                            println!("{:?}", ch_langs);
                            println!("{:?}", decomp_ch_langs);
                            panic!("{:?} ({:?}) must also be in {:?}", decomp_ch, ch, diff);
                        }
                    }
                }
            } */

            // test_doubles
            {
                langs = slang_arr_default::<usize>();
                for &lang in ch_langs {
                    langs[lang as usize] += 1;
                }

                let langs_used: ahash::AHashSet<ScriptLanguage> = langs
                    .into_iter()
                    .enumerate()
                    .filter(|(_, cnt)| *cnt > 1)
                    .map(|(l, _)| unsafe { ScriptLanguage::transmute_from_usize(l) })
                    .collect();

                if !langs_used.is_empty() {
                    panic!(
                        "{:?} are used twice in {:?} for char: {}",
                        langs_used, script, ch
                    );
                }
            }

            /* if script == UcdScript::Arabic {
                for ch2 in ranges.iter().flat_map(|range| range.0..=range.1) {
                    if let Some(ch_new) = composer.compose(ch, ch2) {
                        let ch_langs_new = script_char_to_slangs(script, ch_new);
                        if !ch_langs_new.is_empty() {
                            let ch_langs2 = script_char_to_slangs(script, ch2);
                            let mut langs = ch_langs
                                .iter()
                                .filter(|l| !ch_langs_new.contains(l) && ch_langs2.contains(l));

                            if let Some(first_lang) = langs.next() {
                                panic!(
                                    "{:?} must be in {:?},{:?}",
                                    ch_new,
                                    first_lang,
                                    langs.collect::<Vec<_>>()
                                );
                            }
                        }
                    }
                }
            } */
        }
    }
}

#[test]
fn test_alphabets() {
    use strum::IntoEnumIterator;

    let mut slangs: [Vec<UcdScript>; ScriptLanguage::COUNT] =
        ::core::array::from_fn(|_| Vec::new());
    for script in UcdScript::iter() {
        if script == UcdScript::Common || script == UcdScript::Inherited {
            continue;
        }
        for &slang in ScriptLanguage::all_with_script(script) {
            slangs[slang as usize].push(script);
        }
    }

    let slangs_used: ahash::AHashSet<ScriptLanguage> = slangs
        .iter()
        .enumerate()
        .filter(|(_, scrs)| scrs.is_empty())
        .map(|(l, _)| unsafe { ScriptLanguage::transmute_from_usize(l) })
        .filter(|&sl| ![Language::Unknown, Language::Math].contains(&Language::from(sl)))
        .collect();

    if !slangs_used.is_empty() {
        panic!("{:?} do not have alphabets", slangs_used);
    }

    let slangs_used: ahash::AHashSet<ScriptLanguage> = slangs
        .iter()
        .enumerate()
        .filter(|(_, scrs)| scrs.len() > 1)
        .map(|(sl, _)| unsafe { ScriptLanguage::transmute_from_usize(sl) })
        .filter(|sl| ![ScriptLanguage::Japanese, ScriptLanguage::Korean].contains(sl))
        .collect();

    if !slangs_used.is_empty() {
        panic!("{:?} are used in multiple scripts", slangs_used);
    }
}
