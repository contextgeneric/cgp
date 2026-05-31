use cgp_macro_core::types::getter::FieldMode;
use quote::quote;
use syn::token::Mut;
use syn::{Type, TypeParamBound, parse2};

pub fn derive_getter_constraint(
    field_type: &Type,
    field_mut: &Option<Mut>,
    field_mode: &FieldMode,
    tag_type: &Type,
) -> syn::Result<TypeParamBound> {
    let constraint = if field_mut.is_none() {
        if let FieldMode::Slice = field_mode {
            quote! {
                HasField< #tag_type, Value: AsRef< [ #field_type ] > + 'static >
            }
        } else {
            quote! {
                HasField< #tag_type, Value = #field_type >
            }
        }
    } else {
        quote! {
            HasFieldMut< #tag_type, Value = #field_type >
        }
    };

    parse2(constraint)
}
