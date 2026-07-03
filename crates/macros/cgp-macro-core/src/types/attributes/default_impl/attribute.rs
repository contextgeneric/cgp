use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::In;
use syn::{Generics, ItemImpl, Type};

use crate::functions::override_span;
use crate::parse_internal;
use crate::types::ident::PathWithTypeArgs;
use crate::types::path::UniPathOrType;

pub struct DefaultImplAttribute {
    pub key_type: UniPathOrType,
    pub in_token: In,
    pub namespace: PathWithTypeArgs,
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
            .args
            .push(parse_internal!(__Components__));

        let mut generics = provider_generics.clone();
        generics.params.push(parse_internal!(__Components__));

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let item_impl: ItemImpl = parse_internal! {
            impl #impl_generics #namespace_trait_path for #key_type
            #where_clause
            {
                type Delegate = #provider_type;
            }
        };

        // The impl is built wholly from quasi-quoted tokens, so its `impl`/`for`
        // structural tokens carry the macro `call_site` span. Re-span it onto the
        // user-written key token so a coherence conflict (`E0119`) between two
        // default impls for the same key is reported on that key rather than on
        // the whole `#[cgp_impl]` attribute. Generics keep their own spans,
        // restored afterward, mirroring `EvaluatedDelegateEntry::respan_impl`.
        let generics = item_impl.generics.clone();
        let mut item_impl = override_span(key_type.span(), &item_impl)?;
        item_impl.generics = generics;

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
