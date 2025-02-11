use syn::parse::{Parse, ParseStream};
use syn::Ident;

pub struct ContextSpec {
    pub provider_name: Ident,
}

impl Parse for ContextSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let provider_name = input.parse()?;

        Ok(Self { provider_name })
    }
}
