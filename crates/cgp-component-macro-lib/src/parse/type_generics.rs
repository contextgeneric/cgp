use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{parse2, Error, Generics};

pub struct TypeGenerics {
    pub generics: Generics,
}

impl Parse for TypeGenerics {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generics: Generics = input.parse()?;

        let (_, type_generics, _) = generics.split_for_impl();

        let generics2: Generics = parse2(type_generics.to_token_stream())?;

        if generics != generics2 {
            return Err(Error::new_spanned(generics, "invalid type generics syntax"));
        }

        Ok(Self { generics })
    }
}

impl ToTokens for TypeGenerics {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.generics.to_tokens(tokens);
    }
}
