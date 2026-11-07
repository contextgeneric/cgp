use quote::quote;
use syn::Type;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;

use crate::exports::{Either, Void};
use crate::functions::parse_internal;

/// The parsed `Sum!` list.
pub struct SumType {
    /// The alternatives.
    pub types: Punctuated<Type, Comma>,
}

impl SumType {
    /// Expands the list into nested `Either` types.
    pub fn eval(&self) -> syn::Result<Type> {
        let mut out = quote!(#Void);

        for ty in self.types.iter().rev() {
            out = quote! {
                #Either< #ty, #out >
            };
        }

        parse_internal(out)
    }
}

impl Parse for SumType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let types = Punctuated::parse_terminated(input)?;

        Ok(Self { types })
    }
}
