use crate::helper::skip_eq;
use proc_macro2::{Span, TokenTree};
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Meta};

const NAME: &str = "scr";

pub(super) fn script_derive_inner(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
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

    let mut match_to_str = Vec::new();
    let mut match_to_code = Vec::new();
    let mut match_from_code = Vec::new();
    let mut match_from_bytes = Vec::new();
    let mut str_variants = Vec::new();

    for variant in variants {
        let ident = variant.ident;
        let mut short = None;
        let mut code = None;

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
                                    short = Some(quote! { #v });
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
                        "code" => {
                            skip_eq(&i, &mut tokens)?;
                            match tokens.next() {
                                Some(TokenTree::Literal(v)) => {
                                    code = Some(quote! { #v });
                                }
                                Some(tt) => {
                                    return Err(Error::new(
                                        tt.span(),
                                        format!("Unexpected \"{tt}\""),
                                    ))
                                }
                                _ => return Err(Error::new(i.span(), "No code provided")),
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

        let short = short.ok_or_else(|| Error::new(ident.span(), "No short name provided"))?;
        let code = code.ok_or_else(|| Error::new(ident.span(), "No code provided"))?;

        match_to_str.push(quote! {
            #name::#ident #params => #short
        });
        match_to_code.push(quote! {
            #name::#ident #params => #code
        });
        match_from_code.push(quote! {
            #code => ::core::option::Option::Some(#name::#ident #params)
        });
        match_from_bytes.push(quote! {
            v if ::concat_const::eq_bytes(v, #short.as_bytes()) => ::core::option::Option::Some(#name::#ident #params)
        });
        str_variants.push(quote! { #short });
    }

    match_from_code.push(quote! { _ => ::core::option::Option::None });
    match_from_bytes.push(quote! { _ => ::core::option::Option::None });

    Ok(quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            const VARIANTS: &'static [&'static str] = &[#(#str_variants),*];

            /// 10-bit code (ISO 15924 number)
            #[inline]
            pub const fn into_code(self) -> u16 {
                match self {
                    #(#match_to_code),*
                }
            }
            /// ISO 15924 code string
            #[inline]
            pub const fn into_str(self) -> &'static str {
                match self {
                    #(#match_to_str),*
                }
            }
            /// 10-bit code (ISO 15924 number)
            #[inline]
            pub const fn from_code(v: u16) -> Option<Self> {
                match v {
                    #(#match_from_code),*
                }
            }
            /// ISO 15924 code string
            #[inline]
            pub const fn from_bytes(v: &[u8]) -> Option<Self> {
                match v {
                    #(#match_from_bytes),*
                }
            }
            /// ISO 15924 code string
            #[inline]
            pub const fn from_str(s: &str) -> Option<Self> {
                Self::from_bytes(s.as_bytes())
            }
        }
    })
}
