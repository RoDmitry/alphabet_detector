use crate::{
    lang_arr_default, langs_filter_best, langs_filter_max, word_iter, Language, LanguageArr,
    WordLangsData,
};
use ::core::ops::Range;
use debug_unsafe::slice::SliceGetter;

#[derive(Debug)]
pub struct WordData {
    pub chars: Vec<char>,
    pub range: Range<usize>,
}

impl From<WordLangsData> for WordData {
    #[inline]
    fn from(v: WordLangsData) -> Self {
        Self {
            chars: v.chars,
            range: v.range,
        }
    }
}

pub fn fulltext_langs(
    ch_iter: impl Iterator<Item = (usize, char)>,
) -> (Vec<WordData>, LanguageArr<u32>) {
    let mut words = Vec::new();
    let mut langs_count: LanguageArr<u32> = lang_arr_default();

    let found_words = word_iter::from_ch_iter(ch_iter);
    for wld in found_words {
        // let (langs, count_max) = langs_filter_max(wd.langs_cnt); // worse at detecting
        for (lang, cnt) in wld.langs_cnt.into_iter().enumerate() {
            let v = langs_count.get_safe_unchecked_mut(lang);
            *v += cnt;
        }
        words.push(WordData::from(wld));
    }

    (words, langs_count)
}

/// uses `langs_filter_max`
pub fn fulltext_langs_max(
    ch_iter: impl Iterator<Item = (usize, char)>,
) -> (Vec<WordData>, impl Iterator<Item = Language>, u32) {
    let (words, langs_count) = fulltext_langs(ch_iter);
    let (langs, cnt) = langs_filter_max(langs_count);

    (words, langs, cnt)
}

/// uses `langs_filter_best`.
/// Recommended
pub fn fulltext_langs_best(
    ch_iter: impl Iterator<Item = (usize, char)>,
) -> (Vec<WordData>, Vec<(Language, u32)>) {
    let (words, langs_count) = fulltext_langs(ch_iter);
    let langs = langs_filter_best(langs_count);

    (words, langs)
}
