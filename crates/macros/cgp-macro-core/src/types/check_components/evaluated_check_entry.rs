use proc_macro2::Span;
use syn::Type;

use crate::types::generics::ImplGenerics;

pub struct EvaluatedCheckEntry {
    pub component_type: Type,
    pub component_params: Option<Type>,
    pub span: Span,
    pub generics: ImplGenerics,
}
