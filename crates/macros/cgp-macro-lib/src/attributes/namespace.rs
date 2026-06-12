use cgp_macro_core::types::path::UniPath;
use syn::Ident;
use syn::parse::{Parse, ParseStream};
use syn::token::In;

pub struct PrefixAttribute {
    pub path: UniPath,
    pub _in_token: In,
    pub namespace: Ident,
}

impl Parse for PrefixAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;
        let in_token = input.parse()?;
        let namespace = input.parse()?;

        Ok(PrefixAttribute {
            namespace,
            _in_token: in_token,
            path,
        })
    }
}
