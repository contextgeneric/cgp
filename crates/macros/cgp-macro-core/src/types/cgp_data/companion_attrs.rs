use syn::Attribute;

/// Keep only the documentation of a field or variant on a generated companion type. Every other
/// attribute, such as another derive's helper (`#[default]`, `#[serde(…)]`), belongs to a derive
/// the companion does not have and would fail to resolve. A `#[cfg]` or `#[cfg_attr]` never
/// reaches here: the compiler evaluates both before a derive sees its input.
pub fn retain_companion_attrs(attrs: &mut Vec<Attribute>) {
    attrs.retain(|attr| attr.path().is_ident("doc"));
}
