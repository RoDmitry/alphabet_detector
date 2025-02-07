use crate::{Language, LanguageArr};

pub fn langs_count_max(langs_cnt: &LanguageArr<u32>) -> u32 {
    langs_cnt.iter().fold(1, |acc, &cnt| acc.max(cnt))
}

/// only top langs are retained
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

/// < 5% margin for an error
pub fn langs_filter_best(langs_cnt: LanguageArr<u32>) -> Vec<(Language, u32)> {
    let lang_count_filter = langs_count_max(&langs_cnt) * 95 / 100;

    let mut res: Vec<_> = langs_cnt
        .into_iter()
        .enumerate()
        .filter(move |(_, cnt)| *cnt > lang_count_filter)
        .map(|(l, cnt)| (Language::from(l), cnt))
        .collect();
    res.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    res
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
