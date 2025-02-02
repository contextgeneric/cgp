use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemTrait;

use crate::derive_component::component_name::derive_component_name_struct;
use crate::derive_component::component_spec::ComponentSpec;
use crate::derive_component::consumer_impl::derive_consumer_impl;
use crate::derive_component::provider_impl::derive_provider_impl;
use crate::derive_component::provider_trait::derive_provider_trait;

pub fn derive_component(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let spec: ComponentSpec = syn::parse2(attr)?;
    let consumer_trait: ItemTrait = syn::parse2(item)?;

    derive_component_with_ast(&spec, &consumer_trait)
}

pub fn derive_component_with_ast(
    spec: &ComponentSpec,
    consumer_trait: &ItemTrait,
) -> syn::Result<TokenStream> {
    let provider_name = &spec.provider_name;
    let context_type = &spec.context_type;

    let component_struct =
        derive_component_name_struct(&spec.component_name, &spec.component_params);

    let provider_trait = derive_provider_trait(
        &spec.component_name,
        &spec.component_params,
        consumer_trait,
        provider_name,
        context_type,
    )?;

    let consumer_impl = derive_consumer_impl(consumer_trait, provider_name, context_type);

    let provider_impl = derive_provider_impl(
        context_type,
        consumer_trait,
        &provider_trait,
        &spec.component_name,
        &spec.component_params,
    );

    let derived = quote! {
        #consumer_trait
        #component_struct
        #provider_trait
        #consumer_impl
        #provider_impl
    };

    Ok(derived)
}
