use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Type, parse_quote};

use crate::traits::ToType;
use crate::types::ident::TypeArgs;

/// An identifier followed by an optional type-expression argument list, e.g.
/// `Foo`, `Foo<A, B>`, `Foo<(A, B), C>`, or `Foo<Bar<A>, C>`.
///
/// This is the intended replacement for `IdentWithTypeArgs`. The difference is
/// that the argument list is modelled by [`TypeArgs`] rather than
/// `syn::AngleBracketedGenericArguments`, so invalid forms such as
/// `Foo<A, B = C>` are rejected at parse time.
///
/// For the path-headed counterpart (`path::to::Foo<A, B>`), see
/// [`PathWithTypeArgs`].
///
/// [`PathWithTypeArgs`]: crate::types::ident::PathWithTypeArgs
#[derive(Debug, Clone)]
pub struct IdentWithTypeArgs {
    pub ident: Ident,
    pub type_args: TypeArgs,
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
            type_args: TypeArgs::default(),
        }
    }
}

impl ToType for IdentWithTypeArgs {
    fn to_type(&self) -> Type {
        parse_quote!(#self)
    }
}

impl From<IdentWithTypeArgs> for Type {
    fn from(value: IdentWithTypeArgs) -> Self {
        parse_quote!(#value)
    }
}
