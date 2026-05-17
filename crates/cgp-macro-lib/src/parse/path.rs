use cgp_macro_core::types::{ImplGenerics, PathElement};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Comma, Dot, Lt};
use syn::{Type, braced, parse_quote};

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

        for path in path_head_to_prefix(&path_head) {
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
    Type(Option<ImplGenerics>, Box<PathElement>, Box<PathHead>),
    Group(Punctuated<PathHead, Comma>),
    Wildcard,
}

pub fn path_head_to_prefix(path_head: &PathHead) -> Vec<ComponentPath<TokenStream>> {
    match path_head {
        PathHead::Type(generics, path_type, rest) => {
            let rest_types = path_head_to_prefix(rest);

            prepend_path(path_type.to_token_stream(), generics.clone(), rest_types)
        }
        PathHead::Group(paths) => paths
            .iter()
            .flat_map(|path| path_head_to_prefix(path))
            .collect(),
        PathHead::Wildcard => {
            vec![ComponentPath {
                path_type: quote! { __Wildcard__ },
                generics: parse_quote! { <__Wildcard__> },
            }]
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

            let path_type: PathElement = input.parse()?;

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
