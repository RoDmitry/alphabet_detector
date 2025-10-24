use std::str::FromStr;

use crate::helper::skip_eq;
use proc_macro2::{Literal, Span, TokenTree};
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Meta};

const NAME: &str = "language";

pub(super) fn language_derive_inner(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let variants = match input.data {
        Data::Enum(v) => v.variants,
        _ => {
            return Err(Error::new(
                Span::call_site(),
                "This macro only supports enums",
            ))
        }
    };

    let mut match_to_str = Vec::with_capacity(variants.len());
    let mut match_to_2letter = Vec::with_capacity(variants.len());
    let mut match_to_code = Vec::with_capacity(variants.len());
    let mut match_from_code = Vec::with_capacity(variants.len() + variants.len() / 2);
    let mut match_from_bytes = Vec::with_capacity(variants.len() + variants.len() / 2);
    let mut str_variants = Vec::with_capacity(variants.len());

    for variant in variants {
        let ident = variant.ident;
        let mut short = None;
        let mut old_shorts = Vec::new();
        let mut shortest = None;

        let tokens = variant
            .attrs
            .into_iter()
            .filter_map(|a| match a.meta {
                Meta::List(list) => Some(list),
                _ => None,
            })
            .find(|list| list.path.is_ident(NAME))
            .map(|list| list.tokens.into_iter());

        if let Some(mut tokens) = tokens {
            while let Some(tt) = tokens.next() {
                if let TokenTree::Ident(i) = tt {
                    match i.to_string().as_str() {
                        "short" => {
                            skip_eq(&i, &mut tokens)?;
                            match tokens.next() {
                                Some(TokenTree::Literal(v)) => {
                                    short = Some(v.to_string());
                                }
                                Some(tt) => {
                                    return Err(Error::new(
                                        tt.span(),
                                        format!("Unexpected \"{tt}\""),
                                    ))
                                }
                                _ => return Err(Error::new(i.span(), "No short name provided")),
                            }
                        }
                        "old_short" => {
                            skip_eq(&i, &mut tokens)?;
                            match tokens.next() {
                                Some(TokenTree::Literal(v)) => {
                                    old_shorts.push(v.to_string());
                                }
                                Some(TokenTree::Group(g)) => {
                                    for gt in g.stream() {
                                        match gt {
                                            TokenTree::Literal(v) => {
                                                old_shorts.push(v.to_string());
                                            }
                                            TokenTree::Punct(p) => {
                                                let ch = p.as_char();
                                                if ch != ',' {
                                                    return Err(syn::Error::new(
                                                        p.span(),
                                                        format!("Unexpected \"{ch}\", expected comma \",\""),
                                                    ));
                                                }
                                            }
                                            _ => {
                                                return Err(Error::new(
                                                    i.span(),
                                                    "No old short name provided",
                                                ))
                                            }
                                        }
                                    }
                                }
                                Some(tt) => {
                                    return Err(Error::new(
                                        tt.span(),
                                        format!("Unexpected \"{tt}\""),
                                    ))
                                }
                                _ => {
                                    return Err(Error::new(i.span(), "No old short name provided"))
                                }
                            }
                        }
                        "shortest" => {
                            skip_eq(&i, &mut tokens)?;
                            match tokens.next() {
                                Some(TokenTree::Literal(v)) => {
                                    shortest = Some(v.to_string());
                                }
                                Some(tt) => {
                                    return Err(Error::new(
                                        tt.span(),
                                        format!("Unexpected \"{tt}\""),
                                    ))
                                }
                                _ => return Err(Error::new(i.span(), "No shortest name provided")),
                            }
                        }
                        v => {
                            return Err(Error::new(i.span(), format!("Unexpected \"{v}\"")));
                        }
                    }
                }
                if let Some(TokenTree::Punct(p)) = tokens.next() {
                    let ch = p.as_char();
                    if ch != ',' {
                        return Err(syn::Error::new(
                            p.span(),
                            format!("Unexpected \"{ch}\", expected comma \",\""),
                        ));
                    }
                }
            }
        }

        let params = match variant.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(..) => quote! { (..) },
            Fields::Named(..) => quote! { {..} },
        };

        let short_text = short.ok_or_else(|| Error::new(ident.span(), "No short name provided"))?;
        if short_text.len() > 6 {
            return Err(Error::new(ident.span(), "Too long short name"));
        }

        let short = Literal::from_str(&short_text)?;
        let short = quote! { #short };

        match_to_str.push(quote! {
            #name::#ident #params => #short
        });
        match_from_bytes.push(quote! {
            v if ::concat_const::eq_bytes(v, #short.as_bytes()) => ::core::option::Option::Some(#name::#ident #params)
        });

        let mut short_name_chars = short_text[1..short_text.len() - 1].chars();
        let mut code: u32 = short_name_chars.next().unwrap() as u32 & 0b1_1111;
        for char in short_name_chars {
            code <<= 5;
            code |= char as u32 & 0b1_1111;
        }

        match_to_code.push(quote! {
            #name::#ident #params => #code
        });
        match_from_code.push(quote! {
            #code => ::core::option::Option::Some(#name::#ident #params)
        });

        // same as short
        for old_short_text in old_shorts {
            let old_short = Literal::from_str(&old_short_text)?;
            let old_short = quote! { #old_short };

            match_from_bytes.push(quote! {
                v if ::concat_const::eq_bytes(v, #old_short.as_bytes()) => ::core::option::Option::Some(#name::#ident #params)
            });

            let mut old_short_chars = old_short_text[1..old_short_text.len() - 1].chars();
            let mut code: u32 = old_short_chars.next().unwrap() as u32 & 0b1_1111;
            for char in old_short_chars {
                code <<= 5;
                code |= char as u32 & 0b1_1111;
            }

            match_from_code.push(quote! {
                #code => ::core::option::Option::Some(#name::#ident #params)
            });
        }

        if let Some(s) = shortest {
            if s.len() > 4 {
                return Err(Error::new(ident.span(), "Too long shortest name"));
            }

            let s = Literal::from_str(&s)?;
            match_to_2letter.push(quote! {
                #name::#ident #params => ::core::option::Option::Some(#s)
            });
        }
        str_variants.push(quote! { #short });
    }

    match_from_code.push(quote! { _ => ::core::option::Option::None });
    match_from_bytes.push(quote! { _ => ::core::option::Option::None });

    Ok(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            const VARIANTS: &'static [&'static str] = &[#(#str_variants),*];

            /// 20-bit code (compacted ISO 639-3 code)
            #[inline]
            pub const fn into_code(self) -> u32 {
                match self {
                    #(#match_to_code),*
                }
            }
            /// ISO 639-3 code string
            #[inline]
            pub const fn into_str(self) -> &'static str {
                match self {
                    #(#match_to_str),*
                }
            }
            /// ISO 639-1 code string
            #[inline]
            pub const fn into_2letter(self) -> Option<&'static str> {
                match self {
                    #(#match_to_2letter),*,
                    _ => None
                }
            }
            /// 20-bit code (compacted ISO 639-3 code)
            #[inline]
            pub const fn from_code(v: u32) -> Option<Self> {
                match v {
                    #(#match_from_code),*
                }
            }
            /// ISO 639-3 code string
            #[inline]
            pub const fn from_bytes(v: &[u8]) -> Option<Self> {
                match v {
                    #(#match_from_bytes),*
                }
            }
            /// ISO 639-3 code string
            #[inline]
            pub const fn from_str(s: &str) -> Option<Self> {
                Self::from_bytes(s.as_bytes())
            }
        }
    })
}
