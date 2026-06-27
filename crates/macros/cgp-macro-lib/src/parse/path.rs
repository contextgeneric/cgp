use cgp_macro_core::types::generics::ImplGenerics;
use cgp_macro_core::types::path::PathHead;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Type, parse_quote};

pub struct ComponentPaths {
    pub paths: Vec<ComponentPath<Type>>,
}

impl Parse for ComponentPaths {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path_head = PathHead::parse(input)?;

        if let PathHead::End = path_head {
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

#[derive(Clone)]
pub struct ComponentPath<Path> {
    pub path_type: Path,
    pub generics: ImplGenerics,
}

pub fn path_head_to_prefix(path_head: &PathHead) -> Vec<ComponentPath<TokenStream>> {
    match path_head {
        PathHead::Type(path_element, rest) => {
            let rest_types = path_head_to_prefix(rest);

            prepend_path(
                path_element.element.to_token_stream(),
                path_element.generics.clone(),
                rest_types,
            )
        }
        PathHead::Group(path_elements, rest) => {
            let rest_types = path_head_to_prefix(rest);
            let mut out = Vec::new();

            for path_element in path_elements {
                let paths = prepend_path(
                    path_element.element.to_token_stream(),
                    path_element.generics.clone(),
                    rest_types.clone(),
                );
                out.extend(paths);
            }

            out
        }
        PathHead::Nested(paths) => paths.iter().flat_map(path_head_to_prefix).collect(),
        PathHead::End => {
            vec![ComponentPath {
                path_type: quote! { __Wildcard__ },
                generics: parse_quote! { <__Wildcard__> },
            }]
        }
    }
}

pub fn prepend_path(
    path_type: TokenStream,
    generics: ImplGenerics,
    rest_types: Vec<ComponentPath<TokenStream>>,
) -> Vec<ComponentPath<TokenStream>> {
    rest_types
        .into_iter()
        .map(|mut path| {
            let rest_tokens = path.path_type;

            path.generics
                .generics
                .params
                .extend(generics.generics.params.clone());

            let new_path = quote! { PathCons< #path_type , #rest_tokens > };
            ComponentPath {
                path_type: new_path,
                generics: path.generics,
            }
        })
        .collect()
}
