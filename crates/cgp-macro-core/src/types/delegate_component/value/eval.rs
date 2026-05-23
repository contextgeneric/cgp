use syn::Type;

pub trait EvalDelegateValue {
    fn eval(&self) -> syn::Result<Type>;
}
