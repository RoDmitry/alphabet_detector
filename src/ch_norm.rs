use crate::lang::{char_compose_custom, UcdScript};
use icu_normalizer::properties::CanonicalCompositionBorrowed;

#[cfg(all(debug_assertions, feature = "test_chars"))]
pub(crate) fn test_chars(script: UcdScript, chars: &[char]) {
    let decomp_nfd = icu_normalizer::DecomposingNormalizerBorrowed::new_nfd();
    let composer = CanonicalCompositionBorrowed::new();
    for &ch in chars {
        let ch_script = UcdScript::find(ch);
        assert_eq!(ch_script, script, "char '{ch}'");

        let ch_str = ch.to_string();
        let decomp = decomp_nfd.normalize(&ch_str);
        let mut decomp_chars = decomp.chars();
        let mut recomp_ch = decomp_chars.next().unwrap();
        for c in decomp_chars {
            recomp_ch = char_compose(&composer, recomp_ch, c);
        }
        assert_eq!(
            recomp_ch,
            ch,
            "script: {script:?}, char: '{ch}', decomp '{:?}'",
            decomp.chars()
        );
    }
}

#[inline]
fn char_compose(composer: &CanonicalCompositionBorrowed<'static>, ch: char, mark: char) -> char {
    if let Some(v) = composer.compose(ch, mark) {
        v
    } else {
        let ch_new = char_compose_custom(ch, mark);
        #[cfg(feature = "cli_alphabet_intersection")]
        if ch_new == ch && ch != ' ' && !['\u{361}'].contains(&mark) {
            println!(
                "skipped char_compose mark {:?} char {:?} as {}{} script {:?}",
                mark,
                ch,
                ch,
                mark,
                UcdScript::find(ch)
            );
        }
        ch_new
    }
}

/// decompose ligatures
/// https://en.wikipedia.org/wiki/Alphabetic_Presentation_Forms
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
    pub idx: usize,
    pub ch: char,
}

pub struct CharNormalizingIterator<I: Iterator<Item = CharData>> {
    iter: I,
    /// not normalized
    next_char: Option<CharData>,
    after_next_char: Option<CharData>,
    composer: CanonicalCompositionBorrowed<'static>,
}

#[inline]
pub fn from_ch_ind(
    char_indices: impl Iterator<Item = (usize, char)>,
) -> CharNormalizingIterator<impl Iterator<Item = CharData>> {
    let mut iter = char_indices.map(|(ch_idx, ch)| CharData {
        script: UcdScript::find(ch),
        idx: ch_idx,
        ch,
    });

    let mut next_char = iter.next();
    while let Some(CharData {
        script: UcdScript::Inherited,
        ..
    }) = next_char
    {
        next_char = iter.next();
    }

    CharNormalizingIterator {
        iter,
        next_char,
        after_next_char: None,
        composer: CanonicalCompositionBorrowed::new(),
    }
}

impl<I: Iterator<Item = CharData>> CharNormalizingIterator<I> {
    /// not normalized
    #[inline(always)]
    pub fn peek_next_char(&self) -> Option<CharData> {
        self.next_char
    }
}

impl<I: Iterator<Item = CharData>> Iterator for CharNormalizingIterator<I> {
    type Item = CharData;

    fn next(&mut self) -> Option<Self::Item> {
        let CharData {
            script,
            mut idx,
            mut ch,
        } = self.next_char?;
        // loading next chars
        if let Some(next) = self.after_next_char.take() {
            self.next_char = Some(next);
        } else {
            let (ch_decom, next_char, after_next_char) = char_decompose(ch);
            if let Some(next_ch) = next_char {
                self.next_char = Some(CharData {
                    script,
                    idx,
                    ch: next_ch,
                });
                ch = ch_decom;
                self.after_next_char = after_next_char.map(|ch| CharData { script, idx, ch });
            } else {
                self.next_char = self.iter.next();
            }
        }

        if ['’', 'ʼ'].contains(&ch) {
            ch = '\'';
        } else if ['‐', '‑'].contains(&ch) {
            ch = '-';
        } else {
            // composing `ch` with `next_char` of `UcdScript::Inherited`
            while let Some(CharData {
                script: UcdScript::Inherited,
                idx: i,
                ch: c,
            }) = self.next_char
            {
                ch = char_compose(&self.composer, ch, c);
                idx = i;
                self.next_char = self.iter.next();
            }
        }

        Some(CharData { script, idx, ch })
    }
}
