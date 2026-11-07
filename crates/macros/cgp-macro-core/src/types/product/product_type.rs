use quote::quote;
use syn::Type;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;

use crate::exports::{Cons, Nil};
use crate::functions::parse_internal;

/// The parsed `Product!` list.
pub struct ProductType {
    /// The entries.
    pub types: Punctuated<Type, Comma>,
}

impl ProductType {
    /// Expands the list into nested `Cons` types.
    pub fn eval(&self) -> syn::Result<Type> {
        let mut out = quote!(#Nil);

        for ty in self.types.iter().rev() {
            out = quote! {
                #Cons< #ty, #out >
            };
        }

        parse_internal(out)
    }
}

impl Parse for ProductType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let types = Punctuated::parse_terminated(input)?;

        Ok(Self { types })
    }
}
