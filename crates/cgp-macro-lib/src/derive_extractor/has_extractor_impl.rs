use quote::quote;
use syn::{parse2, Arm, Ident, ItemEnum, ItemImpl};

use crate::derive_builder::to_generic_args;

pub fn derive_has_extractor_impl(
    context_enum: &ItemEnum,
    extractor_ident: &Ident,
) -> syn::Result<ItemImpl> {
    let (impl_generics, ty_generics, where_clause) = context_enum.generics.split_for_impl();

    let context_ident = &context_enum.ident;

    let mut extractor_generics = to_generic_args(&context_enum.generics)?;

    let mut match_arms = Vec::<Arm>::new();

    for (i, variant) in context_enum.variants.iter().enumerate() {
        extractor_generics.args.push(parse2(quote! {
            IsPresent
        })?);

        let variant_ident = &variant.ident;

        match_arms.push(parse2(quote! {
            Self :: #variant_ident ( value ) => {
                #extractor_ident:: #variant_ident ( value )
            }
        })?);
    }

    let item_impl = parse2(quote! {
        impl #impl_generics HasExtractor
            for #context_ident #ty_generics
        #where_clause
        {
            type Extractor = #extractor_ident #extractor_generics;

            fn extractor(self) -> Self::Extractor {
                match self {
                    #(#match_arms)*
                }
            }
        }
    })?;

    Ok(item_impl)
}
