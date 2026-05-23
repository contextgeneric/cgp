use syn::Type;
use syn::parse::{Parse, ParseStream};

use crate::types::delegate_component::{EvalDelegateKey, EvaluatedDelegateKey};
use crate::types::generics::ImplGenerics;

#[derive(Debug, Clone)]
pub struct SingleDelegateKey {
    pub generics: ImplGenerics,
    pub ty: Type,
}

impl Parse for SingleDelegateKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let generics = input.parse()?;
        let ty = input.parse()?;

        Ok(Self { generics, ty })
    }
}

impl EvalDelegateKey for SingleDelegateKey {
    fn eval(&self) -> syn::Result<Vec<EvaluatedDelegateKey>> {
        let key = EvaluatedDelegateKey {
            generics: self.generics.generics.clone(),
            key: self.ty.clone(),
        };

        Ok(vec![key])
    }
}
