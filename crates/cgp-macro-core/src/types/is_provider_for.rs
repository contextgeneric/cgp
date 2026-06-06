use std::collections::BTreeMap;

use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::token::For;
use syn::{Error, ItemImpl, Path, Type, parse_quote, parse2};

use crate::types::cgp_provider::ProviderImplArgs;
use crate::types::ident::IdentWithTypeArgs;
use crate::visitors::replace_provider_in_generics;

pub fn derive_is_provider_for(
    component_type: &Type,
    item_impl: &ItemImpl,
) -> syn::Result<ItemImpl> {
    ItemIsProviderFor {
        component_type: component_type.clone(),
        item_impl: item_impl.clone(),
    }
    .lower()
}

pub struct ItemIsProviderFor {
    pub component_type: Type,
    pub item_impl: ItemImpl,
}

impl ItemIsProviderFor {
    pub fn lower(&self) -> syn::Result<ItemImpl> {
        let component_type = &self.component_type;
        let item_impl = &self.item_impl;

        let (_, provider_path, _) = item_impl.trait_.as_ref().ok_or_else(|| {
            Error::new(item_impl.span(), "provider impl should contain trait path")
        })?;

        let IdentWithTypeArgs {
            ident: provider_ident,
            type_args: provider_generics,
        } = parse2(provider_path.to_token_stream())?;

        let impl_args = ProviderImplArgs::from_generic_args(&provider_generics)?;
        let context_type = &impl_args.context_type;

        let is_provider_path: Path =
            parse_quote!( IsProviderFor < #component_type, #context_type, ( #impl_args ) > );

        let mut is_provider_impl = item_impl.clone();

        is_provider_impl.attrs.clear();
        is_provider_impl.items.clear();
        is_provider_impl.defaultness = None;
        is_provider_impl.unsafety = None;

        is_provider_impl.trait_ = Some((None, is_provider_path, For(Span::call_site())));

        let provider_map = BTreeMap::from([(provider_ident.clone(), component_type.clone())]);
        replace_provider_in_generics(&provider_map, &mut is_provider_impl.generics);

        Ok(is_provider_impl)
    }
}
