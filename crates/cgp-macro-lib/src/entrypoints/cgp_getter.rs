use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, ItemTrait, Type, parse_quote, parse2};

use crate::derive_component::derive_component_with_ast;
use crate::derive_getter::{
    GetterField, derive_use_field_impl, derive_use_fields_impl, derive_with_provider_impl,
    parse_getter_fields,
};
use crate::derive_provider::derive_is_provider_for;
use crate::parse::{ComponentSpec, Entries};

pub fn cgp_getter(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let mut entries = if let Ok(provider_ident) = parse2::<Ident>(attr.clone()) {
        BTreeMap::from([("provider".to_owned(), provider_ident.to_token_stream())])
    } else {
        parse2::<Entries>(attr)?.entries
    };

    let consumer_trait: ItemTrait = syn::parse2(body)?;

    let provider_entry = entries.entry("provider".to_owned());

    if let Entry::Vacant(entry) = provider_entry {
        let consumer_name = consumer_trait.ident.to_string();
        if let Some(field_name) = consumer_name.strip_prefix("Has")
            && !field_name.is_empty()
        {
            let provider_name =
                Ident::new(&format!("{field_name}Getter"), consumer_trait.ident.span());
            entry.insert(parse2(provider_name.to_token_stream())?);
        }
    }

    let spec = ComponentSpec::from_entries(&entries)?;

    let derived_component = derive_component_with_ast(&spec, consumer_trait.clone())?;

    let (fields, field_assoc_type) = parse_getter_fields(&spec.context_type, &consumer_trait)?;

    let use_fields_impl = derive_use_fields_impl(
        &spec,
        &derived_component.provider_trait,
        &fields,
        &field_assoc_type,
    )?;

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
        let use_field_impl = derive_use_field_impl(
            &spec,
            &derived_component.provider_trait,
            &field,
            &field_assoc_type,
        )?;

        let is_provider_use_field_impl =
            derive_is_provider_for(&component_name_type, &use_field_impl)?;

        let use_provider_impl = derive_with_provider_impl(
            &spec,
            &derived_component.provider_trait,
            &field,
            &field_assoc_type,
        )?;

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
