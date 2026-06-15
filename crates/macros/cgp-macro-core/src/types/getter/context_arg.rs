use syn::Type;

#[derive(Clone)]
pub enum ContextArg {
    SelfArg,
    Type(Box<Type>),
}
