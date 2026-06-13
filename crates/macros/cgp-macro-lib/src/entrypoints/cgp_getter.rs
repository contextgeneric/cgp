use cgp_macro_core::types::cgp_component::{
    CgpComponentRawArgs, EvaluatedCgpComponent, ItemCgpComponent,
};
use cgp_macro_core::types::provider_impl::derive_is_provider_for;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemTrait, Type, parse_quote, parse2};

use crate::derive_getter::{
    GetterField, derive_use_field_impl, derive_use_fields_impl, derive_with_provider_impl,
    parse_getter_fields,
};

pub fn cgp_getter(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let mut raw_args: CgpComponentRawArgs = parse2(attr.clone())?;

    let item_trait: ItemTrait = syn::parse2(body)?;

    if raw_args.provider_ident.is_none()
        && let Some(field_name) = item_trait.ident.to_string().strip_prefix("Has")
        && !field_name.is_empty()
    {
        raw_args.provider_ident = Some(Ident::new(
            &format!("{field_name}Getter"),
            item_trait.ident.span(),
        ));
    }

    let item_cgp_component = ItemCgpComponent {
        args: raw_args.try_into()?,
        item_trait,
    };

    let evalutated = item_cgp_component.preprocess()?.eval()?;

    let items = evalutated.to_items()?;

    let EvaluatedCgpComponent {
        args,
        consumer_trait,
        provider_trait,
        ..
    } = evalutated;

    let (fields, field_assoc_type) = parse_getter_fields(&args.context_ident, &consumer_trait)?;

    let use_fields_impl =
        derive_use_fields_impl(&args, &provider_trait, &fields, &field_assoc_type)?;

    let component_name_type: Type = {
        let component_name = &args.component_name;
        parse_quote!( #component_name )
    };

    let is_provider_use_fields_impl =
        derive_is_provider_for(&component_name_type, &use_fields_impl)?;

    let m_field: Option<[GetterField; 1]> = fields.try_into().ok();

    let mut derived = quote! {
        #( #items )*

        #use_fields_impl

        #is_provider_use_fields_impl
    };

    if let Some([field]) = m_field {
        let use_field_impl =
            derive_use_field_impl(&args, &provider_trait, &field, &field_assoc_type)?;

        let is_provider_use_field_impl =
            derive_is_provider_for(&component_name_type, &use_field_impl)?;

        let use_provider_impl =
            derive_with_provider_impl(&args, &provider_trait, &field, &field_assoc_type)?;

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
