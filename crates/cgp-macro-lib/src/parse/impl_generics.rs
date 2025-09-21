use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Error, Generics, TypeGenerics, parse2};

#[derive(Clone, Default)]
pub struct ImplGenerics {
    pub generics: Generics,
}

impl ImplGenerics {
    pub fn as_type_generics(&self) -> TypeGenerics<'_> {
        let (_, type_generics, _) = self.generics.split_for_impl();
        type_generics
    }
}

impl Parse for ImplGenerics {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generics: Generics = input.parse()?;

        let (impl_generics, _, _) = generics.split_for_impl();

        let generics2: Generics = parse2(impl_generics.to_token_stream())?;

        if generics != generics2 {
            return Err(Error::new_spanned(generics, "invalid impl generics syntax"));
        }

        Ok(Self { generics })
    }
}

impl ToTokens for ImplGenerics {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.generics.to_tokens(tokens);
    }
}
