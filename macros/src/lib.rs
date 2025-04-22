use proc_macro::TokenStream;
use syn::parse_macro_input;

mod alphabet_match;
mod helper;
mod language_derive;
mod script_derive;
mod script_lang_derive;

#[proc_macro]
pub fn alphabet_match(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let tokens =
        alphabet_match::alphabet_match_inner(input).unwrap_or_else(|err| err.to_compile_error());

    TokenStream::from(tokens)
}

/// only to use rust-analyzer expansion
#[test]
fn expand() {
    #[cfg(not(test))]
    alphabet_match!([(SomeEnum::A, ['a', 'b', 'c']), (SomeEnum::B, ['a', 'c']),]);
}

#[proc_macro_derive(ScriptLanguage, attributes(slang))]
pub fn script_lang_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let tokens = script_lang_derive::script_lang_derive_inner(input)
        .unwrap_or_else(|err| err.to_compile_error());

    TokenStream::from(tokens)
}

#[proc_macro_derive(Script, attributes(scr))]
pub fn script_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let tokens =
        script_derive::script_derive_inner(input).unwrap_or_else(|err| err.to_compile_error());

    TokenStream::from(tokens)
}

#[proc_macro_derive(Language, attributes(language))]
pub fn language_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let tokens =
        language_derive::language_derive_inner(input).unwrap_or_else(|err| err.to_compile_error());

    TokenStream::from(tokens)
}
