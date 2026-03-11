use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Dot};
use syn::{Ident, Type};

pub struct UseNamespace {
    pub namespace: Ident,
    pub path: Punctuated<Type, Dot>,
}

impl Parse for UseNamespace {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let namespace = input.parse()?;
        let _: Colon = input.parse()?;

        let path = Punctuated::parse_separated_nonempty(input)?;
        Ok(UseNamespace { namespace, path })
    }
}
