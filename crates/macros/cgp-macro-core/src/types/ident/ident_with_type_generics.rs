use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Ident;
use syn::parse::{Parse, ParseStream};

use crate::types::generics::TypeGenerics;

#[derive(Debug, Clone)]
pub struct IdentWithTypeGenerics {
    pub ident: Ident,
    pub type_generics: TypeGenerics,
}

impl From<Ident> for IdentWithTypeGenerics {
    fn from(ident: Ident) -> Self {
        Self {
            ident,
            type_generics: TypeGenerics::default(),
        }
    }
}

impl Parse for IdentWithTypeGenerics {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let type_generics = input.parse()?;

        Ok(Self {
            ident,
            type_generics,
        })
    }
}

impl ToTokens for IdentWithTypeGenerics {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
        self.type_generics.to_tokens(tokens);
    }
}
