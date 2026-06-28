use proc_macro2::{Literal, Span, TokenStream};
use quote::{ToTokens, quote_spanned};
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Type, parse_quote};

use crate::traits::ToType;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub ident: String,
    pub span: Span,
}

impl Symbol {
    pub fn from_ident(ident: Ident) -> Self {
        Self {
            ident: ident.to_string(),
            span: ident.span(),
        }
    }
}

impl Parse for Symbol {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let literal: LitStr = input.parse()?;
        Ok(Self {
            ident: literal.value(),
            span: literal.span(),
        })
    }
}

impl ToTokens for Symbol {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use crate::exports::{Chars, Nil, Symbol};

        let span = self.span;
        let str_value = self.ident.to_string();

        let chars = str_value.chars().rev().fold(
            quote_spanned!(span => #Nil),
            |acc, current| quote_spanned!(span => #Chars < #current, #acc >),
        );

        let len = Literal::usize_unsuffixed(str_value.len());

        let out = quote_spanned! { span => #Symbol < #len, #chars > };

        tokens.extend(out);
    }
}

impl ToType for Symbol {
    fn to_type(&self) -> Type {
        parse_quote!( #self )
    }
}
