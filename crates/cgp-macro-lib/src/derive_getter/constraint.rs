use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, TypeParamBound};

use crate::derive_getter::{FieldMode, GetterField};

pub fn derive_getter_constraint(
    spec: &GetterField,
    field_symbol: TokenStream,
) -> syn::Result<TypeParamBound> {
    let provider_type = &spec.field_type;

    let constraint = if spec.field_mut.is_none() {
        if let FieldMode::Slice = spec.field_mode {
            quote! {
                HasField< #field_symbol, Value: AsRef< [ #provider_type ] > + 'static >
            }
        } else {
            quote! {
                HasField< #field_symbol, Value = #provider_type >
            }
        }
    } else {
        quote! {
            HasFieldMut< #field_symbol, Value = #provider_type >
        }
    };

    parse2(constraint)
}
