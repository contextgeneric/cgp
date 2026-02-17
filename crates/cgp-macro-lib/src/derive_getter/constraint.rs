use proc_macro2::TokenStream;
use quote::quote;
use syn::token::Mut;
use syn::{Ident, Type, TypeParamBound, parse_quote, parse2};

use crate::derive_getter::FieldMode;

pub fn derive_getter_constraint(
    field_type: &Type,
    field_mut: &Option<Mut>,
    field_mode: &FieldMode,
    field_symbol: TokenStream,
    field_assoc_type: &Option<Ident>,
) -> syn::Result<TypeParamBound> {
    let field_type = match field_assoc_type {
        Some(field_assoc_type) => parse_quote! { #field_assoc_type },
        None => field_type.clone(),
    };

    let constraint = if field_mut.is_none() {
        if let FieldMode::Slice = field_mode {
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
