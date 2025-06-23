use proc_macro2::Span;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::token::{Comma, Gt, Lt};
use syn::{Ident, Type};

pub struct MaybeResultType {
    pub success_type: Type,
    pub error_type: Option<Type>,
}

impl Parse for MaybeResultType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        if fork.parse::<Ident>().ok() == Some(Ident::new("Result", Span::call_site())) {
            input.advance_to(&fork);

            let _: Lt = input.parse()?;

            let success_type = input.parse()?;

            let _: Comma = input.parse()?;

            let error_type = input.parse()?;

            let _: Gt = input.parse()?;

            Ok(Self {
                success_type,
                error_type: Some(error_type),
            })
        } else {
            Ok(Self {
                success_type: input.parse()?,
                error_type: None,
            })
        }
    }
}
