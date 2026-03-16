use syn::Ident;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Colon, Dot};

use crate::parse::PathType;

pub struct UseNamespaceAttribute {
    pub namespace: Ident,
    pub path: Punctuated<PathType, Dot>,
}

impl Parse for UseNamespaceAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let namespace = if input.peek2(Colon) {
            let namespace = input.parse()?;
            let _: Colon = input.parse()?;
            namespace
        } else {
            Ident::new("DefaultNamespace", input.span())
        };

        let path = Punctuated::parse_separated_nonempty(input)?;
        Ok(UseNamespaceAttribute { namespace, path })
    }
}
