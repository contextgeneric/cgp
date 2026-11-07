use syn::Type;

/// Lower a mapping value into the provider `Type` stored as the entry's `Delegate`.
/// Lowers a mapping value into its type.
pub trait EvalDelegateValue {
    /// Lowers the value.
    fn eval(&self) -> syn::Result<Type>;
}
