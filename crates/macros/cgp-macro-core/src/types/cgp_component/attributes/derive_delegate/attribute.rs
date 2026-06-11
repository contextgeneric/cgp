use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Gt, Lt, Paren};
use syn::{Error, Ident, parenthesized};

pub struct DeriveDelegateAttribute {
    pub wrapper: Ident,
    pub params: Punctuated<Ident, Comma>,
}

impl Parse for DeriveDelegateAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let wrapper: Ident = input.parse()?;

        let _: Lt = input.parse()?;

        let idents = if input.peek(Paren) {
            let body;
            parenthesized!(body in input);
            let idents = Punctuated::parse_terminated(&body)?;
            if idents.is_empty() {
                return Err(Error::new(
                    body.span(),
                    "expect non-empty tuple list of identifiers in use_delegate_spec",
                ));
            }

            idents
        } else {
            let ident: Ident = input.parse()?;
            Punctuated::from_iter([ident])
        };

        let _: Gt = input.parse()?;
        Ok(Self {
            wrapper,
            params: idents,
        })
    }
}
