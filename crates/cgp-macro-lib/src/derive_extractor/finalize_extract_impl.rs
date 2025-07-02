use quote::quote;
use syn::{parse2, Arm, Ident, ItemEnum, ItemImpl, Type};

use crate::derive_builder::to_generic_args;

pub fn derive_finalize_extract_impl(
    context_enum: &ItemEnum,
    extractor_ident: &Ident,
    is_ref: bool,
) -> syn::Result<ItemImpl> {
    let generics = {
        let mut generics = context_enum.generics.clone();

        if is_ref {
            generics.params.insert(
                0,
                parse2(quote! {
                    'a
                })?,
            );
        }

        generics
    };

    let mut generic_args = to_generic_args(&generics)?;

    let mut match_arms = Vec::<Arm>::new();

    for variant in context_enum.variants.iter() {
        generic_args.args.push(parse2(quote! {
            IsVoid
        })?);

        let variant_ident = &variant.ident;

        match_arms.push(parse2(quote! {
            Self :: #variant_ident ( value ) => {
                match value {}
            }
        })?);
    }

    let (impl_generics, _, where_clause) = generics.split_for_impl();

    let extractor_type: Type = parse2(quote! {
        #extractor_ident #generic_args
    })?;

    let item_impl = parse2(quote! {
        impl #impl_generics FinalizeExtract for #extractor_type
        #where_clause
        {
            fn finalize_extract<__T__>(self) -> __T__ {
                match self {
                    #(#match_arms)*
                }
            }
        }
    })?;

    Ok(item_impl)
}
