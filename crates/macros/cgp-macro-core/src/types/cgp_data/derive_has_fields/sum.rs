use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Type, Variant, parse2};

use crate::exports::{Either, Field, Void};
use crate::types::cgp_data::item_fields_to_product_type;
use crate::types::field::Symbol;

pub fn variants_to_sum_type(
    variants: &Punctuated<Variant, Comma>,
    reference: &TokenStream,
) -> syn::Result<Type> {
    let mut out = quote! { #Void };

    for variant in variants.iter().rev() {
        let variant_ident = &variant.ident;
        let variant_symbol = Symbol::from_ident(variant_ident.clone());

        let variant_fields = item_fields_to_product_type(&variant.fields, reference)?;

        out = quote! {
            #Either<
                #Field< #variant_symbol, #variant_fields >,
                #out,
            >
        };
    }

    parse2(out)
}
