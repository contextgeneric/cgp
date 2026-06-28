use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Error, Fields, LitInt, Type, parse2};

use crate::exports::{Cons, Field, Index, Nil};
use crate::types::field::Symbol;

pub fn item_fields_to_product_type(fields: &Fields, reference: &TokenStream) -> syn::Result<Type> {
    let mut fields_type = quote! { #Nil };

    match fields {
        Fields::Named(fields) => {
            for field in fields.named.iter().rev() {
                let field_name = field.ident.as_ref().ok_or_else(|| {
                    Error::new_spanned(field, "expect struct field to contain name identifier")
                })?;

                let field_tag = Symbol::from_ident(field_name.clone());
                let field_type = &field.ty;

                fields_type = parse2(quote! {
                    #Cons< #Field< #field_tag, #reference #field_type >, #fields_type >
                })?;
            }
        }
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() == 1 {
                let field_type = &fields.unnamed[0].ty;
                fields_type = parse2(quote! {
                    #reference #field_type
                })?;
            } else {
                for (i, field) in fields.unnamed.iter().enumerate().rev() {
                    let index = LitInt::new(&format!("{i}"), field.span());

                    let field_tag = quote! { #Index< #index > };
                    let field_type = &field.ty;

                    fields_type = parse2(quote! {
                        #Cons< #Field< #field_tag, #reference #field_type >, #fields_type >
                    })?;
                }
            }
        }
        Fields::Unit => {}
    }

    parse2(fields_type)
}
