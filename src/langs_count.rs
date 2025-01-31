use crate::{Language, LanguageArr};
use ahash::AHashSet;

pub fn langs_count_max(langs_cnt: LanguageArr<u32>) -> (AHashSet<Language>, u32) {
    let lang_count_max = langs_cnt.iter().fold(1, |acc, cnt| acc.max(*cnt));

    (
        langs_cnt
            .into_iter()
            .enumerate()
            .filter(|(_, cnt)| *cnt == lang_count_max)
            .map(|(l, _)| Language::from(l))
            .collect(),
        lang_count_max,
    )
}
