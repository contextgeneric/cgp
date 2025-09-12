use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse2, Type, Variant};

use crate::derive_has_fields::product::item_fields_to_product_type;
use crate::symbol::symbol_from_string;

pub fn variants_to_sum_type(
    variants: &Punctuated<Variant, Comma>,
    reference: &TokenStream,
) -> syn::Result<Type> {
    let mut out = quote! { θ };

    for variant in variants.iter().rev() {
        let variant_ident = &variant.ident;
        let variant_symbol = symbol_from_string(&variant_ident.to_string());

        let variant_fields = item_fields_to_product_type(&variant.fields, reference)?;

        out = quote! {
            σ<
                ω< #variant_symbol, #variant_fields >,
                #out,
            >
        };
    }

    parse2(out)
}
