use syn::bracketed;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Comma};

use crate::types::check_components::TypeWithGenerics;

pub enum CheckValue {
    Single(Box<TypeWithGenerics>),
    Multi(Punctuated<TypeWithGenerics, Comma>),
}

impl CheckValue {
    pub fn to_values(&self) -> Vec<TypeWithGenerics> {
        match self {
            Self::Single(value) => vec![value.as_ref().clone()],
            Self::Multi(values) => Vec::from_iter(values.iter().cloned()),
        }
    }
}

impl Parse for CheckValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Bracket) {
            let content;
            bracketed!(content in input);

            let values: Punctuated<TypeWithGenerics, Comma> =
                Punctuated::parse_terminated(&content)?;

            Ok(Self::Multi(values))
        } else {
            let value = input.parse()?;

            Ok(Self::Single(value))
        }
    }
}
