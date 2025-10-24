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

    let mut match_to_language = Vec::with_capacity(variants.len());
    let mut match_to_script = Vec::with_capacity(variants.len());
    let mut match_to_parts = Vec::with_capacity(variants.len());
    let mut match_to_str = Vec::with_capacity(variants.len());
    let mut match_to_code = Vec::with_capacity(variants.len());
    let mut match_from_code = Vec::with_capacity(variants.len() + 1);
    let mut match_from_parts = Vec::with_capacity(variants.len() + 1);
    let mut match_from_bytes = Vec::with_capacity(variants.len() + 1);
    let mut str_variants = Vec::with_capacity(variants.len());
    let mut lang_to_script_langs: AHashMap<String, Vec<_>> =
        AHashMap::with_capacity(variants.len());

    for variant in variants {
        let ident = variant.ident;
        let mut language = None;
        let mut script = None;

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
                                Some(tt) => {
                                    return Err(Error::new(
                                        tt.span(),
                                        format!("Unexpected \"{tt}\""),
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

        let script = script.ok_or_else(|| Error::new(ident.span(), "No script provided"))?;
        match_to_script.push(quote! {
            #name::#ident #params => Script::#script
        });
        match_to_parts.push(quote! {
            #name::#ident #params => (Language::#lang, Script::#script)
        });
        match_to_str.push(quote! {
            #name::#ident #params => ::concat_const::concat!(
                Language::#lang.into_str(),
                Script::#script.into_str()
            )
        });
        match_to_code.push(quote! {
            #name::#ident #params => (Language::#lang.into_code() << 10) | Script::#script.into_code() as u32
        });
        match_from_code.push(quote! {
            v if v == const { (Language::#lang.into_code() << 10) | Script::#script.into_code() as u32 } =>
                ::core::option::Option::Some(#name::#ident #params)
        });
        match_from_parts.push(quote! {
            (Language::#lang, Script::#script) => ::core::option::Option::Some(#name::#ident #params)
        });
        match_from_bytes.push(quote! {
            v if ::concat_const::eq_bytes(v, ::concat_const::concat_bytes!(
                Language::#lang.into_str().as_bytes(),
                Script::#script.into_str().as_bytes()
            )) => ::core::option::Option::Some(#name::#ident #params)
        });
        str_variants.push(quote! {
            ::concat_const::concat!(
                Language::#lang.into_str(), Script::#script.into_str()
            )
        });
    }

    let match_lang_to_script_langs = lang_to_script_langs.into_iter().map(|(lang, v)| {
        let l = Ident::new(&lang, Span::call_site());
        quote! { Language::#l => &[#(#v),*] }
    });
    match_from_code.push(quote! { _ => ::core::option::Option::None });
    match_from_parts.push(quote! { _ => ::core::option::Option::None });
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
        impl #impl_generics From<#name #ty_generics> for Script #where_clause {
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
            pub const fn into_parts(self) -> (Language, Script) {
                match self {
                    #(#match_to_parts),*
                }
            }
            /// ISO 639-3 + ISO 15924 codes string
            #[inline]
            pub const fn into_str(self) -> &'static str {
                match self {
                    #(#match_to_str),*
                }
            }
            /// 30-bit code (compacted ISO 639-3 code, ISO 15924 number)
            #[inline]
            pub const fn into_code(self) -> u32 {
                match self {
                    #(#match_to_code),*
                }
            }
            /// 30-bit code (compacted ISO 639-3 code, ISO 15924 number)
            #[inline]
            pub const fn from_code(v: u32) -> Option<Self> {
                match v {
                    #(#match_from_code),*
                }
            }
            #[inline]
            pub const fn from_parts(v: (Language, Script)) -> Option<Self> {
                match v {
                    #(#match_from_parts),*
                }
            }
            /// ISO 639-3 + ISO 15924 codes string
            #[inline]
            pub const fn from_bytes(v: &[u8]) -> Option<Self> {
                match v {
                    #(#match_from_bytes),*
                }
            }
            /// ISO 639-3 + ISO 15924 codes string
            #[inline]
            pub const fn from_str(s: &str) -> Option<Self> {
                Self::from_bytes(s.as_bytes())
            }
        }
    })
}
