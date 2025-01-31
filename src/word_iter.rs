use crate::{
    lang::{char_combine, script_char_to_langs, Script},
    lang_arr_default, LanguageArr,
};
use ::core::ops::Range;
use debug_unsafe::slice::SliceGetter;

pub struct WordIterator<I: Iterator<Item = (Option<Script>, usize, char)>> {
    iter: I,
    next_char: Option<(Option<Script>, usize, char)>,
    word_buf: Vec<char>,
    word_start_index: usize,
    not_saved_word_end_index: usize,
    prev_char_script: Script,
    word_langs_cnt: LanguageArr<u32>,
    word_common_langs_cnt: LanguageArr<u32>,
    res: Option<WordData>,
}

/* impl<CT: Iterator<Item = (usize, char)>, I: Iterator<Item = (Option<Script>, usize, char)>> From<T>
    for WordIterator<I>
{ */
pub fn from_ch_iter(
    ch_iter: impl Iterator<Item = (usize, char)>,
) -> WordIterator<impl Iterator<Item = (Option<Script>, usize, char)>> {
    let mut iter = ch_iter
        .map(|(ch_idx, ch)| (Script::find(ch), ch_idx, ch))
        .chain([(None, usize::MAX - 1, '\0')]); // is it correct?

    let next_char = iter.next();

    WordIterator {
        iter,
        next_char,
        word_buf: Default::default(),
        word_start_index: Default::default(),
        not_saved_word_end_index: Default::default(),
        prev_char_script: Script::Common,
        word_langs_cnt: lang_arr_default(),
        word_common_langs_cnt: lang_arr_default(),
        res: None,
    }
}

#[derive(Debug)]
pub struct WordData {
    pub chars: Vec<char>,
    pub langs_cnt: LanguageArr<u32>,
    pub range: Range<usize>,
}

impl<I: Iterator<Item = (Option<Script>, usize, char)>> Iterator for WordIterator<I> {
    type Item = WordData;

    fn next(&mut self) -> Option<Self::Item> {
        while self.res.is_none() {
            let Some((script, mut ch_idx, mut ch)) =
                ::core::mem::replace(&mut self.next_char, self.iter.next())
            else {
                break;
            };

            if script == Some(Script::Inherited) {
                continue;
            }
            if let Some((Some(Script::Inherited), i, c)) = self.next_char {
                ch = char_combine(script, ch, c);
                ch_idx = i;
                self.next_char = self.iter.next();
            }
            if ch == '’' {
                ch = '\'';
            }

            let langs = script
                .map(|s| script_char_to_langs(s, ch))
                .unwrap_or_default();

            let script = script.unwrap_or(Script::Common); // why Common, maybe skip?

            let langs_not_intersect = if self.prev_char_script != script {
                !langs
                    .iter()
                    .any(|&l| *self.word_langs_cnt.get_safe_unchecked(l as usize) > 0)
            } else {
                false
            };

            let ch_skip = if langs.is_empty() {
                true
            } else if script == Script::Common {
                if self.prev_char_script == Script::Common || langs_not_intersect {
                    true
                } else if let Some((next_char_script, _, _)) = self.next_char {
                    next_char_script.is_none() || next_char_script == Some(Script::Common)
                } else {
                    true
                }
            } else {
                false
            };

            if ch_skip || langs_not_intersect {
                if !self.word_buf.is_empty() {
                    // saves word
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
                        langs_cnt: ::core::mem::replace(
                            &mut self.word_langs_cnt,
                            lang_arr_default(),
                        ),
                        range: self.word_start_index..self.not_saved_word_end_index,
                    });
                    // resets temp variables by taking
                }
                self.word_start_index = ch_idx + ch.len_utf8();
            }

            if !ch_skip {
                self.not_saved_word_end_index = ch_idx + ch.len_utf8();
                self.word_buf.push(ch.to_lowercase().next().unwrap()); // maybe check each char?
                let langs_cnt = if script == Script::Common {
                    &mut self.word_common_langs_cnt
                } else {
                    &mut self.word_langs_cnt
                };
                for &lang in langs {
                    let v = langs_cnt.get_safe_unchecked_mut(lang as usize);
                    *v = v.wrapping_add(1);
                }
            }
            self.prev_char_script = script;
        }

        self.res.take()
    }
}
