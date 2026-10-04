use proc_macro2::Span;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::token::{Comma, Gt, Lt};
use syn::{Ident, Type};

/// The return type of a `#[cgp_computer]` function, read for whether it is
/// fallible. The check is purely syntactic: only a bare two-argument
/// `Result<T, E>` counts, and any other type (a qualified
/// `core::result::Result<T, E>` included) is a plain value. A type that starts
/// with `Result` but is not that exact form is rejected.
pub struct MaybeResultType {
    /// The `E` of `Result<T, E>`, or `None` for a plain value.
    pub error_type: Option<Type>,
}

impl Parse for MaybeResultType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        if fork.parse::<Ident>().ok() == Some(Ident::new("Result", Span::call_site())) {
            input.advance_to(&fork);

            let _: Lt = input.parse()?;

            input.parse::<Type>()?;

            let _: Comma = input.parse()?;

            let error_type = input.parse()?;

            let _: Gt = input.parse()?;

            Ok(Self {
                error_type: Some(error_type),
            })
        } else {
            input.parse::<Type>()?;

            Ok(Self { error_type: None })
        }
    }
}
