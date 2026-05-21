use syn::Type;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Semi};

use crate::define_keyword;

define_keyword!(Open, OpenKeyword, "open");

pub struct OpenDelegateEntry {
    pub open: Open,
    pub components: Punctuated<Type, Comma>,
    pub semi: Semi,
}

impl Parse for OpenDelegateEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let open = input.parse()?;

        let components: Punctuated<Type, Comma> = Punctuated::parse_separated_nonempty(input)?;
        let semi = input.parse()?;

        Ok(Self {
            open,
            components,
            semi,
        })
    }
}
