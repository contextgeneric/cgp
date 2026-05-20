use syn::bracketed;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{At, Bracket, Comma, Type};

use crate::types::generics::ImplGenerics;
use crate::types::path::PathHead;

pub enum DelegateKey {
    Single(SingleDelegateKey),
    Multi(MultiDelegateKey),
    Path(PathDelegateKey),
}

pub struct SingleDelegateKey {
    pub generics: ImplGenerics,
    pub ty: Type,
}

pub struct MultiDelegateKey {
    pub keys: Punctuated<SingleDelegateKey, Comma>,
}

pub struct PathDelegateKey {
    pub generics: ImplGenerics,
    pub at: At,
    pub path: PathHead,
}

impl Parse for DelegateKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        let generics: ImplGenerics = fork.parse()?;

        let key = if fork.peek(At) {
            input.advance_to(&fork);

            let at = input.parse()?;
            let path = input.parse()?;

            Self::Path(PathDelegateKey { generics, at, path })
        } else if input.peek(Bracket) {
            let keys = input.parse()?;
            Self::Multi(keys)
        } else {
            let key = input.parse()?;
            Self::Single(key)
        };

        Ok(key)
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
