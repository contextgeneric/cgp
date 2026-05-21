use syn::{Generics, Type};

pub struct EvaluatedDelegateEntry {
    pub generics: Generics,
    pub key: Type,
    pub value: Type,
}

pub trait EvalDelegateEntry {
    fn eval(&self, context_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>>;
}
