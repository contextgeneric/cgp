use proc_macro2::{TokenStream, TokenTree};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, Dot, Lt};
use syn::{Ident, Type, braced, parse_quote, parse2};

use crate::parse::ImplGenerics;
use crate::symbol::symbol_from_string_spanned;

pub struct ComponentPaths {
    pub paths: Vec<ComponentPath<Type>>,
}

impl Parse for ComponentPaths {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path_head = PathHead::parse(input)?;

        if let PathHead::Wildcard = path_head {
            return Err(syn::Error::new(
                input.span(),
                "Expected at least one path element",
            ));
        }

        let mut paths = Vec::new();

        for path in path_head.to_paths() {
            let path_type: Type = syn::parse2(path.path_type)?;
            paths.push(ComponentPath {
                path_type,
                generics: path.generics,
            });
        }

        Ok(Self { paths })
    }
}

pub struct ComponentPath<Path> {
    pub path_type: Path,
    pub generics: ImplGenerics,
}

pub enum PathHead {
    Type(Option<ImplGenerics>, Box<PathType>, Box<PathHead>),
    Group(Punctuated<PathHead, Comma>),
    Wildcard,
}

impl PathHead {
    pub fn to_paths(&self) -> Vec<ComponentPath<TokenStream>> {
        match self {
            Self::Type(generics, path_type, rest) => {
                let rest_types = rest.to_paths();

                prepend_path(
                    path_type.path_type.to_token_stream(),
                    generics.clone(),
                    rest_types,
                )
            }
            Self::Group(paths) => paths.iter().flat_map(|path| path.to_paths()).collect(),
            Self::Wildcard => {
                vec![ComponentPath {
                    path_type: quote! { __Wildcard__ },
                    generics: parse_quote! { <__Wildcard__> },
                }]
            }
        }
    }
}

pub fn prepend_path(
    path_type: TokenStream,
    generics: Option<ImplGenerics>,
    rest_types: Vec<ComponentPath<TokenStream>>,
) -> Vec<ComponentPath<TokenStream>> {
    rest_types
        .into_iter()
        .map(|mut path| {
            let rest_tokens = path.path_type;

            if let Some(generics) = &generics {
                path.generics
                    .generics
                    .params
                    .extend(generics.generics.params.clone());
            }

            let new_path = quote! { PathCons< #path_type , #rest_tokens > };
            ComponentPath {
                path_type: new_path,
                generics: path.generics,
            }
        })
        .collect()
}

impl Parse for PathHead {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(Self::Wildcard)
        } else if input.peek(Brace) {
            let body;
            braced!(body in input);

            let group = Punctuated::parse_terminated(&body)?;

            Ok(Self::Group(group))
        } else {
            let generics = if input.peek(Lt) {
                Some(input.parse()?)
            } else {
                None
            };

            let path_type: PathType = input.parse()?;

            let rest_path = if input.peek(Dot) {
                let _: Dot = input.parse()?;
                Box::new(Self::parse(input)?)
            } else {
                Box::new(Self::Wildcard)
            };

            Ok(Self::Type(generics, Box::new(path_type), rest_path))
        }
    }
}

pub fn path_type_as_ident(path_type: &Type) -> Option<Ident> {
    let path_tokens = path_type.to_token_stream().into_iter().collect::<Vec<_>>();
    let [path_token]: [TokenTree; 1] = path_tokens.try_into().ok()?;

    if let TokenTree::Ident(path_ident) = path_token {
        let path_str = path_ident.to_string();
        if let Some(path_char) = path_str.chars().next()
            && path_char.is_ascii_lowercase()
            && !is_primitive_type(&path_str)
        {
            return Some(path_ident);
        }
    }

    None
}

pub struct PathType {
    pub path_type: Type,
}

impl Parse for PathType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path_type: Type = input.parse()?;

        if let Some(path_ident) = path_type_as_ident(&path_type) {
            let path_symbol = parse2(symbol_from_string_spanned(
                path_ident.span(),
                &path_ident.to_string(),
            ))?;
            Ok(Self {
                path_type: path_symbol,
            })
        } else {
            Ok(Self { path_type })
        }
    }
}

pub fn is_primitive_type(ident: &str) -> bool {
    if (ident.starts_with("i") || ident.starts_with("u") || ident.starts_with("f"))
        && ident[1..].chars().all(|c| c.is_numeric())
    {
        return true;
    }

    if ["char", "bool", "usize", "isize", "str"].contains(&ident) {
        return true;
    }

    false
}
