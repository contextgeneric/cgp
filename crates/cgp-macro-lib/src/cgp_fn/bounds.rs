use quote::ToTokens;
use syn::TypeParamBound;
use syn::punctuated::Punctuated;
use syn::token::Plus;

use crate::cgp_fn::ImplicitArgField;
use crate::derive_getter::derive_getter_constraint;
use crate::symbol::symbol_from_string;

pub fn build_implicit_args_bounds(
    implicit_args: &[ImplicitArgField],
) -> syn::Result<Punctuated<TypeParamBound, Plus>> {
    let mut constraints: Punctuated<TypeParamBound, Plus> = Punctuated::new();

    for arg in implicit_args {
        let field_symbol = symbol_from_string(&arg.field_name.to_string());

        let constraint = derive_getter_constraint(
            &arg.field_type,
            &arg.field_mut,
            &arg.field_mode,
            field_symbol.to_token_stream(),
            &None,
        )?;

        constraints.push(constraint);
    }

    Ok(constraints)
}
