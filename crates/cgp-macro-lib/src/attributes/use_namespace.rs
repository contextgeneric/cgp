use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Dot};
use syn::{Ident, Type};

pub struct UseNamespaceAttribute {
    pub namespace: Ident,
    pub path: Punctuated<Type, Dot>,
}

impl Parse for UseNamespaceAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let namespace = input.parse()?;
        let _: Colon = input.parse()?;

        let path = Punctuated::parse_separated_nonempty(input)?;
        Ok(UseNamespaceAttribute { namespace, path })
    }
}
