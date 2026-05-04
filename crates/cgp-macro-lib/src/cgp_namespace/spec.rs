use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{At, Colon, Comma, Dot, Lt};
use syn::{Ident, Type, braced, parse2};

use crate::parse::{ComponentPath, ComponentPaths, ImplGenerics, PathType};

pub struct NamespaceSpec {
    pub namespace_ident: Ident,
    pub parent_namespace_ident: Option<Ident>,
    pub entries: Punctuated<NamespaceEntry, Comma>,
}

pub struct NamespaceEntry {
    pub keys: ComponentPaths,
    pub value: Type,
}

impl Parse for NamespaceSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let namespace_ident: Ident = input.parse()?;

        let parent_namespace_ident: Option<Ident> = if input.peek(Colon) {
            let _: Colon = input.parse()?;
            let ident = input.parse()?;
            Some(ident)
        } else {
            None
        };

        let content;
        braced!(content in input);

        let entries = Punctuated::parse_terminated(&content)?;

        Ok(NamespaceSpec {
            namespace_ident,
            parent_namespace_ident,
            entries,
        })
    }
}

impl Parse for NamespaceEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let keys: ComponentPaths = if input.peek(At) {
            let _: At = input.parse()?;

            input.parse()?
        } else {
            let generics: ImplGenerics = if input.peek(Lt) {
                input.parse()?
            } else {
                Default::default()
            };

            let path_type: Type = input.parse()?;

            let path = ComponentPath {
                generics,
                path_type,
            };

            ComponentPaths { paths: vec![path] }
        };

        let _: Colon = input.parse()?;

        let _: At = input.parse()?;

        let value_path: Punctuated<PathType, Dot> = Punctuated::parse_separated_nonempty(input)?;

        let value = value_path.into_iter().rev().fold(
            quote!(PathNil),
            |tail, PathType { path_type }| quote!( PathCons< #path_type, #tail > ),
        );

        let value: Type = parse2(value)?;

        Ok(Self { keys, value })
    }
}
