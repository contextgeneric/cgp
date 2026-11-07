use proc_macro2::Span;
use syn::Type;

use crate::types::check_components::TypeWithGenerics;

/// One check entry after evaluation.
pub struct EvaluatedCheckEntry {
    /// The component.
    pub key: Type,
    /// The component's parameters.
    pub value: Option<TypeWithGenerics>,
    /// The span the check is reported on.
    pub span: Span,
}
