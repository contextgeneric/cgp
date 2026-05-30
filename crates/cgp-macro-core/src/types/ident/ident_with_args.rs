use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Type, parse_quote};

use crate::types::generics::GenericArguments;

#[derive(Debug, Clone)]
pub struct IdentWithTypeArgs {
    pub ident: Ident,
    pub type_args: GenericArguments,
}

impl Parse for IdentWithTypeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let type_args = input.parse()?;

        Ok(Self { ident, type_args })
    }
}

impl ToTokens for IdentWithTypeArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
        self.type_args.to_tokens(tokens);
    }
}

impl From<Ident> for IdentWithTypeArgs {
    fn from(ident: Ident) -> Self {
        Self {
            ident,
            type_args: GenericArguments::default(),
        }
    }
}

impl From<IdentWithTypeArgs> for Type {
    fn from(value: IdentWithTypeArgs) -> Self {
        parse_quote!(#value)
    }
}
