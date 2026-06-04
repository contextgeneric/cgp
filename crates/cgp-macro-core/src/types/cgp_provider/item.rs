use std::collections::BTreeMap;

use proc_macro2::Span;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{Comma, For};
use syn::{Error, Ident, ItemImpl, ItemStruct, Path, Type, parse_quote, parse2};

use crate::types::cgp_provider::ProviderArgs;
use crate::types::ident::{IdentWithTypeArgs, IdentWithTypeGenerics};
use crate::visitors::replace_provider_in_generics;

pub struct ItemCgpProvider {
    pub args: ProviderArgs,
    pub item_impl: ItemImpl,
}

impl ItemCgpProvider {
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

    pub fn to_is_provider_for_impl(&self) -> syn::Result<ItemImpl> {
        let component_name = self.component_type()?;

        let provider_impl = &self.item_impl;

        let (_, provider_path, _) = provider_impl.trait_.as_ref().ok_or_else(|| {
            Error::new(
                provider_impl.span(),
                "provider impl should contain trait path",
            )
        })?;

        let provider_ident: IdentWithTypeArgs = parse2(provider_path.to_token_stream())?;

        let provider_map = BTreeMap::from([(provider_ident.ident.clone(), component_name.clone())]);

        let is_provider_params = provider_ident.type_args.to_param_types()?;
        let is_provider_params = Punctuated::<_, Comma>::from_iter(is_provider_params);

        let context_arg = provider_ident.type_args.type_args().first().ok_or_else(|| {
            Error::new(
                provider_impl.span(),
                "provider impl should contain trait path containing at least one generic parameter",
            )
        })?.clone();

        let is_provider_path: Path = parse_quote!( IsProviderFor < #component_name, #context_arg, ( #is_provider_params ) > );

        let mut is_provider_impl = provider_impl.clone();

        is_provider_impl.attrs.clear();
        is_provider_impl.items.clear();
        is_provider_impl.defaultness = None;
        is_provider_impl.unsafety = None;

        is_provider_impl.trait_ = Some((None, is_provider_path, For(Span::call_site())));

        replace_provider_in_generics(&provider_map, &mut is_provider_impl.generics);

        Ok(is_provider_impl)
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
