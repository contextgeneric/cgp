use cgp_macro_core::types::field::FieldName;
use cgp_macro_core::types::implicits::ImplicitArgField;
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{TypeParamBound, parse_quote};

use crate::derive_getter::derive_getter_constraint;

pub fn build_implicit_args_bounds(
    implicit_args: &[ImplicitArgField],
) -> syn::Result<Punctuated<TypeParamBound, Plus>> {
    let mut constraints: Punctuated<TypeParamBound, Plus> = Punctuated::new();

    for arg in implicit_args {
        let field_name = FieldName::from(arg.field_name.clone());
        let tag_type = parse_quote!(#field_name);

        let constraint =
            derive_getter_constraint(&arg.field_type, &arg.field_mut, &arg.field_mode, &tag_type)?;

        constraints.push(constraint);
    }

    Ok(constraints)
}
