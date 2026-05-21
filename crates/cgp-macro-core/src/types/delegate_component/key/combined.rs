use syn::parse::{Parse, ParseStream};
use syn::token::{At, Bracket};

use crate::types::delegate_component::{MultiDelegateKey, PathDelegateKey, SingleDelegateKey};
use crate::types::generics::ImplGenerics;

pub enum DelegateKey {
    Single(SingleDelegateKey),
    Multi(MultiDelegateKey),
    Path(PathDelegateKey),
}

impl Parse for DelegateKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        let _generics: ImplGenerics = fork.parse()?;

        let key = if fork.peek(At) {
            let path = input.parse()?;
            Self::Path(path)
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
