use syn::Type;

/// How a getter takes its context.
#[derive(Clone)]
pub enum ContextArg {
    /// The getter takes `self`.
    SelfArg,
    /// The getter takes a typed context.
    Type(Box<Type>),
}
