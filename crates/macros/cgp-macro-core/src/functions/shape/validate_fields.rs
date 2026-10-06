use syn::ext::IdentExt;
use syn::{Error, Fields, Visibility};

use crate::functions::reject_non_empty_attributes;

/// Reject the parts of a struct or variant body that a type-level shape has no use for: field
/// attributes, field visibility, the `_` field name, and a field name given twice.
///
/// `syn` parses all of these as part of an ordinary struct body, so a `Struct!` or `Enum!` body
/// gets through its parser and must be checked here. Names are compared with any `r#` removed,
/// since `r#a` and `a` produce the same `Symbol!("a")` tag.
pub fn validate_shape_fields(fields: &Fields) -> syn::Result<()> {
    let mut seen = Vec::new();

    for field in fields {
        reject_non_empty_attributes(&field.attrs)?;

        if !matches!(field.vis, Visibility::Inherited) {
            return Err(Error::new_spanned(
                &field.vis,
                "a field of a type-level shape has no visibility; remove it",
            ));
        }

        if let Some(ident) = &field.ident {
            let name = ident.unraw().to_string();

            if name == "_" {
                return Err(Error::new(
                    ident.span(),
                    "a field of a type-level shape must be named; `_` is not allowed",
                ));
            }

            if seen.contains(&name) {
                return Err(Error::new(
                    ident.span(),
                    format!("duplicate field `{name}`"),
                ));
            }

            seen.push(name);
        }
    }

    Ok(())
}
