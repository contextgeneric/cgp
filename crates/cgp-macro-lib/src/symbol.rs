use proc_macro2::{Literal, TokenStream};
use quote::{ToTokens, quote};
use syn::{LitStr, Type, parse_quote, parse2};

pub fn symbol_from_string(value: &str) -> syn::Result<Type> {
    let chars = value
        .chars()
        .rfold(parse_quote! { ε }, |tail, c: char| -> Type {
            parse_quote!( ζ< #c, #tail > )
        });

    let len = Literal::usize_unsuffixed(value.len());

    parse2(quote! { ψ< #len, #chars > })
}

pub fn make_symbol(input: TokenStream) -> syn::Result<TokenStream> {
    let literal: LitStr = syn::parse2(input)?;

    let symbol = symbol_from_string(&literal.value())?;

    Ok(symbol.to_token_stream())
}
