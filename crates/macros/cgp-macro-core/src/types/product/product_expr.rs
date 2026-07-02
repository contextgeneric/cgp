use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Expr, Type};

use crate::exports::{Cons, Nil};
use crate::functions::parse_internal;

pub struct ProductExpr {
    pub exprs: Punctuated<Expr, Comma>,
}

impl ProductExpr {
    pub fn eval(&self) -> syn::Result<Type> {
        let mut out = quote!(#Nil);

        for expr in self.exprs.iter().rev() {
            out = quote! {
                #Cons(#expr, #out)
            };
        }

        parse_internal(out)
    }
}

impl Parse for ProductExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let exprs = Punctuated::parse_terminated(input)?;

        Ok(Self { exprs })
    }
}
