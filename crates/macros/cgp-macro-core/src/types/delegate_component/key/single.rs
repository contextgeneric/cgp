use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Type};

use crate::types::delegate_component::{EvalDelegateKey, EvaluatedDelegateKey};
use crate::types::generics::ImplGenerics;

#[derive(Debug, Clone)]
pub struct SingleDelegateKey {
    pub attributes: Vec<Attribute>,
    pub generics: ImplGenerics,
    pub ty: Type,
}

impl Parse for SingleDelegateKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attributes = input.call(Attribute::parse_outer)?;
        let generics = input.parse()?;
        let ty = input.parse()?;

        Ok(Self {
            attributes,
            generics,
            ty,
        })
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
