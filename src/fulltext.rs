use crate::{
    filter_max, filter_with_margin, filter_with_margin_sorted, slang_arr_default,
    words::{self, WordBuf},
    ScriptLanguage, ScriptLanguageArr, WordLang,
};
use ::core::ops::Range;
use debug_unsafe::slice::SliceGetter;

#[derive(Clone, Debug)]
pub struct Word<B: WordBuf> {
    pub buf: B,
    pub range: Range<usize>,
}

impl<B: WordBuf> From<WordLang<B>> for Word<B> {
    #[inline(always)]
    fn from(v: WordLang<B>) -> Self {
        Self {
            buf: v.buf,
            range: v.range,
        }
    }
}

/// all words detection summed up
pub fn fulltext<B: WordBuf>(
    char_indices: impl Iterator<Item = (usize, char)>,
) -> (Vec<Word<B>>, ScriptLanguageArr<u32>) {
    let mut words = Vec::new();
    let mut langs_count: ScriptLanguageArr<u32> = slang_arr_default();

    let found_words = words::from_ch_ind(char_indices);
    for wld in found_words {
        // let (langs, count_max) = filter_max(wld.langs_cnt); // worse at detecting
        for (lang, cnt) in wld.langs_cnt.into_iter().enumerate() {
            *langs_count.get_safe_unchecked_mut(lang) += cnt;
        }
        words.push(Word::from(wld));
    }

    (words, langs_count)
}

/// all words detection summed up, then filtered by max ([`filter_max`](fn.filter_max.html)
pub fn fulltext_filter_max<B: WordBuf>(
    char_indices: impl Iterator<Item = (usize, char)>,
) -> (Vec<Word<B>>, impl Iterator<Item = ScriptLanguage>, u32) {
    let (words, langs_count) = fulltext(char_indices);
    let (langs, cnt) = filter_max(langs_count);

    (words, langs, cnt)
}

/// all words detection summed up, then filtered with margin percent
/// ([`filter_with_margin`](fn.filter_with_margin.html)).
/// less then (100 - `PERCENT`)% margin for an error.
/// `PERCENT` = 95 is recommended.
/// Recommended
pub fn fulltext_filter_with_margin<B: WordBuf, const PERCENT: u32>(
    char_indices: impl Iterator<Item = (usize, char)>,
) -> (Vec<Word<B>>, impl Iterator<Item = (ScriptLanguage, u32)>) {
    let (words, langs_count) = fulltext(char_indices);
    let langs = filter_with_margin::<PERCENT>(langs_count);

    (words, langs)
}

/// all words detection summed up, then filtered with margin percent
/// ([`filter_with_margin_sorted`](fn.filter_with_margin_sorted.html)), then sorted.
/// less then (100 - `PERCENT`)% margin for an error.
/// `PERCENT` = 95 is recommended.
pub fn fulltext_filter_with_margin_sorted<B: WordBuf, const PERCENT: u32>(
    char_indices: impl Iterator<Item = (usize, char)>,
) -> (Vec<Word<B>>, Vec<(ScriptLanguage, u32)>) {
    let (words, langs_count) = fulltext(char_indices);
    let langs = filter_with_margin_sorted::<PERCENT>(langs_count);

    (words, langs)
}
