use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Dot;

use crate::exports::{PathCons, PathNil};
use crate::types::path::PathElement;

pub struct UniPath {
    pub elements: Punctuated<PathElement, Dot>,
}

impl Parse for UniPath {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let elements = Punctuated::parse_terminated(input)?;

        Ok(Self { elements })
    }
}

impl ToTokens for UniPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let out = self.elements.iter().rev().fold(
            quote!(#PathNil),
            |acc, current| quote!( #PathCons < #current, #acc > ),
        );

        tokens.extend(out)
    }
}
