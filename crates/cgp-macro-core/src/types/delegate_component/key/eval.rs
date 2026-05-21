use syn::{Generics, Type};

pub struct EvaluatedDelegateKey {
    pub generics: Generics,
    pub key: Type,
}

pub trait EvalDelegateKey {
    fn eval(&self) -> syn::Result<Vec<EvaluatedDelegateKey>>;
}
