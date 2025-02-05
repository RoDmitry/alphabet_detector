use crate::lang::{char_compose_extra, Script};
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
        char_compose_extra(ch, mark)
    }
}

pub struct CharNormalizingIterator<I: Iterator<Item = (Option<Script>, usize, char)>> {
    iter: I,
    next_char: Option<(Option<Script>, usize, char)>,
    composer: CanonicalCompositionBorrowed<'static>,
}

pub fn from_ch_iter(
    ch_iter: impl Iterator<Item = (usize, char)>,
) -> CharNormalizingIterator<impl Iterator<Item = (Option<Script>, usize, char)>> {
    let mut iter = ch_iter
        .map(|(ch_idx, ch)| (Script::find(ch), ch_idx, ch))
        .chain([(None, usize::MAX - 1, '\0')]); // is it correct?

    let mut next_char = iter.next();
    while let Some((Some(Script::Inherited), _, _)) = next_char {
        next_char = iter.next();
    }
    let composer = CanonicalCompositionBorrowed::new();

    CharNormalizingIterator {
        iter,
        next_char,
        composer,
    }
}

impl<I: Iterator<Item = (Option<Script>, usize, char)>> CharNormalizingIterator<I> {
    #[inline(always)]
    pub fn get_next_char(&self) -> Option<(Option<Script>, usize, char)> {
        self.next_char
    }
}

impl<I: Iterator<Item = (Option<Script>, usize, char)>> Iterator for CharNormalizingIterator<I> {
    type Item = (Option<Script>, usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        let (script, mut ch_idx, mut ch) =
            ::core::mem::replace(&mut self.next_char, self.iter.next())?;

        while let Some((Some(Script::Inherited), i, c)) = self.next_char {
            ch = char_compose(&self.composer, ch, c);
            /* let mut chars = self.normalizer.normalize_iter([ch, c].into_iter());
            ch = chars.next().unwrap();
            if let Some(c) = chars.next() {
                ch = char_compose_extra(ch, c);
            } */
            ch_idx = i;
            self.next_char = self.iter.next();
        }
        if ch == '’' {
            ch = '\'';
        }

        Some((script, ch_idx, ch))
    }
}
