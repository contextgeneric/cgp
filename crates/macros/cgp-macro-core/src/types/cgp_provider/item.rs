use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Error, Ident, ItemImpl, Path, Type};

use crate::functions::parse_internal;
use crate::types::cgp_provider::{LoweredCgpProvider, ProviderArgs};
use crate::types::empty_struct::EmptyStruct;
use crate::types::ident::{IdentWithTypeGenerics, PathWithTypeArgs};
use crate::types::provider_impl::ItemProviderImpl;

/// The parsed `#[cgp_provider]` input.
pub struct ItemCgpProvider {
    /// The provider's arguments.
    pub args: ProviderArgs,
    /// The provider impl.
    pub item_impl: ItemImpl,
}

impl ItemCgpProvider {
    /// Lowers the provider into its impl, `IsProviderFor` impl, and struct.
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

    /// The component the provider implements.
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

        // The key keeps the trait's path, so `impl crate::components::Greeter`
        // names `crate::components::GreeterComponent` and needs no import. Its last
        // segment takes the trait identifier's span, so an unresolved key is reported
        // on the trait's name in the header rather than on the whole attribute; with
        // both names resolved, Rust Analyzer's hover there shows the key beside the
        // trait. See cgp-knowledge-base/cgp/reference/macros/cgp_impl.md.
        let provider_ident = provider_trait.ident();

        let mut component_path: Path = provider_trait.path.clone();

        if let Some(last_segment) = component_path.segments.last_mut() {
            last_segment.ident =
                Ident::new(&format!("{provider_ident}Component"), provider_ident.span());
        }

        parse_internal(component_path.to_token_stream())
    }

    /// Builds the provider struct, when the attribute declares one.
    pub fn to_provider_struct(&self) -> syn::Result<Option<EmptyStruct>> {
        if self.args.new.is_none() {
            return Ok(None);
        }

        let provider_impl = &self.item_impl;

        let impl_self_type = &provider_impl.self_ty;

        let provider_type: IdentWithTypeGenerics = parse_internal!( #impl_self_type );

        let provider_ident = &provider_type.ident;
        let doc = match provider_impl.trait_.as_ref().map(|(_, path, _)| path) {
            Some(path) if !path.segments.is_empty() => {
                let segments: Vec<String> =
                    path.segments.iter().map(|s| s.ident.to_string()).collect();
                let provider_trait = &segments[segments.len() - 1];
                if segments.len() == 1 {
                    format!("Provider `{provider_ident}` for [`{provider_trait}`].")
                } else {
                    // Link the full path, so the link resolves where only the path's first
                    // segment is in scope, as with `::cgp::macro_prelude::Computer`.
                    let target = segments.join("::");
                    format!("Provider `{provider_ident}` for [`{provider_trait}`]({target}).")
                }
            }
            _ => format!("Provider `{provider_ident}`."),
        };

        let provider_struct = EmptyStruct {
            ident: provider_type.ident.clone(),
            generics: provider_type.type_generics.to_generics(),
            doc: Some(doc),
        };

        Ok(Some(provider_struct))
    }
}
