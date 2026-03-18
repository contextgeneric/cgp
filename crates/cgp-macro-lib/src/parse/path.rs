use proc_macro2::{TokenStream, TokenTree};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, Dot, Star};
use syn::{Ident, Type, braced};

use crate::symbol::symbol_from_string_spanned;

pub struct ComponentPath {
    pub paths: Vec<Type>,
}

impl Parse for ComponentPath {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path_head = PathHead::parse(input)?;

        let mut paths = Vec::new();

        for path in path_head.to_types() {
            let path_type: Type = syn::parse2(path)?;
            paths.push(path_type);
        }

        Ok(Self { paths })
    }
}

// pub struct ComponentPath {
//     elements: Punctuated<PathElement, Dot>,
// }

// impl Parse for ComponentPath {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         let elements = Punctuated::parse_terminated(input)?;
//         Ok(Self { elements })
//     }
// }

// impl ComponentPath {
//     pub fn to_types(&self) -> Vec<TokenStream> {

//         todo!()
//     }
// }

// pub fn path_elements_to_types(mut elements: impl Iterator<Item = PathElement>) -> Vec<TokenStream> {
//     if let Some(element) = elements.next() {
//         let types = element.to_types();
//         let mut rest_types = path_elements_to_types(elements);
//         todo!()
//     } else {
//         vec![ quote! { PathNil } ]
//     }

// }

pub enum PathHead {
    Type(Type, Box<PathHead>),
    Symbol(Ident, Box<PathHead>),
    Group(Punctuated<PathHead, Comma>),
    Wildcard,
    Nil,
}

impl PathHead {
    pub fn to_types(&self) -> Vec<TokenStream> {
        match self {
            Self::Type(path_type, rest) => {
                let rest_types = rest.to_types();
                rest_types
                    .into_iter()
                    .map(|rest_type| {
                        quote! { PathCons< #path_type , #rest_type > }
                    })
                    .collect()
            }
            Self::Symbol(ident, rest) => {
                let ident_str = ident.to_string();
                let path_type = symbol_from_string_spanned(ident.span(), &ident_str);

                let rest_types = rest.to_types();
                rest_types
                    .into_iter()
                    .map(|rest_type| {
                        quote! { PathCons< #path_type , #rest_type > }
                    })
                    .collect()
            }
            Self::Group(paths) => paths.iter().flat_map(|path| path.to_types()).collect(),
            Self::Wildcard => {
                vec![quote! { __Wildcard__ }]
            }
            Self::Nil => {
                vec![quote! { PathNil }]
            }
        }
    }
}

impl Parse for PathHead {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(Self::Nil)
        } else if input.peek(Star) {
            let _: Star = input.parse()?;
            Ok(Self::Wildcard)
        } else if input.peek(Brace) {
            let body;
            braced!(body in input);

            let group = Punctuated::parse_terminated(&body)?;

            Ok(Self::Group(group))
        } else {
            let path_type: Type = input.parse()?;

            let rest_path = if input.peek(Dot) {
                let _: Dot = input.parse()?;
                Box::new(Self::parse(input)?)
            } else {
                Box::new(Self::Nil)
            };

            if let Some(path_ident) = path_type_as_ident(&path_type) {
                Ok(Self::Symbol(path_ident, rest_path))
            } else {
                Ok(Self::Type(path_type, rest_path))
            }
        }
    }
}

pub fn path_type_as_ident(path_type: &Type) -> Option<Ident> {
    let path_tokens = path_type.to_token_stream().into_iter().collect::<Vec<_>>();
    let [path_token]: [TokenTree; 1] = path_tokens.try_into().ok()?;

    if let TokenTree::Ident(path_ident) = path_token {
        let path_str = path_ident.to_string();
        if let Some(path_char) = path_str.chars().next() {
            if path_char.is_ascii_lowercase() {
                return Some(path_ident);
            }
        }
    }

    None
}
