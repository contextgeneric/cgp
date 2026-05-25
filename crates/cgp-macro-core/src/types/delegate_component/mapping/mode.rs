use syn::parse::{Parse, ParseStream};
use syn::token::{Colon, FatArrow, RArrow};

pub enum DelegateMode {
    Normal(Colon),
    Direct(RArrow),
    Redirect(FatArrow),
}

impl Parse for DelegateMode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(RArrow) {
            Ok(Self::Direct(input.parse()?))
        } else if input.peek(FatArrow) {
            Ok(Self::Redirect(input.parse()?))
        } else {
            Ok(Self::Normal(input.parse()?))
        }
    }
}
