use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Type;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{At, Dot};

use crate::exports::{Nil, PathCons};
use crate::types::path::{PathElement, PrefixPath};

#[derive(Debug, Clone, Default)]
pub struct UniPath {
    pub elements: Punctuated<PathElement, Dot>,
}

impl FromIterator<PathElement> for UniPath {
    fn from_iter<T: IntoIterator<Item = PathElement>>(elements: T) -> Self {
        Self {
            elements: Punctuated::from_iter(elements),
        }
    }
}

impl UniPath {
    pub fn append_type(&mut self, ty: Type) {
        self.elements.push(PathElement::Type(ty));
    }

    pub fn to_prefix(self, suffix: Type) -> PrefixPath {
        PrefixPath {
            elements: self.elements,
            suffix,
        }
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
