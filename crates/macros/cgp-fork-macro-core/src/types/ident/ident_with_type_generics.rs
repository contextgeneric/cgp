use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Type, parse_quote};

use crate::types::ident::TypeGenericParams;

/// An identifier followed by an optional definition-site generic parameter
/// list, e.g. `Foo`, `Foo<A, B>`, or `Bar<'a, C>`.
#[derive(Debug, Clone)]
pub struct IdentWithTypeGenerics {
    pub ident: Ident,
    pub type_generics: TypeGenericParams,
}

impl IdentWithTypeGenerics {
    pub fn to_type(&self) -> Type {
        parse_quote!(#self)
    }
}

impl From<Ident> for IdentWithTypeGenerics {
    fn from(ident: Ident) -> Self {
        Self {
            ident,
            type_generics: TypeGenericParams::default(),
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
