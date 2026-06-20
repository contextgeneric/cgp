use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Type, parse_quote};

use crate::types::ident::TypeGenericParams;

/// An identifier followed by an optional definition-site generic parameter
/// list, e.g. `Foo`, `Foo<A, B>`, or `Bar<'a, C>`.
///
/// This is the intended replacement for `IdentWithTypeGenerics`. The difference
/// is that the parameter list is modelled by [`TypeGenericParams`] rather than
/// `syn::Generics`, so only simple, unconstrained parameters are accepted.
/// Invalid forms such as `Foo<A: Clone>` (bounds) and `Foo<A = B>` (defaults)
/// are rejected at parse time, without the `split_for_impl` round-trip hack
/// used by the original `TypeGenerics`.
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
