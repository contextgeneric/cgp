use core::any::type_name;

use proc_macro2::TokenStream;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::{Error, parse2};

use crate::functions::strip_macro_prelude;
pub use crate::macros::parse_internal;

/// Parse a token stream into a `syn` type `T`, attaching an error that names both
/// `T` and the offending tokens (prelude prefix stripped) on failure. Usually
/// invoked through the `parse_internal!` macro.
pub fn parse_internal<T>(body: TokenStream) -> Result<T, Error>
where
    T: Parse,
{
    parse2(body.clone()).map_err(|mut e| {
        e.combine(Error::new(
            body.span(),
            format!(
                "failed to parse internal tokens to type `{}`:\n{}",
                type_name::<T>(),
                strip_macro_prelude(body.clone()),
            ),
        ));
        e
    })
}
