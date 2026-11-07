use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Expr;
use syn::token::Mut;

use crate::exports::{HasField, HasFieldMut};
use crate::types::field::FieldName;

/// A fully qualified field read.
pub struct GetFieldExpr {
    /// The receiver.
    pub receiver: Expr,
    /// The `mut` of a mutable read.
    pub field_mut: Option<Mut>,
    /// The field's name.
    pub field_name: FieldName,
}

impl ToTokens for GetFieldExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let receiver = &self.receiver;
        let field_name = &self.field_name;

        // A fully qualified trait call, so an inherent method of the same name on the
        // context cannot capture the read.
        let (field_trait, method) = if self.field_mut.is_none() {
            (quote!(#HasField), quote!(get_field))
        } else {
            (quote!(#HasFieldMut), quote!(get_field_mut))
        };

        tokens.extend(quote! {
            #field_trait::#method(
                #receiver,
                ::core::marker::PhantomData::<#field_name>
            )
        })
    }
}
