use cgp_macro_core::types::cgp_component::{
    CgpComponentRawArgs, EvaluatedCgpComponent, ItemCgpComponent,
};
use cgp_macro_core::types::cgp_getter::{GetterField, ItemCgpGetter};
use cgp_macro_core::types::provider_impl::derive_is_provider_for;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemTrait, Type, parse_quote, parse2};

use crate::derive_getter::{derive_use_field_impl, derive_with_provider_impl};

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

    let evaluated = item_cgp_component.preprocess()?.eval()?;

    let items = evaluated.to_items()?;

    let item_getter = ItemCgpGetter::try_from(evaluated)?;

    let use_fields_impl = item_getter.to_use_fields_impl()?.to_item_impls()?;

    let ItemCgpGetter {
        item_component:
            EvaluatedCgpComponent {
                args,
                provider_trait,
                ..
            },
        fields,
        field_assoc_type,
    } = item_getter;

    let component_name_type: Type = {
        let component_name = &args.component_name;
        parse_quote!( #component_name )
    };

    let m_field: Option<[GetterField; 1]> = fields.try_into().ok();

    let mut derived = quote! {
        #( #items )*

        #( #use_fields_impl )*
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
