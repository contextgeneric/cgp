use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, TypeParamBound, parse_quote, parse2};

use crate::derive_getter::{FieldMode, GetterField};

pub fn derive_getter_constraint(
    spec: &GetterField,
    field_symbol: TokenStream,
    field_assoc_type: &Option<Ident>,
) -> syn::Result<TypeParamBound> {
    let field_type = match field_assoc_type {
        Some(field_assoc_type) => parse_quote! { #field_assoc_type },
        None => spec.field_type.clone(),
    };

    let constraint = if spec.field_mut.is_none() {
        if let FieldMode::Slice = spec.field_mode {
            quote! {
                HasField< #field_symbol, Value: AsRef< [ #field_type ] > + 'static >
            }
        } else {
            quote! {
                HasField< #field_symbol, Value = #field_type >
            }
        }
    } else {
        quote! {
            HasFieldMut< #field_symbol, Value = #field_type >
        }
    };

    parse2(constraint)
}
