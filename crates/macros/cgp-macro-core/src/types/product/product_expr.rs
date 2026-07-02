use quote::quote;
use syn::Expr;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;

use crate::exports::{Cons, Nil};
use crate::functions::parse_internal;

/// The value-level `product!` macro: a list of expression items that folds into a
/// `Cons(..)`/`Nil` value expression. The type-level counterpart is `ProductType`.
pub struct ProductExpr {
    pub exprs: Punctuated<Expr, Comma>,
}

impl ProductExpr {
    /// Fold the items right-to-left onto `Nil` with the `Cons` tuple-struct
    /// constructor, yielding a value expression (not a type).
    pub fn eval(&self) -> syn::Result<Expr> {
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
