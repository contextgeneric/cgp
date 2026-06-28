use syn::{ItemEnum, ItemImpl};

use crate::exports::FromVariant;
use crate::parse_internal;
use crate::types::cgp_data::get_variant_type;
use crate::types::field::Symbol;

pub fn derive_from_variant_from_enum(item_enum: &ItemEnum) -> syn::Result<Vec<ItemImpl>> {
    let enum_ident = &item_enum.ident;

    let (impl_generics, ty_generics, where_clause) = item_enum.generics.split_for_impl();

    let mut item_impls: Vec<ItemImpl> = Vec::new();

    for variant in item_enum.variants.iter() {
        let variant_ident = &variant.ident;
        let variant_tag = Symbol::from_ident(variant_ident.clone());
        let variant_type = get_variant_type(variant)?;

        let item_impl: ItemImpl = parse_internal! {
            impl #impl_generics #FromVariant<#variant_tag> for #enum_ident #ty_generics
            #where_clause
            {
                type Value = #variant_type;

                fn from_variant(_tag: ::core::marker::PhantomData<#variant_tag>, value: Self::Value) -> Self {
                    Self::#variant_ident(value)
                }
            }
        };

        item_impls.push(item_impl);
    }

    Ok(item_impls)
}
