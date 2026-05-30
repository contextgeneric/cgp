use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::AngleBracketedGenericArguments;
use syn::parse::{Parse, ParseStream};
use syn::token::Lt;

pub struct GenericArguments {
    pub args: Option<AngleBracketedGenericArguments>,
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
