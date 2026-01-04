# Alphabet Detector

[![Crate](https://img.shields.io/crates/v/alphabet_detector.svg)](https://crates.io/crates/alphabet_detector)
[![API](https://docs.rs/alphabet_detector/badge.svg)](https://docs.rs/alphabet_detector)

## Detects 416 alphabets of 339 languages in 174 scripts

> One language can be written in multiple scripts, so it will be detected as a different [`ScriptLanguage`](https://docs.rs/alphabet_detector/latest/alphabet_detector/enum.ScriptLanguage.html) (language + script)

Does not have any models, just matches the alphabet. Not recommended to use as a standalone detector. 
It's more like a word separator + language prefilter for an actual language detector ([`Langram`](https://github.com/RoDmitry/langram)).

Splits text (iterator `CharIndices`) to words, and detects [`ScriptLanguage`](https://docs.rs/alphabet_detector/latest/alphabet_detector/enum.ScriptLanguage.html)s (language + script) of words by used letters (chars).

### Extras

Look at the [alphabets.rs](https://github.com/RoDmitry/alphabet_detector/blob/main/src/lang/alphabets.rs#L73) to understand what languages have already defined alphabets. Some of them need validation.

Warning: can return words with chars from the Unicode private area (for example `Lingala`, `Nuer` or `Yoruba` languages), because of char normalization (composition with `Inherited`), and there are no such chars defined in Unicode.
