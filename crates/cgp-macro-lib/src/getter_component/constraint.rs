use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, TypeParamBound};

use crate::getter_component::GetterField;

pub fn derive_getter_constraint(
    spec: &GetterField,
    field_symbol: TokenStream,
) -> syn::Result<TypeParamBound> {
    let provider_type = &spec.field_type;

    let constraint = if spec.field_mut.is_none() {
        quote! {
            HasField< #field_symbol, Value = #provider_type >
        }
    } else {
        quote! {
            HasFieldMut< #field_symbol, Value = #provider_type >
        }
    };

    parse2(constraint)
}
