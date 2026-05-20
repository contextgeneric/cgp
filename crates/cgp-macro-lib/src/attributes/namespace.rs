use cgp_macro_core::types::path::UniPath;
use syn::Ident;
use syn::parse::{Parse, ParseStream};
use syn::token::In;

pub struct UseNamespaceAttribute {
    pub namespace: Ident,
    pub path: UniPath,
}

impl Parse for UseNamespaceAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;

        let namespace = if input.peek(In) {
            let _: In = input.parse()?;
            input.parse()?
        } else {
            Ident::new("DefaultNamespace", input.span())
        };

        Ok(UseNamespaceAttribute { namespace, path })
    }
}
