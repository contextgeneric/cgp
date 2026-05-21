use syn::bracketed;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;

use crate::types::delegate_component::SingleDelegateKey;

pub struct MultiDelegateKey {
    pub keys: Punctuated<SingleDelegateKey, Comma>,
}

impl Parse for MultiDelegateKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body;
        bracketed!(body in input);
        let keys = Punctuated::parse_terminated(&body)?;

        Ok(Self { keys })
    }
}
