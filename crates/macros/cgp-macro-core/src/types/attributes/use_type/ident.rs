use syn::parse::{Parse, ParseStream};
use syn::token::{As, Eq};
use syn::{Ident, Type};

#[derive(Clone)]
pub struct UseTypeIdent {
    pub type_ident: Ident,
    pub as_alias: Option<Ident>,
    pub equals: Option<Type>,
}

impl UseTypeIdent {
    pub fn alias_ident(&self) -> &Ident {
        self.as_alias.as_ref().unwrap_or(&self.type_ident)
    }
}

impl Parse for UseTypeIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let type_ident: Ident = input.parse()?;

        let as_alias = if input.peek(As) {
            let _: As = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        let equals = if input.peek(Eq) {
            let _: Eq = input.parse()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self {
            type_ident,
            as_alias,
            equals,
        })
    }
}
