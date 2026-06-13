use syn::{ItemImpl, ItemTrait};

use crate::types::attributes::CgpComponentAttributes;
use crate::types::cgp_component::CgpComponentArgs;
use crate::types::empty_struct::EmptyStruct;
use crate::types::provider_impl::{ItemProviderImpl, ItemProviderImpls};

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

    pub fn to_use_delegate_impls(&self) -> syn::Result<ItemProviderImpls> {
        let provider_trait = &self.provider_trait;
        let component_type = self.args.component_name.to_type();
        let mut provider_impls = ItemProviderImpls::default();

        for delegate_attribute in &self.args.derive_delegate_attributes.attributes {
            let item_impl = delegate_attribute.to_provider_impl(provider_trait)?;
            provider_impls.items.push(ItemProviderImpl {
                component_type: component_type.clone(),
                item_impl,
            })
        }

        Ok(provider_impls)
    }
}
