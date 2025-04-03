use proc_macro2::{Ident, TokenTree};

pub(crate) fn skip_eq(i: Ident, tt_iter: &mut impl Iterator<Item = TokenTree>) -> syn::Result<()> {
    match tt_iter.next() {
        Some(TokenTree::Punct(p)) if p.as_char() == '=' => Ok(()),
        Some(tt) => Err(syn::Error::new_spanned(
            &tt,
            format!("Unexpected \"{}\", expected equal sign \"=\"", tt),
        )),
        None => Err(syn::Error::new_spanned(i, "expected: equal sign \"=\"")),
    }
}
