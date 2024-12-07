use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Type};
use syn::{Error, Expr, Ident};

pub enum Product {
    Types(Punctuated<Type, Comma>),
    Idents(Punctuated<Ident, Comma>),
    Exprs(Punctuated<Expr, Comma>),
}

impl Parse for Product {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Ok(types) = <Punctuated<Type, Comma>>::parse_terminated(&input.fork()) {
            Ok(Product::Types(types))
        } else if let Ok(idents) = <Punctuated<Ident, Comma>>::parse_terminated(&input.fork()) {
            Ok(Product::Idents(idents))
        } else if let Ok(exprs) = <Punctuated<Expr, Comma>>::parse_terminated(&input.fork()) {
            Ok(Product::Exprs(exprs))
        } else {
            Err(Error::new(input.span(), "product macro can only be used on comma-separated list of types, identifiers, or expressions"))
        }
    }
}

impl ToTokens for Product {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Types(types) => types.to_tokens(tokens),
            Self::Idents(idents) => idents.to_tokens(tokens),
            Self::Exprs(exprs) => exprs.to_tokens(tokens),
        }
    }
}

pub fn build_product<T: ToTokens>(items: Punctuated<T, Comma>) -> TokenStream {
    items.iter().rfold(TokenStream::new(), |res, item| {
        quote! {
            Cons< #item , #res >
        }
    })
}

pub fn parse_product(input: TokenStream) -> TokenStream {
    let product: Product = syn::parse2(input).unwrap();

    match product {
        Product::Types(types) => build_product(types),
        Product::Idents(idents) => build_product(idents),
        Product::Exprs(exprs) => build_product(exprs),
    }
}
