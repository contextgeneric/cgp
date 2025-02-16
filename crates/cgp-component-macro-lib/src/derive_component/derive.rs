use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemImpl, ItemStruct, ItemTrait};

use crate::derive_component::component_name::derive_component_name_struct;
use crate::derive_component::component_spec::ComponentSpec;
use crate::derive_component::consumer_impl::derive_consumer_impl;
use crate::derive_component::provider_impl::derive_provider_impl;
use crate::derive_component::provider_trait::derive_provider_trait;

pub fn derive_component(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let spec: ComponentSpec = syn::parse2(attr)?;
    let consumer_trait: ItemTrait = syn::parse2(item)?;

    let derived = derive_component_with_ast(&spec, consumer_trait)?;

    Ok(derived.to_token_stream())
}

pub fn derive_component_with_ast(
    spec: &ComponentSpec,
    consumer_trait: ItemTrait,
) -> syn::Result<DerivedComponent> {
    let provider_name = &spec.provider_name;
    let context_type = &spec.context_type;

    let component_struct =
        derive_component_name_struct(&spec.component_name, &spec.component_params);

    let provider_trait = derive_provider_trait(
        &spec.component_name,
        &spec.component_params,
        &consumer_trait,
        provider_name,
        context_type,
    )?;

    let consumer_impl = derive_consumer_impl(&consumer_trait, provider_name, context_type);

    let provider_impl = derive_provider_impl(
        context_type,
        &consumer_trait,
        &provider_trait,
        &spec.component_name,
        &spec.component_params,
    );

    let derived = DerivedComponent {
        component_struct,
        consumer_trait,
        provider_trait,
        consumer_impl,
        provider_impl,
    };

    Ok(derived)
}

pub struct DerivedComponent {
    pub component_struct: ItemStruct,
    pub consumer_trait: ItemTrait,
    pub provider_trait: ItemTrait,
    pub consumer_impl: ItemImpl,
    pub provider_impl: ItemImpl,
}

impl ToTokens for DerivedComponent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.component_struct.to_tokens(tokens);
        self.consumer_trait.to_tokens(tokens);
        self.provider_trait.to_tokens(tokens);
        self.consumer_impl.to_tokens(tokens);
        self.provider_impl.to_tokens(tokens);
    }
}
