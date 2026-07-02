use syn::{Generics, Type};

/// A lowered key: the key type plus the generics it introduces. One source key
/// can yield several of these (array keys, brace groups in a path).
pub struct EvaluatedDelegateKey {
    pub generics: Generics,
    pub key: Type,
}

/// Lower a key form into its evaluated keys.
pub trait EvalDelegateKey {
    fn eval(&self) -> syn::Result<Vec<EvaluatedDelegateKey>>;
}
