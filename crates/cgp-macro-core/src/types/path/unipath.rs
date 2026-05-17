use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Type;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{At, Dot};

use crate::exports::{Nil, PathCons};
use crate::types::path::PathElement;

#[derive(Debug, Clone, Default)]
pub struct UniPath {
    pub elements: Punctuated<PathElement, Dot>,
}

impl UniPath {
    pub fn from_iter(elements: impl IntoIterator<Item = PathElement>) -> Self {
        Self {
            elements: Punctuated::from_iter(elements),
        }
    }

    pub fn append_type(&mut self, ty: Type) {
        self.elements.push(PathElement::Type(ty));
    }
}

impl Parse for UniPath {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: At = input.parse()?;

        let elements = Punctuated::parse_separated_nonempty(input)?;

        Ok(Self { elements })
    }
}

impl ToTokens for UniPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let out = self.elements.iter().rev().fold(
            quote!(#Nil),
            |acc, current| quote!( #PathCons < #current, #acc > ),
        );

        tokens.extend(out)
    }
}
