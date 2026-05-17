use cgp_macro_core::types::path::UniPath;
use syn::Ident;
use syn::parse::{Parse, ParseStream};
use syn::token::Colon;

pub struct UseNamespaceAttribute {
    pub namespace: Ident,
    pub path: UniPath,
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

        let path = input.parse()?;

        Ok(UseNamespaceAttribute { namespace, path })
    }
}
