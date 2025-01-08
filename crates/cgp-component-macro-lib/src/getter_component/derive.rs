use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Ident, ItemTrait};

use crate::derive_component::component_spec::ComponentSpec;
use crate::derive_component::derive::derive_component_with_ast;
use crate::getter_component::blanket::derive_blanket_impl;
use crate::getter_component::getter_field::GetterField;
use crate::getter_component::parse::parse_getter_fields;
use crate::getter_component::use_field::derive_use_field_impl;
use crate::getter_component::use_fields::derive_use_fields_impl;
use crate::getter_component::with_provider::derive_with_provider_impl;

pub fn derive_getter_component(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let spec: ComponentSpec = syn::parse2(attr)?;
    let consumer_trait: ItemTrait = syn::parse2(body)?;

    let derived_component = derive_component_with_ast(&spec, &consumer_trait)?;

    let fields = parse_getter_fields(&spec.context_type, &consumer_trait)?;

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

pub fn derive_auto_getter_component(
    attr: TokenStream,
    body: TokenStream,
) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "#[derive_auto_getter] does not accept any attribute argument",
        ));
    }

    let consumer_trait: ItemTrait = syn::parse2(body)?;

    let context_type = Ident::new("Context", Span::call_site());

    let fields = parse_getter_fields(&context_type, &consumer_trait)?;

    let blanket_impl = derive_blanket_impl(&context_type, &consumer_trait, &fields);

    Ok(quote! {
        #consumer_trait
        #blanket_impl
    })
}
