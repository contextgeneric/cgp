use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::token::{Comma, Gt, Lt};
use syn::{Error, Ident, Type};

/// The return type of a `#[cgp_computer]` function, read for whether it is
/// fallible. The check is purely syntactic: only a bare two-argument
/// `Result<T, E>` counts, and any other type (a qualified
/// `core::result::Result<T, E>` included) is a plain value. A type that starts
/// with `Result` but is not that exact form, such as a one-argument alias
/// `Result<T>`, is rejected, since the macro cannot tell whether it is fallible.
pub struct MaybeResultType {
    /// The `E` of `Result<T, E>`, or `None` for a plain value.
    pub error_type: Option<Type>,
}

impl Parse for MaybeResultType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();

        if let Ok(ident) = fork.parse::<Ident>()
            && ident == "Result"
        {
            input.advance_to(&fork);

            let error_type = parse_result_arguments(input).map_err(|_| {
                Error::new(
                    ident.span(),
                    "A `Result` return type must be written as `Result<T, E>`, naming its error type",
                )
            })?;

            Ok(Self {
                error_type: Some(error_type),
            })
        } else {
            input.parse::<Type>()?;

            Ok(Self { error_type: None })
        }
    }
}

/// Parse the `<T, E>` after `Result`, returning `E`.
fn parse_result_arguments(input: ParseStream) -> syn::Result<Type> {
    let _: Lt = input.parse()?;
    let _: Type = input.parse()?;
    let _: Comma = input.parse()?;
    let error_type = input.parse()?;
    let _: Gt = input.parse()?;

    Ok(error_type)
}
