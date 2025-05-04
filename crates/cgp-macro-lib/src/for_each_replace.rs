use alloc::vec::Vec;

use proc_macro2::{Group, TokenStream, TokenTree};
use quote::ToTokens;
use syn::__private::parse_brackets;
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Or};
use syn::{braced, Ident};

use crate::parse::{DelegateKey, SimpleType};

pub struct ReplaceSpecs {
    pub target_ident: Ident,
    pub replacements: Vec<TokenStream>,
    pub body: TokenStream,
}

impl Parse for ReplaceSpecs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let raw_replacements: Vec<DelegateKey<SimpleType>> = {
            let content = parse_brackets(input)?.content;
            let types = <Punctuated<DelegateKey<SimpleType>, Comma>>::parse_terminated(&content)?;
            types.into_iter().collect()
        };

        Comma::parse(input)?;

        let exclude: Vec<Ident> = {
            let fork = input.fork();

            match parse_brackets(&fork) {
                Ok(bracket) => {
                    let types = <Punctuated<Ident, Comma>>::parse_terminated(&bracket.content)?;

                    input.advance_to(&fork);
                    Comma::parse(input)?;

                    types.into_iter().collect()
                }
                _ => Vec::new(),
            }
        };

        Or::parse(input)?;

        let target_ident = Ident::parse(input)?;

        Or::parse(input)?;

        let body = {
            let content;
            braced!(content in input);
            TokenStream::parse(&content)?
        };

        let replacements = raw_replacements
            .into_iter()
            .filter(|replacement| {
                let target_ident = &replacement.ty.name;

                exclude.iter().all(|exclude| exclude != target_ident)
            })
            .map(|ast| ast.to_token_stream())
            .collect();

        Ok(ReplaceSpecs {
            target_ident,
            replacements,
            body,
        })
    }
}

pub fn replace_stream(
    target_ident: &Ident,
    replacement: &TokenStream,
    body: TokenStream,
) -> TokenStream {
    body.into_iter()
        .map(|tree| replace_tree(target_ident, replacement, tree))
        .collect()
}

pub fn replace_tree(
    target_ident: &Ident,
    replacement: &TokenStream,
    body: TokenTree,
) -> TokenStream {
    match body {
        TokenTree::Group(group) => TokenTree::Group(Group::new(
            group.delimiter(),
            replace_stream(target_ident, replacement, group.stream()),
        ))
        .into(),
        TokenTree::Ident(ident) => {
            if &ident == target_ident {
                replacement.to_token_stream()
            } else {
                TokenTree::Ident(ident).into()
            }
        }
        tokens => tokens.into(),
    }
}
