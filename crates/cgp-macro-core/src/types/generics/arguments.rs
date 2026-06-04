use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Lt};
use syn::{AngleBracketedGenericArguments, GenericArgument, parse_quote};

use crate::types::generics::TypeGenerics;

#[derive(Debug, Clone, Default)]
pub struct GenericArguments {
    pub args: Option<AngleBracketedGenericArguments>,
}

impl GenericArguments {
    pub fn make_args(&mut self) -> &mut Punctuated<GenericArgument, Comma> {
        &mut self.args.get_or_insert_with(|| parse_quote!(<>)).args
    }
}

impl Parse for GenericArguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Lt) {
            let args = input.parse()?;
            Ok(Self { args: Some(args) })
        } else {
            Ok(Self { args: None })
        }
    }
}

impl ToTokens for GenericArguments {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(args) = &self.args {
            args.to_tokens(tokens);
        }
    }
}

impl From<TypeGenerics> for GenericArguments {
    fn from(generics: TypeGenerics) -> Self {
        if generics.params.is_empty() {
            Self { args: None }
        } else {
            let args = parse_quote!(#generics);
            Self { args: Some(args) }
        }
    }
}
