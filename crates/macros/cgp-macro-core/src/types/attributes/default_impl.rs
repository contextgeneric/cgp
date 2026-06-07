use syn::spanned::Spanned;
use syn::token::Type;
use syn::{Error, Generics, ItemImpl, parse_quote};

use crate::types::ident::IdentWithTypeArgs;

pub struct DefaultImplAttribute {
    pub namespace: IdentWithTypeArgs,
}

impl DefaultImplAttribute {
    pub fn to_item_impl(
        &self,
        provider_generics: &Generics,
        provider_trait_path: &IdentWithTypeArgs,
        provider_type: &Type,
    ) -> syn::Result<ItemImpl> {
        let mut namespace_trait_path = self.namespace.clone();

        namespace_trait_path
            .type_args
            .make_args()
            .push(parse_quote!(__Components__));

        let mut generics = provider_generics.clone();
        generics.params.push(parse_quote!(__Components__));

        let type_args = provider_trait_path
            .type_args
            .args
            .as_ref()
            .ok_or_else(|| Error::new(
                provider_trait_path.span(),
                "#[default_impl] can only be used with CGP traits with at least one generic argument"
            ))?
            .args
            .clone();

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let item_impl = parse_quote! {
            impl #impl_generics #namespace_trait_path for (#type_args)
            #where_clause
            {
                type Delegate = #provider_type;
            }
        };

        Ok(item_impl)
    }
}
