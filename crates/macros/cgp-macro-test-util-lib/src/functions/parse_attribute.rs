use cgp_macro_core::traits::IsKeyword;
use cgp_macro_core::types::keyword::Keyword;
use proc_macro2::TokenStream;
use syn::parse::ParseStream;
use syn::token::Paren;
use syn::{Error, Ident, braced, bracketed, parenthesized};

pub fn parse_attribute(expected_keyword: &str, input: ParseStream) -> syn::Result<TokenStream> {
    let outer_body;
    bracketed!(outer_body in input);

    let keyword: Ident = outer_body.parse()?;
    if keyword != expected_keyword {
        return Err(Error::new(
            keyword.span(),
            format!("expect attribute with keyword {expected_keyword}, but got {keyword}"),
        ));
    }

    let body = if outer_body.is_empty() {
        TokenStream::new()
    } else if outer_body.peek(Paren) {
        let inner_body;
        parenthesized!(inner_body in outer_body);
        inner_body.parse()?
    } else {
        let inner_body;
        braced!(inner_body in outer_body);
        inner_body.parse()?
    };

    Ok(body)
}

pub fn parse_attribute_with_keyword<K: IsKeyword>(input: ParseStream) -> syn::Result<TokenStream> {
    let outer_body;
    bracketed!(outer_body in input);

    let _: Keyword<K> = outer_body.parse()?;

    let body = if outer_body.is_empty() {
        TokenStream::new()
    } else if outer_body.peek(Paren) {
        let inner_body;
        parenthesized!(inner_body in outer_body);
        inner_body.parse()?
    } else {
        let inner_body;
        braced!(inner_body in outer_body);
        inner_body.parse()?
    };

    Ok(body)
}
