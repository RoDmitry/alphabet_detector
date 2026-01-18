//! # Natural language alphabet detection library
//!
//! ## Detects 426 alphabets of 348 languages in 174 scripts
//!
//! One language can be written in multiple scripts, so it will be detected as a different [`ScriptLanguage`](enum.ScriptLanguage.html) (language + script).
//!
//! Does not have any models, just matches the alphabet. Not recommended to use as a standalone detector.
//! It's more like a word separator + language prefilter for an actual language detector ([`Langram`](https://docs.rs/langram/latest/langram)).
//!
//! Splits text (iterator `CharIndices`) to words, and detects [`ScriptLanguage`](enum.ScriptLanguage.html)s (language + script) of words by used letters (chars).
//!
//! `ISO 639-3` (using [`Language`](enum.Language.html#implementations)) and `ISO 15924` (using [`Script`](enum.Script.html#implementations))
//! are implemented, also combined using [`ScriptLanguage`](enum.ScriptLanguage.html#implementations).
//!
//! # Examples
//! To split `text` to the iterator of [`Word`](words/struct.Word.html):
//! ```rust
//! use alphabet_detector::words;
//!
//! let text = "test text";
//! let word_iter = words::from_ch_ind::<Vec<char>>(text.char_indices());
//! ```
//!
//! If you don't need individual words, but just want to analyze a full text:
//! ```rust
//! use alphabet_detector::fulltext_filter_with_margin_sorted;
//!
//! let text = "test text";
//! let (all_words, all_langs, _) = fulltext_filter_with_margin_sorted::<Vec<char>, 95>(text.char_indices());
//! ```
//!
//! It will give you all [`Word`](struct.Word.html)s (`Vec<Word<Vec<char>>>`) of `text` and `Vec<(ScriptLanguage, u32)>` filtered with a less then 5% margin for an error.
//!
//! Instead of `Vec<char>` you can use [other types](words/trait.WordBuf.html#foreign-impls) of words.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(feature = "files_read", feature(string_into_chars))]

pub mod ch_norm;
mod filter;
mod fulltext;
mod lang;
#[cfg(feature = "files_read")]
pub mod reader;
pub mod words;

pub use ch_norm::{CharData, CharNormalizingIterator};
pub use filter::*;
pub use fulltext::*;
pub use lang::*;
pub use words::{Word, WordIterator};
