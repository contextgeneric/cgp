use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::LitInt;

use crate::exports::Index as Delta;

/// A tuple field's position as a type-level index.
pub struct Index {
    /// The position.
    pub index: usize,
    /// The source span.
    pub span: Span,
}

impl ToTokens for Index {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let index = LitInt::new(&self.index.to_string(), self.span);

        tokens.extend(quote! {
            #Delta<#index>
        });
    }
}
