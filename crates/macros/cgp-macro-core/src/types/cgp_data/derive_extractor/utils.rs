use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Field, FieldMutability, Fields, FieldsUnnamed, Type, Variant, Visibility};

/// Return a newtype variant's single payload type, or a spanned error if the
/// variant is not exactly one unnamed field — the requirement the variant
/// derives enforce.
pub fn get_variant_type(variant: &Variant) -> syn::Result<&Type> {
    match &variant.fields {
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            if let Some(field) = fields.unnamed.first() {
                return Ok(&field.ty);
            }
        }
        _ => {}
    }

    Err(syn::Error::new(
        variant.span(),
        "Expected variant to contain exactly one unnamed field",
    ))
}

/// Wrap a type as a single-unnamed-field variant body, used to re-shape a
/// partial enum's variants around their `MapType`-wrapped payloads.
pub fn type_to_variant_fields(type_: &Type) -> Fields {
    Fields::Unnamed(FieldsUnnamed {
        unnamed: Punctuated::from_iter([Field {
            attrs: Vec::new(),
            ident: None,
            vis: Visibility::Inherited,
            ty: type_.clone(),
            colon_token: None,
            mutability: FieldMutability::None,
        }]),
        paren_token: Default::default(),
    })
}
