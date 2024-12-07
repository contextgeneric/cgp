use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Expr, Type};

pub struct ParsePunctuated<T>(pub Punctuated<T, Comma>);

impl<T: Parse> Parse for ParsePunctuated<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let types = Punctuated::parse_terminated(input)?;
        Ok(ParsePunctuated(types))
    }
}

pub fn make_product_type(input: TokenStream) -> TokenStream {
    let types: ParsePunctuated<Type> = syn::parse2(input).unwrap();

    types.0.iter().rfold(quote! { Nil }, |res, item| {
        quote! {
            Cons< #item , #res >
        }
    })
}

pub fn make_sum_type(input: TokenStream) -> TokenStream {
    let types: ParsePunctuated<Type> = syn::parse2(input).unwrap();

    types.0.iter().rfold(quote! { Void }, |res, item| {
        quote! {
            Either< #item , #res >
        }
    })
}

pub fn make_product_expr(input: TokenStream) -> TokenStream {
    let types: ParsePunctuated<Expr> = syn::parse2(input).unwrap();

    types.0.iter().rfold(quote! { Nil }, |res, item| {
        quote! {
            Cons( #item , #res )
        }
    })
}
