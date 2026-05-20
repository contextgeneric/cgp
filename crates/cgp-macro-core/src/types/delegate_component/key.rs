use syn::bracketed;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Comma, Type};

use crate::types::generics::ImplGenerics;

pub enum DelegateKey {
    Single(SingleDelegateKey),
    Multi(MultiDelegateKey),
}

pub struct SingleDelegateKey {
    pub generics: ImplGenerics,
    pub ty: Type,
}

pub struct MultiDelegateKey {
    pub keys: Punctuated<SingleDelegateKey, Comma>,
}

impl Parse for DelegateKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Bracket) {
            let keys = input.parse()?;
            Ok(Self::Multi(keys))
        } else {
            let key = input.parse()?;
            Ok(Self::Single(key))
        }
    }
}

impl Parse for SingleDelegateKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generics = input.parse()?;
        let ty = input.parse()?;

        Ok(Self { generics, ty })
    }
}

impl Parse for MultiDelegateKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body;
        bracketed!(body in input);
        let keys = Punctuated::parse_terminated(&body)?;

        Ok(Self { keys })
    }
}
