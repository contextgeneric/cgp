use syn::Type;

pub enum ContextArg {
    SelfArg,
    Type(Type),
}
