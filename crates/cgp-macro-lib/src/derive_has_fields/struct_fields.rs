use syn::{Error, Fields, ItemStruct, Type};

use crate::derive_has_fields::field_tag::FieldTag;

pub fn extract_struct_fields(item_struct: &ItemStruct) -> syn::Result<Vec<(FieldTag, Type)>> {
    let mut res = Vec::new();

    match &item_struct.fields {
        Fields::Named(fields) => {
            for field in fields.named.iter() {
                let field_name = field.ident.as_ref().ok_or_else(|| {
                    Error::new_spanned(field, "expect struct field to contain name identifier")
                })?;

                let field_tag = FieldTag::Named(field_name.to_string());

                res.push((field_tag, field.ty.clone()))
            }
        }
        Fields::Unnamed(fields) => {
            for (i, field) in fields.unnamed.iter().enumerate() {
                let field_tag = FieldTag::Indexed(i);

                res.push((field_tag, field.ty.clone()))
            }
        }
        Fields::Unit => {}
    }

    Ok(res)
}
