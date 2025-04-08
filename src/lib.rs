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
pub use words::{WordIterator, WordLang};
