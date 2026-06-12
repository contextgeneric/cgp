use syn::Type;
use syn::parse::{Parse, ParseStream};
use syn::token::In;

use crate::types::path::UniPath;

pub struct PrefixAttribute {
    pub path: UniPath,
    pub _in_token: In,
    pub namespace: Type,
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
