use proc_macro2::TokenStream;
use quote::quote;
use syn::token::Mut;

use crate::exports::MRef;

/// The conversion a getter body applies to a raw field read, chosen from the
/// getter method's return type by [`parse_field_type`](crate::functions::parse_field_type).
#[derive(Clone, Eq, PartialEq)]
pub enum FieldMode {
    Reference,
    OptionRef,
    OptionStr,
    MRef,
    Str,
    Copy,
    Slice,
}

impl FieldMode {
    /// Wrap a field-read expression in the conversion this mode calls for, picking
    /// the shared or mutable variant from `field_mut`. This is the single source of
    /// truth shared by the getter-method body and the `#[implicit]` argument
    /// binding, so both families convert a given field identically.
    pub fn apply(&self, call_expr: TokenStream, field_mut: &Option<Mut>) -> TokenStream {
        match self {
            FieldMode::Reference => call_expr,
            FieldMode::OptionRef => {
                if field_mut.is_none() {
                    quote! { #call_expr .as_ref() }
                } else {
                    quote! { #call_expr .as_mut() }
                }
            }
            FieldMode::OptionStr => {
                if field_mut.is_none() {
                    quote! { #call_expr .as_deref() }
                } else {
                    quote! { #call_expr .as_deref_mut() }
                }
            }
            FieldMode::MRef => quote! { #MRef::Ref( #call_expr ) },
            FieldMode::Str => {
                if field_mut.is_none() {
                    quote! { #call_expr .as_str() }
                } else {
                    quote! { #call_expr .as_mut_str() }
                }
            }
            FieldMode::Copy => quote! { #call_expr .clone() },
            FieldMode::Slice => {
                if field_mut.is_none() {
                    quote! { #call_expr .as_ref() }
                } else {
                    quote! { #call_expr .as_mut() }
                }
            }
        }
    }
}
