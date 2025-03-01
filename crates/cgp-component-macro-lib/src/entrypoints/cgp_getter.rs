use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, ItemTrait, Type};

use crate::derive_component::derive_component_with_ast;
use crate::derive_provider::derive_is_provider_for;
use crate::getter_component::{
    derive_use_field_impl, derive_use_fields_impl, derive_with_provider_impl, parse_getter_fields,
    GetterField,
};
use crate::parse::ComponentSpec;

pub fn cgp_getter(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let spec: ComponentSpec = syn::parse2(attr)?;
    let consumer_trait: ItemTrait = syn::parse2(body)?;

    let derived_component = derive_component_with_ast(&spec, consumer_trait.clone())?;

    let fields = parse_getter_fields(&spec.context_type, &consumer_trait)?;

    let use_fields_impl =
        derive_use_fields_impl(&spec, &derived_component.provider_trait, &fields)?;

    let component_name_type: Type = {
        let component_name = &spec.component_name;
        let component_params = &spec.component_params;
        parse_quote!( #component_name < #component_params > )
    };

    let is_provider_use_fields_impl =
        derive_is_provider_for(&component_name_type, &use_fields_impl)?;

    let m_field: Option<[GetterField; 1]> = fields.try_into().ok();

    let mut derived = quote! {
        #derived_component

        #use_fields_impl

        #is_provider_use_fields_impl
    };

    if let Some([field]) = m_field {
        let use_field_impl =
            derive_use_field_impl(&spec, &derived_component.provider_trait, &field)?;
        let is_provider_use_field_impl =
            derive_is_provider_for(&component_name_type, &use_field_impl)?;

        let use_provider_impl =
            derive_with_provider_impl(&spec, &derived_component.provider_trait, &field)?;
        let is_provider_use_provider_impl =
            derive_is_provider_for(&component_name_type, &use_provider_impl)?;

        derived.extend(quote! {
            #use_field_impl
            #is_provider_use_field_impl

            #use_provider_impl
            #is_provider_use_provider_impl
        });
    }

    Ok(derived)
}
