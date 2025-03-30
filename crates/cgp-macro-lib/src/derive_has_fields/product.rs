use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Error, Fields, Type};

use crate::symbol::symbol_from_string;

pub fn item_fields_to_product_type(fields: &Fields, reference: &TokenStream) -> syn::Result<Type> {
    let mut fields_type = quote! { Nil };

    match fields {
        Fields::Named(fields) => {
            for field in fields.named.iter().rev() {
                let field_name = field.ident.as_ref().ok_or_else(|| {
                    Error::new_spanned(field, "expect struct field to contain name identifier")
                })?;

                let field_tag = symbol_from_string(&field_name.to_string());
                let field_type = &field.ty;

                fields_type = parse2(quote! {
                    Cons< Field< #field_tag, #reference #field_type >, #fields_type >
                })?;
            }
        }
        Fields::Unnamed(fields) => {
            for (i, field) in fields.unnamed.iter().enumerate().rev() {
                let field_tag = quote! { Index< #i > };
                let field_type = &field.ty;

                fields_type = parse2(quote! {
                    Cons< Field< #field_tag, #field_type >, #fields_type >
                })?;
            }
        }
        Fields::Unit => {}
    }

    parse2(fields_type)
}
