use proc_macro2::{Literal, Span, TokenStream};
use quote::quote_spanned;
use syn::{LitStr, Type, parse2};

pub fn symbol_from_string(value: &str) -> syn::Result<Type> {
    parse2(symbol_from_string_spanned(Span::call_site(), value))
}

pub fn symbol_from_string_spanned(span: Span, value: &str) -> TokenStream {
    let mut chars = quote_spanned! { span => ε };

    for c in value.chars().rev() {
        chars = quote_spanned! { span => ζ< #c, #chars > };
    }

    let len = Literal::usize_unsuffixed(value.len());

    quote_spanned! { span => ψ< #len, #chars > }
}

pub fn make_symbol(input: TokenStream) -> syn::Result<TokenStream> {
    let literal: LitStr = syn::parse2(input)?;

    let symbol = symbol_from_string_spanned(literal.span(), &literal.value());

    Ok(symbol)
}
