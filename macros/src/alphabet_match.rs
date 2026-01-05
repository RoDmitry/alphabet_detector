use ahash::AHashMap;
#[cfg(debug_assertions)]
use ahash::AHashSet;
use quote::quote;
use syn::{spanned::Spanned, Error, Expr, ExprArray, ExprTuple, Lit};

pub(super) fn alphabet_match_inner(input: ExprArray) -> syn::Result<proc_macro2::TokenStream> {
    let mut value_to_keys = AHashMap::new();

    let mut keys_all = Vec::new();
    let mut unknown_lang = None;
    // Parse the input array of tuples
    for tuple in input.elems {
        match tuple {
            Expr::Tuple(ExprTuple { elems, .. }) => {
                // We assume the key can be any expression, so we just clone it
                let key = &elems[0];
                keys_all.push(key.clone());

                // Handle the array of values associated with the key
                match &elems[1] {
                    Expr::Array(ExprArray { elems: values, .. }) => {
                        #[cfg(debug_assertions)]
                        let mut values_set: AHashSet<char> = AHashSet::new();

                        for val in values {
                            let value = match val {
                                Expr::Lit(expr_lit) => match &expr_lit.lit {
                                    Lit::Char(lit_char) => lit_char.value(),
                                    v => {
                                        return Err(Error::new(
                                            v.span(),
                                            "Expected char literal for value",
                                        ))
                                    }
                                },
                                v => {
                                    return Err(Error::new(
                                        v.span(),
                                        "Expected char literal for value",
                                    ))
                                }
                            };

                            #[cfg(debug_assertions)]
                            if let Expr::Path(ep) = key {
                                let ki = &ep.path.segments[1].ident;
                                if !values_set.insert(value) {
                                    return Err(Error::new(
                                        val.span(),
                                        format!(
                                            "Char literal: {value} was already added for key: {ki}",
                                        ),
                                    ));
                                }
                            }

                            value_to_keys
                                .entry(value)
                                .or_insert_with(Vec::new)
                                .push(key.clone()); // Clone the key to store it in the map
                        }
                    }
                    v => {
                        return Err(Error::new(v.span(), "Expected an array of values"));
                    }
                }
            }
            Expr::Path(p) => {
                unknown_lang = Some(p);
            }
            v => {
                return Err(Error::new(v.span(), "Expected a tuple"));
            }
        }
    }

    // Generate match arms
    let arms = value_to_keys
        .iter()
        .filter(|(_, k)| k.len() < keys_all.len())
        .map(|(value, keys)| {
            quote! {
                #value => &[#(#keys),*],
            }
        });

    let other = if let Some(l) = unknown_lang {
        let chars_other = value_to_keys
            .iter()
            .filter(|(_, k)| k.len() >= keys_all.len())
            .map(|(c, _)| quote! { #c });

        quote! {
            #(#chars_other)|* => &[#(#keys_all),*],
            _ => &[#l]
        }
    } else {
        quote! { _ => &[#(#keys_all),*] }
    };

    let chars = value_to_keys.keys().map(|c| quote! { #c });
    // Generate the entire match block
    let expanded = quote! {{
        #[cfg(all(debug_assertions, feature = "test_chars"))]
        {
            let chars = [#(#chars),*];
            test_chars(script, &chars);
        }
        match ch {
            #(#arms)*
            #other
        }
    }};

    Ok(expanded)
}
