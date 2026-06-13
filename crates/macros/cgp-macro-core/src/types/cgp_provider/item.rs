use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Error, Ident, ItemImpl, Type, parse_quote, parse2};

use crate::types::cgp_provider::{LoweredCgpProvider, ProviderArgs};
use crate::types::empty_struct::EmptyStruct;
use crate::types::ident::{IdentWithTypeArgs, IdentWithTypeGenerics};
use crate::types::provider_impl::ItemProviderImpl;

pub struct ItemCgpProvider {
    pub args: ProviderArgs,
    pub item_impl: ItemImpl,
}

impl ItemCgpProvider {
    pub fn lower(&self) -> syn::Result<LoweredCgpProvider> {
        let provider_struct = self.to_provider_struct()?;

        let is_provider_for_impl = ItemProviderImpl {
            component_type: self.component_type()?,
            item_impl: self.item_impl.clone(),
        }
        .to_is_provider_for_impl()?;

        Ok(LoweredCgpProvider {
            item_impl: self.item_impl.clone(),
            is_provider_for_impl,
            provider_struct,
        })
    }

    pub fn component_type(&self) -> syn::Result<Type> {
        let item_impl = &self.item_impl;

        let (_, provider_trait_path, _) = item_impl.trait_.as_ref().ok_or_else(|| {
            Error::new(item_impl.span(), "expect provider trait name to be present")
        })?;

        let provider_trait: IdentWithTypeArgs = parse2(provider_trait_path.to_token_stream())?;

        let component_ident = Ident::new(
            &format!("{}Component", provider_trait.ident),
            provider_trait.span(),
        );

        parse2(component_ident.to_token_stream())
    }

    pub fn to_provider_struct(&self) -> syn::Result<Option<EmptyStruct>> {
        if self.args.new.is_none() {
            return Ok(None);
        }

        let provider_impl = &self.item_impl;

        let impl_self_type = &provider_impl.self_ty;

        let provider_type: IdentWithTypeGenerics = parse_quote!( #impl_self_type );

        let provider_struct = EmptyStruct {
            ident: provider_type.ident.clone(),
            generics: provider_type.type_generics.generics.clone(),
        };

        Ok(Some(provider_struct))
    }
}
