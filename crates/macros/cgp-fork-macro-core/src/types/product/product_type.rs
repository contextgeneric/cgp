use quote::quote;
use syn::Type;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;

use crate::exports::{Cons, Nil};
use crate::functions::parse_internal;

pub struct ProductType {
    pub types: Punctuated<Type, Comma>,
}

impl ProductType {
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
