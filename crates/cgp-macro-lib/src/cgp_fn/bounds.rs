use cgp_macro_core::types::implicits::ImplicitArgField;
use syn::punctuated::Punctuated;
use syn::token::Plus;
use syn::{TypeParamBound, parse_quote};

pub fn build_implicit_args_bounds(
    implicit_args: &[ImplicitArgField],
) -> syn::Result<Punctuated<TypeParamBound, Plus>> {
    let mut constraints: Punctuated<TypeParamBound, Plus> = Punctuated::new();

    for arg in implicit_args {
        let constraint = arg.to_has_field_bound()?;
        constraints.push(parse_quote!(#constraint));
    }

    Ok(constraints)
}
