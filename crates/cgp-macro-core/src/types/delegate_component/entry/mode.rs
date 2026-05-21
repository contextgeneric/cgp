use syn::parse::{Parse, ParseStream};
use syn::token::{Colon, RArrow};

pub enum DelegateMode {
    Normal(Colon),
    Direct(RArrow),
}

impl Parse for DelegateMode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(RArrow) {
            Ok(Self::Direct(input.parse()?))
        } else {
            Ok(Self::Normal(input.parse()?))
        }
    }
}
