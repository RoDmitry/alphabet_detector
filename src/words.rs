use crate::{
    ch_norm::{self, CharData},
    lang::{script_char_to_slangs, UcdScript, WORD_COMMON_FIRST_CHAR_NOT_SKIPPABLE},
    slang_arr_default, CharNormalizingIterator, ScriptLanguage, ScriptLanguageArr,
};
use ::core::ops::Range;
use debug_unsafe::slice::SliceGetter;
use strum::IntoEnumIterator;

pub trait WordBuf: Default {
    fn push(&mut self, ch: char);
    fn is_empty(&self) -> bool;
}

impl WordBuf for Vec<char> {
    #[inline(always)]
    fn push(&mut self, ch: char) {
        self.push(ch);
    }
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl WordBuf for String {
    #[inline(always)]
    fn push(&mut self, ch: char) {
        self.push(ch);
    }
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "compact_str")))]
#[cfg(feature = "compact_str")]
impl WordBuf for compact_str::CompactString {
    #[inline(always)]
    fn push(&mut self, ch: char) {
        self.push(ch);
    }
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

/// if you don't need word buf
impl WordBuf for bool {
    #[inline(always)]
    fn push(&mut self, _ch: char) {
        *self = true;
    }
    #[inline(always)]
    fn is_empty(&self) -> bool {
        !self
    }
}

pub struct WordIterator<I: Iterator<Item = CharData>, B: WordBuf> {
    norm_iter: CharNormalizingIterator<I>,
    word_buf: B,
    word_start_index: usize,
    not_saved_word_end_index: usize,
    prev_char_script: UcdScript,
    word_langs_cnt: ScriptLanguageArr<u32>,
    word_common_langs_cnt: ScriptLanguageArr<u32>,
    res: Option<WordLang<B>>,
}

impl<I: Iterator<Item = CharData>, B: WordBuf> From<CharNormalizingIterator<I>>
    for WordIterator<I, B>
{
    #[inline]
    fn from(norm_iter: CharNormalizingIterator<I>) -> WordIterator<I, B> {
        Self {
            norm_iter,
            word_buf: Default::default(),
            word_start_index: Default::default(),
            not_saved_word_end_index: Default::default(),
            prev_char_script: UcdScript::Common,
            word_langs_cnt: slang_arr_default(),
            word_common_langs_cnt: slang_arr_default(),
            res: None,
        }
    }
}

/* impl<CI: Iterator<Item = (usize, char)>, I: Iterator<Item = CharData>> From<CI>
    for WordIterator<I>
{ */
// impl<I: Iterator<Item = CharData>> WordIterator<I> {
#[inline]
pub fn from_ch_ind<B: WordBuf>(
    char_indices: impl Iterator<Item = (usize, char)>,
) -> WordIterator<impl Iterator<Item = CharData>, B> {
    let norm_iter = ch_norm::from_ch_ind(char_indices);
    WordIterator::from(norm_iter)
}

#[derive(Clone, Debug)]
pub struct WordLang<B: WordBuf> {
    pub buf: B,
    pub range: Range<usize>,
    pub langs_cnt: ScriptLanguageArr<u32>,
}

impl<I: Iterator<Item = CharData>, B: WordBuf> WordIterator<I, B> {
    fn save_word(&mut self) {
        if !self.word_buf.is_empty() {
            ::core::mem::replace(&mut self.word_common_langs_cnt, slang_arr_default())
                .into_iter()
                .enumerate()
                .for_each(|(lang, cnt)| *self.word_langs_cnt.get_safe_unchecked_mut(lang) += cnt);

            self.res = Some(WordLang {
                buf: ::core::mem::take(&mut self.word_buf),
                range: self.word_start_index..self.not_saved_word_end_index,
                langs_cnt: ::core::mem::replace(&mut self.word_langs_cnt, slang_arr_default()),
            });
            // resets temp variables by taking
        }
    }
}

impl<I: Iterator<Item = CharData>, B: WordBuf> Iterator for WordIterator<I, B> {
    type Item = WordLang<B>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.res.is_none() {
            let Some(CharData { script, idx, ch }) = self.norm_iter.next() else {
                self.save_word();
                break;
            };

            let langs = script_char_to_slangs(script, ch);

            let langs_not_intersect = if self.prev_char_script != script {
                !(ch == '-' || {
                    let langs_cnt = if self.prev_char_script == UcdScript::Common {
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

            let ch_skip = if script == UcdScript::Common {
                if langs_not_intersect
                    || self.prev_char_script == UcdScript::Common
                        && !WORD_COMMON_FIRST_CHAR_NOT_SKIPPABLE.contains(&ch)
                {
                    true
                } else if let Some(CharData {
                    script: next_char_script,
                    ..
                }) = self.norm_iter.get_next_char()
                {
                    next_char_script == UcdScript::Common
                } else {
                    true
                }
            } else {
                false
            };

            if ch_skip {
                self.save_word();
                self.word_start_index = idx + ch.len_utf8();
            } else {
                if langs_not_intersect {
                    self.save_word();
                    self.word_start_index = idx;
                }

                // saving char
                self.not_saved_word_end_index = idx + ch.len_utf8();
                // lowercase
                self.word_buf.push(ch.to_lowercase().next().unwrap());
                let langs_cnt = if script == UcdScript::Common {
                    &mut self.word_common_langs_cnt
                } else {
                    &mut self.word_langs_cnt
                };
                let langs_cnt_incr =
                    |lang: ScriptLanguage| *langs_cnt.get_safe_unchecked_mut(lang as usize) += 1;
                if ch == '-' {
                    ScriptLanguage::iter().for_each(langs_cnt_incr);
                } else {
                    langs.iter().copied().for_each(langs_cnt_incr);
                }
            }
            self.prev_char_script = script;
        }

        self.res.take()
    }
}
