use syn::bracketed;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;

use crate::types::delegate_component::{EvalDelegateKey, EvaluatedDelegateKey, SingleDelegateKey};

#[derive(Debug, Clone)]
pub struct MultiDelegateKey {
    pub keys: Punctuated<SingleDelegateKey, Comma>,
}

impl Parse for MultiDelegateKey {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body;
        bracketed!(body in input);
        let keys = Punctuated::parse_terminated(&body)?;

        Ok(Self { keys })
    }
}

impl EvalDelegateKey for MultiDelegateKey {
    fn eval(&self) -> syn::Result<Vec<EvaluatedDelegateKey>> {
        let mut keys = Vec::new();

        for key in &self.keys {
            let key = EvaluatedDelegateKey {
                generics: key.generics.generics.clone(),
                key: key.ty.clone(),
            };

            keys.push(key)
        }

        Ok(keys)
    }
}
