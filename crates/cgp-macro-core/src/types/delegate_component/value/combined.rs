use syn::Type;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};

use crate::types::delegate_component::{DelegateValueWithInnerEntries, EvalDelegateValue};

pub enum DelegateValue {
    Type(Type),
    WithInner(DelegateValueWithInnerEntries),
}

impl Parse for DelegateValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();

        if let Ok(value) = fork.parse::<DelegateValueWithInnerEntries>() {
            input.advance_to(&fork);
            return Ok(Self::WithInner(value));
        }

        let ty: Type = input.parse()?;
        Ok(Self::Type(ty))
    }
}

impl EvalDelegateValue for DelegateValue {
    fn eval(&self) -> syn::Result<Type> {
        match self {
            Self::Type(ty) => Ok(ty.clone()),
            Self::WithInner(value) => value.eval(),
        }
    }
}
