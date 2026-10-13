use syn::Attribute;

/// Keep only the attributes a generated companion type can carry from a field or variant: `#[cfg]`,
/// so the companion has the same members as the input, and documentation. Every other attribute,
/// such as another derive's helper (`#[default]`, `#[serde(…)]`), belongs to a derive the companion
/// does not have and would fail to resolve.
pub fn retain_companion_attrs(attrs: &mut Vec<Attribute>) {
    attrs.retain(|attr| attr.path().is_ident("cfg") || attr.path().is_ident("doc"));
}
