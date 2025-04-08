#![cfg_attr(feature = "files_read", feature(string_into_chars))]

pub mod ch_norm_iter;
mod fulltext;
mod lang;
mod langs_filter;
#[cfg(feature = "files_read")]
pub mod read_iter;
pub mod word_iter;

pub use ch_norm_iter::{CharData, CharNormalizingIterator};
pub use fulltext::*;
pub use lang::*;
pub use langs_filter::*;
pub use word_iter::{WordIterator, WordLangsData};
