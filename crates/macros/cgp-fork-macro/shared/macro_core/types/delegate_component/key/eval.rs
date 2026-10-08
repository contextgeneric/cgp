use proc_macro2::Span;
use syn::{Generics, Type};

/// A lowered key: the key type plus the generics it introduces. One source key
/// can yield several of these (array keys, brace groups in a path).
pub struct EvaluatedDelegateKey {
    pub generics: Generics,
    pub key: Type,
    /// The span of the source token the key was written as, carried separately
    /// because `key` may be a synthesized type (a `PathCons<..>` nest for an
    /// `@`-path key) whose own span points at the macro `call_site` rather than
    /// at what the user wrote. It becomes the entry's diagnostic span.
    pub span: Span,
}

/// Lower a key form into its evaluated keys.
pub trait EvalDelegateKey {
    fn eval(&self) -> syn::Result<Vec<EvaluatedDelegateKey>>;
}
