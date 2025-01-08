use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemTrait;

use crate::derive_component::component_spec::ComponentSpec;
use crate::derive_component::derive::derive_component_with_ast;
use crate::getter_component::getter_field::GetterField;
use crate::getter_component::parse::parse_getter_fields;
use crate::getter_component::use_field::derive_use_field_impl;
use crate::getter_component::use_fields::derive_use_fields_impl;
use crate::getter_component::with_provider::derive_with_provider_impl;

pub fn derive_getter_component(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let spec: ComponentSpec = syn::parse2(attr)?;
    let consumer_trait: ItemTrait = syn::parse2(item)?;

    let derived_component = derive_component_with_ast(&spec, &consumer_trait)?;

    let fields = parse_getter_fields(&spec, &consumer_trait)?;

    let use_fields_impl = derive_use_fields_impl(&spec, &consumer_trait, &fields);

    let m_field: Option<[GetterField; 1]> = fields.try_into().ok();

    let mut derived = quote! {
        #derived_component

        #use_fields_impl
    };

    if let Some([field]) = m_field {
        let use_field_impl = derive_use_field_impl(&spec, &consumer_trait, &field);
        let use_provider_impl = derive_with_provider_impl(&spec, &consumer_trait, &field);

        derived.extend(use_field_impl);
        derived.extend(use_provider_impl);
    }

    Ok(derived)
}
