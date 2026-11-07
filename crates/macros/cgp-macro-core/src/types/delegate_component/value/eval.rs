use syn::Type;

/// Lower a mapping value into the provider `Type` stored as the entry's `Delegate`.
pub trait EvalDelegateValue {
    /// Lowers the value.
    fn eval(&self) -> syn::Result<Type>;
}
