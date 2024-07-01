use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Gt, Lt};
use syn::Ident;

pub struct ComponentSpec {
    pub provider_name: Ident,
    pub context_type: Ident,
    pub component_name: Ident,
    pub component_params: Punctuated<Ident, Comma>,
}

impl Parse for ComponentSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let component_name: Ident = input.parse()?;

        let component_params = if input.peek(Lt) {
            let _: Lt = input.parse()?;

            let component_params: Punctuated<Ident, Comma> =
                Punctuated::parse_separated_nonempty(input)?;

            let _: Gt = input.parse()?;

            component_params
        } else {
            Punctuated::default()
        };

        let _: Comma = input.parse()?;

        let provider_name: Ident = input.parse()?;

        let _: Lt = input.parse()?;

        let context_type: Ident = input.parse()?;

        let _: Gt = input.parse()?;

        Ok(ComponentSpec {
            component_name,
            provider_name,
            context_type,
            component_params,
        })
    }
}
