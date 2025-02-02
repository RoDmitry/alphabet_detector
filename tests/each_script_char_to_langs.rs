// cargo test each_script_char_to_langs --features test_chars -- --exact
#![cfg(feature = "test_chars")]

use alphabet_detector::*;
use strum::IntoEnumIterator;

// tests chars_combine in each script
#[test]
fn each_script_char_to_langs() {
    for script in Script::iter() {
        script_char_to_langs(script, char::default());
    }
}
