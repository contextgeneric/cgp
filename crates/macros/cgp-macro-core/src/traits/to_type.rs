use syn::Type;

/// Converts a value into a `syn::Type`.
pub trait ToType {
    /// The type the value represents.
    fn to_type(&self) -> Type;
}
