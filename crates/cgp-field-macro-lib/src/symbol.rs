use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, LitStr, Type};

pub fn symbol_from_string(value: &str) -> Type {
    value
        .chars()
        .rfold(parse_quote! { Nil }, |tail, c: char| -> Type {
            parse_quote!( Cons< Char< #c >, #tail > )
        })
}

pub fn make_symbol(input: TokenStream) -> TokenStream {
    let literal: LitStr = syn::parse2(input).unwrap();

    let symbol = symbol_from_string(&literal.value());

    symbol.to_token_stream()
}
