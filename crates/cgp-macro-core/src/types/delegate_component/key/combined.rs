use syn::parse::{Parse, ParseStream};
use syn::token::{At, Bracket};

use crate::types::delegate_component::{
    EvalDelegateKey, EvaluatedDelegateKey, MultiDelegateKey, PathDelegateKey, SingleDelegateKey,
};
use crate::types::generics::ImplGenerics;

#[derive(Debug, Clone)]
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

impl EvalDelegateKey for DelegateKey {
    fn eval(&self) -> syn::Result<Vec<EvaluatedDelegateKey>> {
        match self {
            Self::Single(key) => key.eval(),
            Self::Multi(key) => key.eval(),
            Self::Path(key) => key.eval(),
        }
    }
}
