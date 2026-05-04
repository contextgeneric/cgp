use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{At, Colon, Dot};
use syn::{Ident, Type, parse2};

use crate::parse::PathType;

pub struct UseNamespaceAttribute {
    pub namespace: Ident,
    pub path: Type,
}

impl Parse for UseNamespaceAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let namespace = if input.peek2(Colon) {
            let namespace = input.parse()?;
            let _: Colon = input.parse()?;
            namespace
        } else {
            Ident::new("DefaultNamespace", input.span())
        };

        let _: At = input.parse()?;

        let paths: Punctuated<PathType, Dot> = Punctuated::parse_separated_nonempty(input)?;

        let raw_path_type = paths.into_iter().rev().fold(
            quote!(PathNil),
            |tail, PathType { path_type }| quote!(PathCons<#path_type, #tail>),
        );

        let path: Type = parse2(raw_path_type)?;

        Ok(UseNamespaceAttribute { namespace, path })
    }
}
