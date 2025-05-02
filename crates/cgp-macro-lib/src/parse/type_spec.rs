use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::token::Lt;
use syn::Ident;

use crate::parse::ImplGenerics;

pub struct TypeSpec {
    pub name: Ident,
    pub generics: Option<ImplGenerics>,
}

impl Parse for TypeSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let generics = if input.peek(Lt) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self { name, generics })
    }
}

impl ToTokens for TypeSpec {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.name.to_tokens(tokens);
        self.generics.to_tokens(tokens);
    }
}
