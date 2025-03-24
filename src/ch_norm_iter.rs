use crate::lang::{char_compose_custom, Script};
use icu_normalizer::properties::CanonicalCompositionBorrowed;

#[cfg(all(debug_assertions, feature = "test_chars"))]
pub(crate) fn test_chars(chars: &[char]) {
    let decomp_nfd = icu_normalizer::DecomposingNormalizerBorrowed::new_nfd();
    let composer = CanonicalCompositionBorrowed::new();
    for &raw_ch in chars {
        let raw_ch_str = raw_ch.to_string();
        let decomp = decomp_nfd.normalize(&raw_ch_str);
        let mut decomp_chars = decomp.chars();
        let mut ch = decomp_chars.next().unwrap();
        for c in decomp_chars {
            ch = char_compose(&composer, ch, c);
        }
        assert_eq!(ch, raw_ch, "decomp '{:?}'", decomp.chars());
    }
}

#[inline]
fn char_compose(composer: &CanonicalCompositionBorrowed<'static>, ch: char, mark: char) -> char {
    if let Some(v) = composer.compose(ch, mark) {
        v
    } else {
        char_compose_custom(ch, mark)
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

pub type CharData = (Script, usize, char);

pub struct CharNormalizingIterator<I: Iterator<Item = CharData>> {
    iter: I,
    /// not normalized
    next_char: Option<CharData>,
    after_next_char: Option<CharData>,
    composer: CanonicalCompositionBorrowed<'static>,
}

#[inline]
pub fn from_ch_iter(
    ch_iter: impl Iterator<Item = (usize, char)>,
) -> CharNormalizingIterator<impl Iterator<Item = CharData>> {
    let mut iter = ch_iter.map(|(ch_idx, ch)| (Script::find(ch), ch_idx, ch));

    let mut next_char = iter.next();
    while let Some((Script::Inherited, _, _)) = next_char {
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
    pub fn get_next_char(&self) -> Option<CharData> {
        self.next_char
    }
}

impl<I: Iterator<Item = CharData>> Iterator for CharNormalizingIterator<I> {
    type Item = CharData;

    fn next(&mut self) -> Option<Self::Item> {
        let (script, mut ch_idx, mut ch) = self.next_char?;
        // loading next chars
        if let Some(next) = self.after_next_char.take() {
            self.next_char = Some(next);
        } else {
            let (ch_decom, next_char, after_next_char) = char_decompose(ch);
            if let Some(next_ch) = next_char {
                self.next_char = Some((script, ch_idx, next_ch));
                ch = ch_decom;
                self.after_next_char = after_next_char.map(|ch| (script, ch_idx, ch));
            } else {
                self.next_char = self.iter.next();
            }
        }

        // composing `ch` with `next_char` of `Script::Inherited`
        while let Some((Script::Inherited, i, c)) = self.next_char {
            ch = char_compose(&self.composer, ch, c);
            ch_idx = i;
            self.next_char = self.iter.next();
        }

        if ch == '’' {
            ch = '\'';
        }

        Some((script, ch_idx, ch))
    }
}
