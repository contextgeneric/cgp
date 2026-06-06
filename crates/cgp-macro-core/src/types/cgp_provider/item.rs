use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Error, Ident, ItemImpl, ItemStruct, Type, parse_quote, parse2};

use crate::types::cgp_provider::{LoweredCgpProvider, ProviderArgs};
use crate::types::ident::{IdentWithTypeArgs, IdentWithTypeGenerics};
use crate::types::is_provider_for::ItemIsProviderFor;

pub struct ItemCgpProvider {
    pub args: ProviderArgs,
    pub item_impl: ItemImpl,
}

impl ItemCgpProvider {
    pub fn lower(&self) -> syn::Result<LoweredCgpProvider> {
        let provider_struct = self.to_provider_struct()?;

        let is_provider_for_impl = ItemIsProviderFor {
            component_type: self.component_type()?,
            item_impl: self.item_impl.clone(),
        }
        .lower()?;

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

    pub fn to_provider_struct(&self) -> syn::Result<Option<ItemStruct>> {
        if self.args.new.is_none() {
            return Ok(None);
        }

        let provider_impl = &self.item_impl;

        let impl_self_type = &provider_impl.self_ty;

        let provider_type: IdentWithTypeGenerics = parse_quote!( #impl_self_type );

        let provider_name = &provider_type.ident;
        let type_generics_params = &provider_type.type_generics.params;

        let provider_struct = if type_generics_params.is_empty() {
            parse_quote! {
                pub struct #provider_name;
            }
        } else {
            parse_quote! {
                pub struct #provider_name<#type_generics_params>(
                    pub ::core::marker::PhantomData<(#type_generics_params)>
                );
            }
        };

        Ok(Some(provider_struct))
    }
}
