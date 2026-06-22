use syn::bracketed;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Comma};

use crate::types::delegate_and_check_components::SingleDelegateAndCheckKey;

pub enum DelegateAndCheckKey {
    Single(SingleDelegateAndCheckKey),
    Multi(Punctuated<SingleDelegateAndCheckKey, Comma>),
}

impl Parse for DelegateAndCheckKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Bracket) {
            let body;
            bracketed!(body in input);

            let keys = Punctuated::parse_terminated(&body)?;
            Ok(Self::Multi(keys))
        } else {
            let key = input.parse()?;
            Ok(Self::Single(key))
        }
    }
}
