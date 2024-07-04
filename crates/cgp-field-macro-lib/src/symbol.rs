use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_quote, LitStr, Type};

pub fn symbol_from_string(value: &str) -> Type {
    let char_types = <Punctuated<Type, Comma>>::from_iter(
        value
            .chars()
            .map(|c: char| -> Type { parse_quote!( Char< #c > ) }),
    );

    parse_quote!( ( #char_types ) )
}

pub fn make_symbol(input: TokenStream) -> TokenStream {
    let literal: LitStr = syn::parse2(input).unwrap();

    let symbol = symbol_from_string(&literal.value());

    symbol.to_token_stream()
}
