use syn::Type;

#[derive(Clone)]
/// How a getter takes its context.
pub enum ContextArg {
    /// The getter takes `self`.
    SelfArg,
    /// The getter takes a typed context.
    Type(Box<Type>),
}
