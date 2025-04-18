use crate::helper::skip_eq;
use ahash::AHashMap;
use proc_macro2::{Ident, Span, TokenTree};
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Meta};

const NAME: &str = "slang";

pub(super) fn script_lang_derive_inner(
    input: DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
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

    let mut match_to_language = Vec::new();
    let mut match_to_script = Vec::new();
    let mut match_to_script_str = Vec::new();
    let mut match_to_str = Vec::new();
    let mut match_from_bytes = Vec::new();
    let mut str_variants = Vec::new();
    let mut lang_to_script_langs: AHashMap<String, Vec<_>> = AHashMap::new();

    for variant in variants {
        let ident = variant.ident;
        let mut language = None;
        let mut script = None;
        let mut script_str = None;

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
                        "script" => {
                            skip_eq(&i, &mut tokens)?;
                            match tokens.next() {
                                Some(TokenTree::Ident(v)) => {
                                    script = Some(quote! { #v });
                                }
                                Some(TokenTree::Literal(v)) => {
                                    script_str = Some(quote! { #v });
                                }
                                Some(tt) => {
                                    return Err(Error::new(
                                        tt.span(),
                                        format!("Unexpected \"{}\"", tt),
                                    ))
                                }
                                _ => return Err(Error::new(i.span(), "No script provided")),
                            }
                        }
                        "lang" => {
                            skip_eq(&i, &mut tokens)?;
                            if let Some(TokenTree::Ident(v)) = tokens.next() {
                                language = Some(v.to_string());
                            }
                        }
                        v => {
                            return Err(Error::new(i.span(), format!("Unexpected \"{}\"", v)));
                        }
                    }
                }
                if let Some(TokenTree::Punct(p)) = tokens.next() {
                    let ch = p.as_char();
                    if ch != ',' {
                        return Err(syn::Error::new(
                            p.span(),
                            format!("Unexpected \"{}\", expected comma \",\"", ch),
                        ));
                    }
                }
            }
        }

        let language = language.unwrap_or_else(|| ident.to_string());
        let lang = Ident::new(&language, Span::call_site());
        let lang = quote! { #lang };

        let params = match variant.fields {
            Fields::Unit => quote! {},
            Fields::Unnamed(..) => quote! { (..) },
            Fields::Named(..) => quote! { {..} },
        };

        lang_to_script_langs
            .entry(language)
            .or_default()
            .push(quote! { #name::#ident #params });

        match_to_language.push(quote! {
            #name::#ident #params => Language::#lang
        });

        if let Some(scr) = script {
            match_to_script.push(quote! {
                #name::#ident #params => ::core::option::Option::Some(Script::#scr)
            });
            match_to_script_str.push(quote! {
                #name::#ident #params => Script::#scr.into_str()
            });
            match_to_str.push(quote! {
                #name::#ident #params => ::concat_const::concat!(
                    Language::#lang.into_str(),
                    Script::#scr.into_str()
                )
            });
            match_from_bytes.push(quote! {
                v if ::concat_const::eq_bytes(v, ::concat_const::concat_bytes!(
                    Language::#lang.into_str().as_bytes(),
                    Script::#scr.into_str().as_bytes()
                )) => ::core::option::Option::Some(#name::#ident #params)
            });
            str_variants.push(quote! {
                ::concat_const::concat!(
                    Language::#lang.into_str(), Script::#scr.into_str()
                )
            });
        } else if let Some(scr) = script_str {
            match_to_script_str.push(quote! {
                #name::#ident #params => #scr
            });
            match_to_str.push(quote! {
                #name::#ident #params => ::concat_const::concat!(
                    Language::#lang.into_str(), #scr
                )
            });
            match_from_bytes.push(quote! {
                v if ::concat_const::eq_bytes(v, ::concat_const::concat_bytes!(
                    Language::#lang.into_str().as_bytes(), #scr.as_bytes()
                )) => ::core::option::Option::Some(#name::#ident #params)
            });
            str_variants.push(quote! {
                ::concat_const::concat!(Language::#lang.into_str(), #scr)
            });
        } else {
            match_to_script_str.push(quote! {
                #name::#ident #params => ""
            });
            match_to_str.push(quote! {
                #name::#ident #params => Language::#lang.into_str()
            });
            match_from_bytes.push(quote! {
                v if ::concat_const::eq_bytes(
                    v,
                    Language::#lang.into_str().as_bytes()
                ) => ::core::option::Option::Some(#name::#ident #params)
            });
            str_variants.push(quote! {
                Language::#lang.into_str()
            });
        }
    }

    let match_lang_to_script_langs = lang_to_script_langs.into_iter().map(|(lang, v)| {
        let l = Ident::new(&lang, Span::call_site());
        quote! { Language::#l => &[#(#v),*] }
    });
    match_to_script.push(quote! { _ => ::core::option::Option::None });
    match_from_bytes.push(quote! { _ => ::core::option::Option::None });

    Ok(quote! {
        impl #impl_generics From<#name #ty_generics> for Language #where_clause {
            #[inline]
            fn from(sl: #name) -> Self {
                match sl {
                    #(#match_to_language),*
                }
            }
        }
        impl #impl_generics From<Language> for &'static [#name #ty_generics] #where_clause {
            #[inline]
            fn from(l: Language) -> Self {
                match l {
                    #(#match_lang_to_script_langs),*
                }
            }
        }
        impl #impl_generics From<#name #ty_generics> for Option<Script> #where_clause {
            #[inline]
            fn from(sl: #name) -> Self {
                match sl {
                    #(#match_to_script),*
                }
            }
        }
        impl #impl_generics #name #ty_generics #where_clause {
            const VARIANTS: &'static [&'static str] = &[#(#str_variants),*];

            #[inline]
            pub const fn into_script_str(self) -> &'static str {
                match self {
                    #(#match_to_script_str),*
                }
            }
            #[inline]
            pub const fn into_str(self) -> &'static str {
                match self {
                    #(#match_to_str),*
                }
            }
            #[inline]
            pub const fn from_bytes(v: &[u8]) -> Option<Self> {
                match v {
                    #(#match_from_bytes),*
                }
            }
            #[inline]
            pub const fn from_str(s: &str) -> Option<Self> {
                Self::from_bytes(s.as_bytes())
            }
        }
    })
}
