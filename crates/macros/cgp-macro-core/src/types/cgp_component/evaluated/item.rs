use syn::{Item, ItemImpl, ItemTrait};

use crate::types::attributes::CgpComponentAttributes;
use crate::types::cgp_component::CgpComponentArgs;
use crate::types::empty_struct::EmptyStruct;
use crate::types::provider_impl::{ItemProviderImpl, ItemProviderImpls};

/// Final pipeline stage: all derived items, plus the args and attributes needed
/// to render the standard provider impls (`UseContext`, `RedirectLookup`, and the
/// per-attribute `UseDelegate`/prefix impls).
pub struct EvaluatedCgpComponent {
    pub component_struct: EmptyStruct,
    pub consumer_trait: ItemTrait,
    pub consumer_impl: ItemImpl,
    pub provider_trait: ItemTrait,
    pub provider_impl: ItemImpl,
    pub args: CgpComponentArgs,
    pub attributes: CgpComponentAttributes,
}

impl EvaluatedCgpComponent {
    /// Emit the five core items in fixed order (consumer trait, consumer impl,
    /// provider trait, provider impl, marker struct), then the provider impls.
    pub fn to_items(&self) -> syn::Result<Vec<Item>> {
        let mut items = vec![
            Item::Trait(self.consumer_trait.clone()),
            Item::Impl(self.consumer_impl.clone()),
            Item::Trait(self.provider_trait.clone()),
            Item::Impl(self.provider_impl.clone()),
            Item::Struct(self.component_struct.to_item_struct()),
        ];

        let item_impls = self.to_item_impls()?.into_iter().map(Item::Impl);

        items.extend(item_impls);

        Ok(items)
    }

    pub fn to_item_impls(&self) -> syn::Result<Vec<ItemImpl>> {
        let mut item_impls = self.to_provider_impls()?.to_item_impls()?;

        item_impls.extend(self.to_prefix_impls()?);

        Ok(item_impls)
    }

    pub fn to_provider_impls(&self) -> syn::Result<ItemProviderImpls> {
        let mut provider_impls = ItemProviderImpls::default();

        let use_context_impl = self.to_use_context_impl()?;
        provider_impls.items.push(use_context_impl);

        let redirect_lookup_impl = self.to_redirect_lookup_impl()?;
        provider_impls.items.push(redirect_lookup_impl);

        let use_delegate_impls = self.to_use_delegate_impls()?;
        provider_impls.items.extend(use_delegate_impls.items);

        Ok(provider_impls)
    }

    /// One `UseDelegate` provider impl per `#[derive_delegate]` attribute.
    pub fn to_use_delegate_impls(&self) -> syn::Result<ItemProviderImpls> {
        let provider_trait = &self.provider_trait;
        let component_type = self.args.component_name.to_type();
        let mut provider_impls = ItemProviderImpls::default();

        for delegate_attribute in &self.attributes.derive_delegate_attributes.attributes {
            let item_impl = delegate_attribute.to_provider_impl(provider_trait)?;
            provider_impls.items.push(ItemProviderImpl {
                component_type: component_type.clone(),
                item_impl,
            })
        }

        Ok(provider_impls)
    }

    /// One namespace prefix impl per `#[prefix]` attribute (from `#[cgp_namespace]`).
    pub fn to_prefix_impls(&self) -> syn::Result<Vec<ItemImpl>> {
        let component_name = &self.args.component_name;
        let mut provider_impls = Vec::new();

        for attribute in &self.attributes.prefixes {
            let provider_impl = attribute.to_namespace_impl(component_name)?;
            provider_impls.push(provider_impl)
        }

        Ok(provider_impls)
    }
}
