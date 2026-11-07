use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{Error, Fields, Ident, ItemImpl, ItemStruct, LitInt};

use crate::exports::{Cons, Nil, ToFields};
use crate::parse_internal;

/// Derives the `ToFields` impl for a struct.
pub fn derive_to_fields_for_struct(item_struct: &ItemStruct) -> syn::Result<ItemImpl> {
    let struct_name = &item_struct.ident;
    let (impl_generics, type_generics, where_clause) = item_struct.generics.split_for_impl();

    let constructor =
        derive_to_fields_constructor(&item_struct.fields, |field_name| match field_name {
            FieldLabel::None => quote! {
                self.0
            },
            _ => quote! {
                ::core::convert::Into::into(self #field_name)
            },
        })?;

    let item_impl = parse_internal! {
        impl #impl_generics
            #ToFields for #struct_name #type_generics
        #where_clause
        {
            fn to_fields(
                self,
            ) -> Self::Fields {
                #constructor
            }
        }
    };

    Ok(item_impl)
}

/// A field's label in a struct's shape.
pub enum FieldLabel {
    /// A named field.
    Named(Ident),
    /// A tuple field.
    Unnamed(LitInt),
    /// A single-field tuple struct.
    None,
}

impl ToTokens for FieldLabel {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Named(label) => tokens.extend(quote! { . #label }),
            Self::Unnamed(label) => tokens.extend(quote! { . #label }),
            Self::None => {}
        }
    }
}

/// Builds the constructor that turns a struct into its field list.
pub fn derive_to_fields_constructor(
    fields: &Fields,
    construct_field: impl Fn(FieldLabel) -> TokenStream,
) -> syn::Result<TokenStream> {
    let mut constructors = quote! { #Nil };

    match &fields {
        Fields::Named(fields) => {
            for field in fields.named.iter().rev() {
                let field_name = field.ident.as_ref().cloned().ok_or_else(|| {
                    Error::new_spanned(field, "expect struct field to contain name identifier")
                })?;

                let constructor = construct_field(FieldLabel::Named(field_name));

                constructors = quote! {
                    #Cons(
                        #constructor,
                        #constructors
                    )
                };
            }
        }
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() == 1 {
                constructors = construct_field(FieldLabel::None);
            } else {
                for (i, field) in fields.unnamed.iter().enumerate().rev() {
                    let field_name = LitInt::new(&format!("{i}"), field.span());

                    let constructor = construct_field(FieldLabel::Unnamed(field_name));

                    constructors = quote! {
                        #Cons(
                            #constructor,
                            #constructors
                        )
                    };
                }
            }
        }
        Fields::Unit => {}
    };

    Ok(constructors)
}
