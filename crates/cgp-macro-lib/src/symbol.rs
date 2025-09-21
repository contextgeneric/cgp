use proc_macro2::{Literal, TokenStream};
use quote::ToTokens;
use syn::{LitStr, Type, parse_quote};

pub fn symbol_from_string(value: &str) -> Type {
    let chars = value
        .chars()
        .rfold(parse_quote! { ε }, |tail, c: char| -> Type {
            parse_quote!( ζ< #c, #tail > )
        });

    let len = Literal::usize_unsuffixed(value.len());

    parse_quote!( ψ< #len, #chars > )
}

pub fn make_symbol(input: TokenStream) -> TokenStream {
    let literal: LitStr = syn::parse2(input).unwrap();

    let symbol = symbol_from_string(&literal.value());

    symbol.to_token_stream()
}
