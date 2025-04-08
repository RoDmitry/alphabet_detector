# Alphabet Detector

[![Crate](https://img.shields.io/crates/v/alphabet_detector.svg)](https://crates.io/crates/alphabet_detector)
[![API](https://docs.rs/alphabet_detector/badge.svg)](https://docs.rs/alphabet_detector)

## Detects 400 alphabets of 323 languages in 170 scripts

> One language can be written in multiple scripts, so it will be detected as a different [`ScriptLanguage`](https://docs.rs/alphabet_detector/latest/alphabet_detector/enum.ScriptLanguage.html) (language + script)

Splits text (iterator `CharIndices`) to words, and detects [`ScriptLanguage`](https://docs.rs/alphabet_detector/latest/alphabet_detector/enum.ScriptLanguage.html) (language + script) of words by used letters (chars).

### Examples

To split `text` to the iterator of [`WordLang`](https://docs.rs/alphabet_detector/latest/alphabet_detector/words/struct.WordLang.html):
```rust
let word_iter = words::from_ch_ind::<Vec<char>>(text.char_indices());
```

If you don't need individual words, but just want to analyze a full text:
```rust
let (all_words, all_langs) = fulltext_filter_with_margin_sorted::<Vec<char>, 95>(text.char_indices());
```

It will give you all [`Word`](https://docs.rs/alphabet_detector/latest/alphabet_detector/struct.Word.html)s (`Vec<Word<Vec<char>>>`) of `text` and `Vec<(ScriptLanguage, u32)>` filtered with a less then 5% margin for an error.

### Extras

Look at the [alphabets.rs](https://github.com/RoDmitry/alphabet_detector/blob/main/src/lang/alphabets.rs#L73) to understand what languages have already defined alphabets. Some of them need validation.

Warning: can return words with chars from the Unicode private area (for example `Lingala`, `Nuer` or `Yoruba` languages), because of char normalization (composition with `Inherited`), and there are no such chars defined in Unicode.
