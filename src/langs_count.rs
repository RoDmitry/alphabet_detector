use crate::{Language, LanguageArr};

pub fn langs_count_max(langs_cnt: &LanguageArr<u32>) -> u32 {
    langs_cnt.iter().fold(1, |acc, cnt| acc.max(*cnt))
}

/// filters 100%
pub fn langs_filter_max(langs_cnt: LanguageArr<u32>) -> (impl Iterator<Item = Language>, u32) {
    let lang_count_max = langs_count_max(&langs_cnt);

    (
        langs_cnt
            .into_iter()
            .enumerate()
            .filter(move |(_, cnt)| *cnt == lang_count_max)
            .map(|(l, _)| Language::from(l)),
        lang_count_max,
    )
}

/// filters 99%
pub fn langs_filter_best(langs_cnt: LanguageArr<u32>) -> impl Iterator<Item = Language> {
    let lang_count_filter = langs_count_max(&langs_cnt) * 99 / 100;

    langs_cnt
        .into_iter()
        .enumerate()
        .filter(move |(_, cnt)| *cnt > lang_count_filter)
        .map(|(l, _)| Language::from(l))
}
