use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::token::In;
use syn::{ItemImpl, parse_quote, parse2};

use crate::exports::RedirectLookup;
use crate::types::ident::{IdentWithTypeArgs, IdentWithTypeGenerics};
use crate::types::path::UniPath;

#[derive(Clone)]
pub struct PrefixAttribute {
    pub path: UniPath,
    pub _in_token: In,
    pub namespace: IdentWithTypeArgs,
}

impl PrefixAttribute {
    pub fn to_namespace_impl(
        &self,
        component_name: &IdentWithTypeGenerics,
    ) -> syn::Result<ItemImpl> {
        let mut namespace = self.namespace.clone();
        namespace
            .type_args
            .make_args()
            .push(parse_quote!(__Components__));

        let mut path = self.path.clone();
        path.append_type(parse_quote!(#component_name));

        let mut type_generics = component_name.type_generics.clone();
        type_generics.params.insert(0, parse_quote!(__Components__));

        let item_impl = parse2(quote! {
            impl #type_generics #namespace for #component_name
            {
                type Delegate = #RedirectLookup< __Components__, #path >;
            }
        })?;

        Ok(item_impl)
    }
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
