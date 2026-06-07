use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Expr;
use syn::token::Mut;

use crate::types::field::FieldName;

pub struct GetFieldExpr {
    pub receiver: Expr,
    pub field_mut: Option<Mut>,
    pub field_name: FieldName,
}

impl ToTokens for GetFieldExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let receiver = &self.receiver;
        let field_name = &self.field_name;

        let method = if self.field_mut.is_none() {
            quote!(get_field)
        } else {
            quote!(get_field_mut)
        };

        tokens.extend(quote! {
            #receiver.#method(
                ::core::marker::PhantomData::<#field_name>
            )
        })
    }
}
