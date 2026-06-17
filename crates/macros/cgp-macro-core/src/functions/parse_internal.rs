use core::any::type_name;

use proc_macro2::TokenStream;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::{Error, parse2};

pub fn parse_internal<T>(body: TokenStream) -> Result<T, Error>
where
    T: Parse,
{
    parse2(body.clone()).map_err(|e| {
        Error::new(
            body.span(),
            format!(
                "failed to parse internal tokens to type {}.\nerror: {}\tokens:\n{}",
                type_name::<T>(),
                e,
                body,
            ),
        )
    })
}
