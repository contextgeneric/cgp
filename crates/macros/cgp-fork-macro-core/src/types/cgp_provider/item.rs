use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Error, Ident, ItemImpl, Type};

use crate::functions::parse_internal;
use crate::types::cgp_provider::{LoweredCgpProvider, ProviderArgs};
use crate::types::empty_struct::EmptyStruct;
use crate::types::ident::{IdentWithTypeGenerics, PathWithTypeArgs};
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
        // An explicit `#[cgp_provider(Component)]` / `#[cgp_impl(Provider: Component)]`
        // override names the component directly; it is a user-written type, so it
        // keeps its own span. Only fall back to the `{Provider}Component` default
        // when no override is given.
        if let Some(component_type) = &self.args.component_type {
            return Ok(component_type.clone());
        }

        let item_impl = &self.item_impl;

        let (_, provider_trait_path, _) = item_impl.trait_.as_ref().ok_or_else(|| {
            Error::new(item_impl.span(), "expect provider trait name to be present")
        })?;

        let provider_trait: PathWithTypeArgs =
            parse_internal(provider_trait_path.to_token_stream())?;

        // Span the synthesized `{Provider}Component` reference on `call_site`, not
        // the provider trait's span. The derived component appears only as the
        // first type argument of the generated `IsProviderFor<..>` impl — an
        // interior token that anchors no error caret — so giving it the provider
        // trait's span buys nothing at the compiler, and it actively harms the
        // editor: rust-analyzer maps a source token to its expansion by source
        // range, so a component reference sharing the provider trait's range makes
        // go-to-definition on the provider trait offer the component struct too.
        // A `call_site` span shares no narrow user token's range, so it stays out
        // of the way. (This is the reference-side dual of the `#[cgp_component]`
        // marker struct, whose *definition* is instead spanned on the provider
        // identifier so navigation lands there cleanly.)
        let component_ident = Ident::new(
            &format!("{}Component", provider_trait.ident()),
            Span::call_site(),
        );

        parse_internal(component_ident.to_token_stream())
    }

    pub fn to_provider_struct(&self) -> syn::Result<Option<EmptyStruct>> {
        if self.args.new.is_none() {
            return Ok(None);
        }

        let provider_impl = &self.item_impl;

        let impl_self_type = &provider_impl.self_ty;

        let provider_type: IdentWithTypeGenerics = parse_internal!( #impl_self_type );

        let provider_struct = EmptyStruct {
            ident: provider_type.ident.clone(),
            generics: provider_type.type_generics.to_generics(),
        };

        Ok(Some(provider_struct))
    }
}
