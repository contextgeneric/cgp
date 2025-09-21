use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::{ItemImpl, ItemStruct, ItemTrait, parse2};

use crate::derive_component::component_name::derive_component_name_struct;
use crate::derive_component::consumer_impl::derive_consumer_impl;
use crate::derive_component::provider_impl::derive_provider_impl;
use crate::derive_component::provider_trait::derive_provider_trait;
use crate::derive_component::use_context_impl::derive_use_context_impl;
use crate::derive_component::use_delegate_impl::derive_delegate_impl;
use crate::derive_provider::derive_is_provider_for;
use crate::parse::ComponentSpec;

pub fn derive_component_with_ast(
    spec: &ComponentSpec,
    consumer_trait: ItemTrait,
) -> syn::Result<DerivedComponent> {
    let provider_name = &spec.provider_name;
    let context_type = &spec.context_type;

    let component_name = &spec.component_name;
    let component_params = &spec.component_params;

    let component_struct = derive_component_name_struct(component_name, component_params)?;

    let provider_trait = derive_provider_trait(
        component_name,
        component_params,
        &consumer_trait,
        provider_name,
        context_type,
    )?;

    let consumer_impl = derive_consumer_impl(&consumer_trait, provider_name, context_type)?;

    let provider_impl = derive_provider_impl(
        context_type,
        &consumer_trait,
        &provider_trait,
        component_name,
        component_params,
    )?;

    let use_context_impl = derive_use_context_impl(context_type, &consumer_trait, &provider_trait)?;

    let use_context_is_provider_impl = derive_is_provider_for(
        &parse2(quote! {
            #component_name < #component_params >
        })?,
        &use_context_impl,
    )?;

    let mut item_impls = vec![
        consumer_impl,
        provider_impl,
        use_context_impl,
        use_context_is_provider_impl,
    ];

    if !spec.use_delegate_spec.is_empty() {
        for spec in spec.use_delegate_spec.iter() {
            let use_delegate_impl = derive_delegate_impl(&provider_trait, spec)?;

            let use_delegate_is_provider_impl = derive_is_provider_for(
                &parse2(quote! {
                    #component_name < #component_params >
                })?,
                &use_delegate_impl,
            )?;

            item_impls.push(use_delegate_impl);
            item_impls.push(use_delegate_is_provider_impl);
        }
    }

    let derived = DerivedComponent {
        component_struct,
        consumer_trait,
        provider_trait,
        item_impls,
    };

    Ok(derived)
}

pub struct DerivedComponent {
    pub component_struct: ItemStruct,
    pub consumer_trait: ItemTrait,
    pub provider_trait: ItemTrait,
    pub item_impls: Vec<ItemImpl>,
}

impl ToTokens for DerivedComponent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.component_struct.to_tokens(tokens);
        self.consumer_trait.to_tokens(tokens);
        self.provider_trait.to_tokens(tokens);
        tokens.append_all(self.item_impls.iter());
    }
}
