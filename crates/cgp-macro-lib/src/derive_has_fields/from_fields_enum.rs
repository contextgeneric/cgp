use quote::quote;
use syn::{parse2, ItemEnum, ItemImpl};

use crate::derive_has_fields::from_fields_struct::derive_from_field_params;

pub fn derive_from_fields_for_enum(item_enum: &ItemEnum) -> syn::Result<ItemImpl> {
    let enum_name = &item_enum.ident;
    let (impl_generics, type_generics, where_clause) = item_enum.generics.split_for_impl();

    let mut match_expr = quote! {
        match rest {}
    };

    for variant in item_enum.variants.iter().rev() {
        let variant_ident = &variant.ident;

        let (product_arg, product_constructor_args) = derive_from_field_params(&variant.fields)?;

        match_expr = quote! {
            match rest {
                σ::Left( field ) => {
                    let #product_arg = field.value;
                    Self:: #variant_ident #product_constructor_args
                }
                σ::Right(rest) => {
                    #match_expr
                }
            }
        }
    }

    let item_impl = quote! {
        impl #impl_generics
            FromFields for #enum_name #type_generics
        #where_clause
        {
            fn from_fields(
                rest: Self::Fields,
            ) -> Self {
                #match_expr
            }
        }
    };

    parse2(item_impl)
}
