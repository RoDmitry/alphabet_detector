// cargo test test_script_chars --features test_chars -- --exact
#![cfg(feature = "test_chars")]

use alphabet_detector::*;
use strum::IntoEnumIterator;

// `script_char_to_langs` has a test call to `ch_norm_iter::test_chars` with all chars of the script
#[test]
fn test_script_chars() {
    for script in Script::iter() {
        script_char_to_langs(script, char::default());
    }
}
