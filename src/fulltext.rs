use crate::{lang_arr_default, langs_count_max, word_iter, Language, LanguageArr};
use ahash::AHashSet;
use debug_unsafe::slice::SliceGetter;

pub fn fulltext_langs(
    ch_iter: impl Iterator<Item = (usize, char)>,
) -> (Vec<Vec<char>>, AHashSet<Language>, u32) {
    let mut words = Vec::new();
    let mut languages: LanguageArr<u32> = lang_arr_default();

    let found_words = word_iter::from_ch_iter(ch_iter);
    for wd in found_words {
        words.push(wd.chars);
        for (lang, cnt) in wd.langs_cnt.into_iter().enumerate() {
            let v = languages.get_safe_unchecked_mut(lang);
            *v = v.wrapping_add(cnt);
        }
    }

    let (langs, cnt) = langs_count_max(languages);

    (words, langs, cnt)
}
