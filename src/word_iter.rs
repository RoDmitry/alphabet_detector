use crate::{
    ch_norm_iter::{self, CharData},
    lang::{script_char_to_langs, Script, WORD_COMMON_FIRST_CHAR_NOT_SKIPPABLE},
    lang_arr_default, CharNormalizingIterator, Language, LanguageArr,
};
use ::core::ops::Range;
use debug_unsafe::slice::SliceGetter;
use strum::IntoEnumIterator;

pub struct WordIterator<I: Iterator<Item = CharData>> {
    norm_iter: CharNormalizingIterator<I>,
    word_buf: Vec<char>,
    word_start_index: usize,
    not_saved_word_end_index: usize,
    prev_char_script: Script,
    word_langs_cnt: LanguageArr<u32>,
    word_common_langs_cnt: LanguageArr<u32>,
    res: Option<WordData>,
}

impl<I: Iterator<Item = CharData>> From<CharNormalizingIterator<I>> for WordIterator<I> {
    #[inline]
    fn from(norm_iter: CharNormalizingIterator<I>) -> WordIterator<I> {
        Self {
            norm_iter,
            word_buf: Default::default(),
            word_start_index: Default::default(),
            not_saved_word_end_index: Default::default(),
            prev_char_script: Script::Common,
            word_langs_cnt: lang_arr_default(),
            word_common_langs_cnt: lang_arr_default(),
            res: None,
        }
    }
}

/* impl<CI: Iterator<Item = (usize, char)>, I: Iterator<Item = CharData>> From<CI>
    for WordIterator<I>
{ */
// impl<I: Iterator<Item = CharData>> WordIterator<I> {
#[inline]
pub fn from_ch_iter(
    ch_iter: impl Iterator<Item = (usize, char)>,
) -> WordIterator<impl Iterator<Item = CharData>> {
    let norm_iter = ch_norm_iter::from_ch_iter(ch_iter);
    WordIterator::from(norm_iter)
}

#[derive(Debug)]
pub struct WordData {
    pub chars: Vec<char>,
    pub langs_cnt: LanguageArr<u32>,
    pub range: Range<usize>,
}

impl<I: Iterator<Item = CharData>> WordIterator<I> {
    fn save_word(&mut self) {
        if !self.word_buf.is_empty() {
            for (lang, cnt) in
                ::core::mem::replace(&mut self.word_common_langs_cnt, lang_arr_default())
                    .into_iter()
                    .enumerate()
            {
                let v = self.word_langs_cnt.get_safe_unchecked_mut(lang);
                *v = v.wrapping_add(cnt);
            }

            self.res = Some(WordData {
                chars: ::core::mem::take(&mut self.word_buf),
                langs_cnt: ::core::mem::replace(&mut self.word_langs_cnt, lang_arr_default()),
                range: self.word_start_index..self.not_saved_word_end_index,
            });
            // resets temp variables by taking
        }
    }
}

impl<I: Iterator<Item = CharData>> Iterator for WordIterator<I> {
    type Item = WordData;

    fn next(&mut self) -> Option<Self::Item> {
        while self.res.is_none() {
            let Some((script, ch_idx, ch)) = self.norm_iter.next() else {
                self.save_word();
                break;
            };

            let langs = script
                .map(|s| script_char_to_langs(s, ch))
                .unwrap_or_default();

            // optimization, allows to comment out unused Common ranges in `ucd`
            let script = script.unwrap_or(Script::Common);

            let langs_not_intersect = if self.prev_char_script != script {
                !(ch == '-' || {
                    let langs_cnt = if self.prev_char_script == Script::Common {
                        &self.word_common_langs_cnt
                    } else {
                        &self.word_langs_cnt
                    };
                    langs
                        .iter()
                        .any(|&l| *langs_cnt.get_safe_unchecked(l as usize) > 0)
                })
            } else {
                false
            };

            let ch_skip = if script == Script::Common {
                if langs_not_intersect
                    || self.prev_char_script == Script::Common
                        && !WORD_COMMON_FIRST_CHAR_NOT_SKIPPABLE.contains(&ch)
                {
                    true
                } else if let Some((next_char_script, _, _)) = self.norm_iter.get_next_char() {
                    next_char_script.is_none() || next_char_script == Some(Script::Common)
                } else {
                    true
                }
            } else {
                false
            };

            if ch_skip {
                self.save_word();
                self.word_start_index = ch_idx.wrapping_add(ch.len_utf8());
            } else {
                if langs_not_intersect {
                    self.save_word();
                    self.word_start_index = ch_idx;
                }

                // saving char
                self.not_saved_word_end_index = ch_idx.wrapping_add(ch.len_utf8());
                self.word_buf.push(ch.to_lowercase().next().unwrap()); // maybe check each char?
                let langs_cnt = if script == Script::Common {
                    &mut self.word_common_langs_cnt
                } else {
                    &mut self.word_langs_cnt
                };
                let langs_cnt_incr = |lang: Language| {
                    let v = langs_cnt.get_safe_unchecked_mut(lang as usize);
                    *v = v.wrapping_add(1);
                };
                if ch == '-' {
                    Language::iter().for_each(langs_cnt_incr);
                } else {
                    langs.iter().copied().for_each(langs_cnt_incr);
                }
            }
            self.prev_char_script = script;
        }

        self.res.take()
    }
}
