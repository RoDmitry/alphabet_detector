use crate::lang::{char_compose_custom, UcdScript};
use array_buf::ArrayDequePlain;
use icu_normalizer::{properties::CanonicalCompositionBorrowed, DecomposingNormalizerBorrowed};
use unicode_normalization::char::canonical_combining_class;

#[cfg(all(debug_assertions, feature = "test_chars"))]
pub(crate) fn test_chars(script: UcdScript, chars: &[char]) {
    let decomp_nfd = DecomposingNormalizerBorrowed::new_nfd();
    let composer = CanonicalCompositionBorrowed::new();
    // let composer_nfkc = icu_normalizer::ComposingNormalizerBorrowed::new_nfkc();
    for &ch in chars {
        let ch_script = UcdScript::find(ch);
        if ch_script == UcdScript::Inherited {
            if UcdScript::scripts_skip_inherited().contains(&script) {
                panic!("Inherited chars are skipped for script {:?}", script);
            }
            continue;
        }
        assert_eq!(ch_script, script, "char '{ch}'");

        /* let recomp: Vec<char> = composer_nfkc.normalize_iter([ch].into_iter()).collect();
        if *recomp.first().unwrap() != ch {
            panic!("Not normal script: {script:?}, char: {ch:?}, normal: {recomp:?}");
        } */

        if ('\u{FB50}'..='\u{FDFF}').contains(&ch) || ('\u{FE70}'..='\u{FEFF}').contains(&ch) {
            panic!("Must not contain Arabic Presentation Forms: char: {ch:?}");
        }

        let mut decomp = decomp_nfd.normalize_iter([ch].into_iter());
        let mut recomp_ch = decomp.next().unwrap();
        for c in decomp {
            recomp_ch = composer.compose(recomp_ch, c).unwrap();
        }
        assert_eq!(recomp_ch, ch, "script: {script:?}, char: '{ch}'");
    }
}

#[inline]
fn char_compose(
    composer: &CanonicalCompositionBorrowed<'static>,
    ch: char,
    mark: char,
) -> Option<char> {
    if let Some(v) = composer.compose(ch, mark) {
        Some(v)
    } else {
        char_compose_custom(ch, mark)
    }
}

/// Decompose ligatures.
/// https://en.wikipedia.org/wiki/Alphabetic_Presentation_Forms
///
/// nfkc/nfkd is not suitable for this crate.
#[inline(always)]
fn char_decompose(ch: char) -> (char, Option<char>, Option<char>) {
    match ch {
        'ﬀ' => ('f', Some('f'), None),
        'ﬁ' => ('f', Some('i'), None),
        'ﬂ' => ('f', Some('l'), None),
        'ﬃ' => ('f', Some('f'), Some('i')),
        'ﬄ' => ('f', Some('f'), Some('l')),
        'ﬅ' | 'ﬆ' => ('s', Some('t'), None),
        ch => (ch, None, None),
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CharData {
    pub script: UcdScript,
    pub ccc: u8,
    pub idx: usize,
    pub ch: char,
}

pub struct CharNormalizingIterator<I: Iterator<Item = CharData>> {
    iter: I,
    /// Chars are not normalized, raw.
    /// Bounded, so it would not be possible to eat all of the memory.
    buf: ArrayDequePlain<CharData, 32>,
    decomposer: DecomposingNormalizerBorrowed<'static>,
    composer: CanonicalCompositionBorrowed<'static>,
}

#[inline]
pub fn from_ch_ind(
    char_indices: impl Iterator<Item = (usize, char)>,
) -> CharNormalizingIterator<impl Iterator<Item = CharData>> {
    let mut iter = char_indices.map(|(ch_idx, ch)| CharData {
        script: UcdScript::find(ch),
        ccc: canonical_combining_class(ch),
        idx: ch_idx,
        ch,
    });

    let mut next_char = iter.next();
    while next_char
        .filter(|c| c.script == UcdScript::Inherited)
        .is_some()
    {
        next_char = iter.next();
    }

    let mut buf = ArrayDequePlain::new();
    if let Some(ch) = next_char {
        unsafe { buf.push_last_unchecked(ch) };
    }

    CharNormalizingIterator {
        iter,
        buf,
        decomposer: DecomposingNormalizerBorrowed::new_nfkd(),
        composer: CanonicalCompositionBorrowed::new(),
    }
}

impl<I: Iterator<Item = CharData>> CharNormalizingIterator<I> {
    /// Not normalized, raw.
    #[inline(always)]
    pub fn peek_next_char(&self) -> Option<CharData> {
        self.buf.first().copied()
    }
}

impl<I: Iterator<Item = CharData>> Iterator for CharNormalizingIterator<I> {
    type Item = CharData;

    fn next(&mut self) -> Option<Self::Item> {
        debug_assert!(!self.buf.is_empty() || self.iter.next().is_none());
        let CharData {
            script,
            ccc,
            mut idx,
            mut ch,
        } = self.buf.pop_first()?;
        debug_assert!(self.buf.len() < self.buf.capacity() - 2);

        let next_char;
        let after_next_char;
        (ch, next_char, after_next_char) = char_decompose(ch);

        if let Some(c) = next_char {
            if let Some(c2) = after_next_char {
                unsafe {
                    self.buf.push_first_unchecked(CharData {
                        script,
                        ccc,
                        idx,
                        ch: c2,
                    })
                };
            }

            unsafe {
                self.buf.push_first_unchecked(CharData {
                    script,
                    ccc,
                    idx,
                    ch: c,
                })
            };
        } else {
            if ['’', 'ʼ'].contains(&ch) {
                ch = '\'';
            } else if ['‐', '‑'].contains(&ch) {
                ch = '-';
            } else if ch == '\u{6e1}' {
                ch = '\u{652}'; // Arabic Sukūn
            } else if ('\u{FB50}'..='\u{FDFF}').contains(&ch)
                || ('\u{FE70}'..='\u{FEFF}').contains(&ch)
            {
                // decomposes Arabic Presentation Forms A & B
                if self.buf.len() < 2 {
                    let mut decomp = self.decomposer.normalize_iter([ch].into_iter());
                    if let Some(c) = decomp.next() {
                        if UcdScript::find(c) == script {
                            ch = c;

                            if let Some(c2) = decomp.next() {
                                let last_loaded_char = self.buf.first().copied();
                                self.buf.clear();

                                for ci in [c2].into_iter().chain(decomp) {
                                    let cd = CharData {
                                        script: UcdScript::find(ci),
                                        ccc: canonical_combining_class(ci),
                                        idx,
                                        ch: ci,
                                    };
                                    if let Err(_e) = self.buf.push_last(cd) {
                                        #[cfg(debug_assertions)]
                                        panic!("Ch norm {}", _e);
                                    }
                                }

                                if let Some(cd) = last_loaded_char {
                                    if let Err(_e) = self.buf.push_last(cd) {
                                        #[cfg(debug_assertions)]
                                        panic!("Ch norm {}", _e);
                                    }
                                }
                                unsafe { ::core::hint::assert_unchecked(!self.buf.is_empty()) };
                            }
                        }
                    } else {
                        #[cfg(debug_assertions)]
                        unreachable!();
                    }
                } else {
                    #[cfg(debug_assertions)]
                    unreachable!();
                }
            }

            if self.buf.is_empty() {
                // loading next char
                for c in self.iter.by_ref() {
                    // skips "\u{fe0f}" and "\u{fe0e}"
                    if c.script != UcdScript::Inherited || c.ccc != 0 {
                        unsafe { self.buf.push_last_unchecked(c) };
                        break;
                    }
                }
            }

            // reorder chars, compose `UcdScript::Inherited`
            if self.buf.first().filter(|c| c.ccc > 0).is_some() {
                let mut last_loaded_char = None;
                if self.buf.len() == 1 {
                    self.buf.linearize_one();
                    // load all chars with `ccc` > 0
                    for c in self.iter.by_ref() {
                        if c.ccc == 0 {
                            // skips "\u{fe0f}" and "\u{fe0e}"
                            if c.script == UcdScript::Inherited {
                                continue;
                            }
                            last_loaded_char = Some(c);
                            break;
                        }

                        // leaves space for `last_loaded_char` - next `pop_first` + 2 from `char_decompose`
                        if self.buf.len() < self.buf.capacity() - 2 {
                            unsafe { self.buf.push_last_unchecked(c) };
                        }
                    }
                    debug_assert!(last_loaded_char.is_some() || self.iter.next().is_none());

                    // reorder chars by ccc
                    unsafe { self.buf.as_mut_slice() }.sort_by(|a, b| a.ccc.cmp(&b.ccc));
                }

                // composing `ch` with next char of `UcdScript::Inherited`
                while let Some(CharData {
                    script: UcdScript::Inherited,
                    ccc: cc,
                    idx: i,
                    ch: c,
                }) = self.buf.first().copied()
                {
                    debug_assert!(cc > 0);

                    if let Some(ch_new) = char_compose(&self.composer, ch, c) {
                        unsafe { self.buf.pop_first_unchecked() };
                        ch = ch_new;
                        idx = idx.max(i); // looks like it needs range?
                    } else {
                        break;
                    }
                }

                if let Some(c) = last_loaded_char {
                    unsafe { self.buf.push_last_unchecked(c) };
                }
            }
        }

        Some(CharData {
            script,
            ccc,
            idx,
            ch,
        })
    }
}
