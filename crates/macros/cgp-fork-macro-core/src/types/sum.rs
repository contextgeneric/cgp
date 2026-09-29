use quote::quote;
use syn::Type;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;

use crate::exports::{Either, Void};
use crate::functions::parse_internal;

pub struct SumType {
    pub types: Punctuated<Type, Comma>,
}

impl SumType {
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
