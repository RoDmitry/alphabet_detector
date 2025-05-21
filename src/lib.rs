//! # Natural language alphabet detection library
//!
//! ## Detects 401 alphabets of 323 languages in 170 scripts
//!
//! One language can be written in multiple scripts, so it will be detected as a different [`ScriptLanguage`](enum.ScriptLanguage.html) (language + script).
//!
//! Does not have any models, just matches the alphabet. Not recommended to use as a standalone detector.
//! It's more like a word separator + language prefilter for an actual language detector ([`Langram`](https://docs.rs/langram/latest/langram)).
//! Splits text (iterator `CharIndices`) to words, and detects [`ScriptLanguage`](enum.ScriptLanguage.html)s (language + script) of words by used letters (chars).
//!
//! `ISO 639-3` (using [`Language`](enum.Language.html#implementations)) and `ISO 15924` (using [`Script`](enum.Script.html#implementations))
//! are implemented, also combined using [`ScriptLanguage`](enum.ScriptLanguage.html#implementations).

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
