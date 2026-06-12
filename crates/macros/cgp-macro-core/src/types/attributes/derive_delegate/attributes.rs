use syn::bracketed;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Comma};

use crate::types::attributes::DeriveDelegateAttribute;

#[derive(Default, Clone)]
pub struct DeriveDelegateAttributes {
    pub attributes: Vec<DeriveDelegateAttribute>,
}

impl Parse for DeriveDelegateAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Bracket) {
            let body;
            bracketed!(body in input);

            let attributes = <Punctuated<DeriveDelegateAttribute, Comma>>::parse_terminated(&body)?;
            Ok(Self {
                attributes: Vec::from_iter(attributes),
            })
        } else {
            let spec = input.parse()?;
            Ok(Self {
                attributes: vec![spec],
            })
        }
    }
}
