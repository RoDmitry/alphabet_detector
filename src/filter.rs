use crate::{ScriptLanguage, ScriptLanguageArr};

#[inline]
pub fn slangs_count_max(langs_cnt: &ScriptLanguageArr<u32>) -> u32 {
    langs_cnt.iter().fold(1, |acc, &cnt| acc.max(cnt))
}

/// Only top `ScriptLanguage`s are retained.
#[inline]
pub fn filter_max(
    langs_cnt: ScriptLanguageArr<u32>,
) -> (impl Iterator<Item = ScriptLanguage>, u32) {
    let lang_count_max = slangs_count_max(&langs_cnt);

    (
        langs_cnt
            .into_iter()
            .enumerate()
            .filter(move |(_, cnt)| *cnt == lang_count_max)
            .map(|(l, _)| ScriptLanguage::transmute_from_usize(l)),
        lang_count_max,
    )
}

/// Only top (100 - `PERCENT`)% `ScriptLanguage`s are retained.
///
/// Less then (100 - `PERCENT`)% margin for an error.
/// `PERCENT` = 95 is recommended
#[inline]
pub fn filter_with_margin<const PERCENT: u32>(
    langs_cnt: ScriptLanguageArr<u32>,
) -> (impl Iterator<Item = (ScriptLanguage, u32)>, u32) {
    assert!(PERCENT < 100);
    let langs_count_margin = slangs_count_max(&langs_cnt) * PERCENT / 100;

    (
        langs_cnt
            .into_iter()
            .enumerate()
            .filter(move |(_, cnt)| *cnt > langs_count_margin)
            .map(|(l, cnt)| (ScriptLanguage::transmute_from_usize(l), cnt)),
        langs_count_margin,
    )
}

/// Only top (100 - `PERCENT`)% `ScriptLanguage`s are retained, then sorted.
///
/// Less then (100 - `PERCENT`)% margin for an error.
/// `PERCENT` = 95 is recommended
#[inline]
pub fn filter_with_margin_sorted<const PERCENT: u32>(
    langs_cnt: ScriptLanguageArr<u32>,
) -> (Vec<(ScriptLanguage, u32)>, u32) {
    let (iter, langs_count_margin) = filter_with_margin::<PERCENT>(langs_cnt);
    let mut res: Vec<_> = iter.collect();
    res.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    (res, langs_count_margin)
}

/*
pub fn langs_count_max3(langs_cnt: &LanguageArr<u32>) -> (u32, u32) {
    let res = langs_cnt.iter().fold((1, 0, 0), |(a, b, c), &cnt| {
        if a < cnt {
            (cnt, a, b)
        } else if a != cnt && b < cnt {
            (a, cnt, b)
        } else if a != cnt && b != cnt && c < cnt {
            (a, b, cnt)
        } else {
            (a, b, c)
        }
    });
    (res.0, res.2)
}
pub fn langs_filter_best(langs_cnt: LanguageArr<u32>) -> Vec<(Language, u32)> {
    let (max1, max3) = langs_count_max3(&langs_cnt);
    let lang_count_filter = (max1 * 95 / 100).max(max3.saturating_sub(1));

    let mut res: Vec<_> = langs_cnt
        .into_iter()
        .enumerate()
        .filter(move |(_, cnt)| *cnt > lang_count_filter)
        .map(|(l, cnt)| (Language::from(l), cnt))
        .collect();
    res.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    res
} */
