use proc_macro2::TokenStream;
use quote::quote;
use syn::token::Mut;

use crate::exports::{MRef, OptionAsRef};

/// The conversion a getter body applies to a raw field read, chosen from the
/// getter method's return type by [`parse_field_type`](crate::functions::parse_field_type).
#[derive(Clone, Eq, PartialEq)]
pub enum FieldMode {
    /// Return the field by reference.
    Reference,
    /// Return an `Option` field by reference.
    OptionRef,
    /// Return an `Option` field as an `Option<&str>`.
    OptionStr,
    /// Return the field as an [`MRef`].
    MRef,
    /// Return the field as a `&str`.
    Str,
    /// Return a copy of the field.
    Copy,
    /// Return the field as a slice.
    Slice,
}

impl FieldMode {
    /// Wrap a field-read expression in the conversion this mode calls for, picking
    /// the shared or mutable variant from `field_mut`. This is the single source of
    /// truth shared by the getter-method body and the `#[implicit]` argument
    /// binding, so both families convert a given field identically.
    pub fn apply(&self, call_expr: TokenStream, field_mut: &Option<Mut>) -> TokenStream {
        // Every conversion is a fully qualified trait call, so an inherent method of
        // the same name on the field's type cannot capture it.
        match self {
            FieldMode::Reference => call_expr,
            FieldMode::OptionRef => {
                // The field is an `Option<T>`, so the read is `Option`'s own inherent
                // method, not the `AsRef`/`AsMut` conversion.
                if field_mut.is_none() {
                    quote! { ::core::option::Option::as_ref( #call_expr ) }
                } else {
                    quote! { ::core::option::Option::as_mut( #call_expr ) }
                }
            }
            FieldMode::OptionStr => {
                if field_mut.is_none() {
                    quote! { #OptionAsRef::option_as_ref( #call_expr ) }
                } else {
                    quote! { ::core::option::Option::as_deref_mut( #call_expr ) }
                }
            }
            FieldMode::MRef => quote! { #MRef::Ref( #call_expr ) },
            FieldMode::Str => {
                if field_mut.is_none() {
                    quote! { ::core::convert::AsRef::as_ref( #call_expr ) }
                } else {
                    quote! { ::core::convert::AsMut::as_mut( #call_expr ) }
                }
            }
            FieldMode::Copy => quote! { ::core::clone::Clone::clone( #call_expr ) },
            FieldMode::Slice => {
                if field_mut.is_none() {
                    quote! { ::core::convert::AsRef::as_ref( #call_expr ) }
                } else {
                    quote! { ::core::convert::AsMut::as_mut( #call_expr ) }
                }
            }
        }
    }
}
