use syn::{ReturnType, Type};

use crate::macro_core::parse_internal;

/// The type a function returns, reading an omitted return type as `()`.
pub fn return_type(output: &ReturnType) -> syn::Result<Type> {
    match output {
        ReturnType::Type(_, ty) => Ok(ty.as_ref().clone()),
        ReturnType::Default => Ok(parse_internal!(())),
    }
}
