use syn::parse::{Parse, ParseStream};
use syn::token::In;
use syn::{Generics, ItemImpl, Type};

use crate::parse_internal;
use crate::types::ident::IdentWithTypeArgs;
use crate::types::path::UniPathOrType;

pub struct DefaultImplAttribute {
    pub key_type: UniPathOrType,
    pub in_token: In,
    pub namespace: IdentWithTypeArgs,
}

impl DefaultImplAttribute {
    pub fn to_item_impl(
        &self,
        provider_generics: &Generics,
        provider_type: &Type,
    ) -> syn::Result<ItemImpl> {
        let key_type = &self.key_type;
        let mut namespace_trait_path = self.namespace.clone();

        namespace_trait_path
            .type_args
            .make_args()
            .push(parse_internal!(__Components__));

        let mut generics = provider_generics.clone();
        generics.params.push(parse_internal!(__Components__));

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let item_impl = parse_internal! {
            impl #impl_generics #namespace_trait_path for #key_type
            #where_clause
            {
                type Delegate = #provider_type;
            }
        };

        Ok(item_impl)
    }
}

impl Parse for DefaultImplAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key_type = input.parse()?;
        let in_token = input.parse()?;
        let namespace = input.parse()?;

        Ok(Self {
            key_type,
            in_token,
            namespace,
        })
    }
}
