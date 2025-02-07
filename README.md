# Alphabet Detector

[![Crate](https://img.shields.io/crates/v/alphabet_detector.svg)](https://crates.io/crates/alphabet_detector)
[![API](https://docs.rs/alphabet_detector/badge.svg)](https://docs.rs/alphabet_detector)

### Detects 388 alphabets in 170 scripts
> one spoken language can be written in multiple scripts, so it will be detected as a different alphabet/language

> look at the [alphabet.rs](https://github.com/RoDmitry/alphabet_detector/blob/main/src/lang/alphabet.rs#L68) to understand what languages have already defined alphabets. Some of them need validation

Separates words in text (from iterator `CharIndices`), and detects language of words by used alphabets (chars).

Warning: can return words with chars from the Unicode private area (for `Yoruba` or `Nuer` language)

Alpha version (Work In Progress)
