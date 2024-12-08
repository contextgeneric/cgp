use proc_macro2::{Group, TokenStream, TokenTree};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Or};
use syn::{braced, bracketed, Ident, Type};

pub struct ReplaceSpecs {
    pub target_ident: Ident,
    pub replacements: Vec<Type>,
    pub body: TokenStream,
}

impl Parse for ReplaceSpecs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let raw_replacements: Vec<Type> = {
            let content;
            bracketed!(content in input);

            let types = <Punctuated<Type, Comma>>::parse_terminated(&content)?;
            types.into_iter().collect()
        };

        Comma::parse(input)?;

        let exclude: Vec<Type> = {
            let content;
            bracketed!(content in input);

            let types = <Punctuated<Type, Comma>>::parse_terminated(&content)?;
            types.into_iter().collect()
        };

        Comma::parse(input)?;

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
            .filter(|replacement| !exclude.iter().any(|exclude| exclude == replacement))
            .collect();

        Ok(ReplaceSpecs {
            target_ident,
            replacements,
            body,
        })
    }
}

pub fn handle_for_each_replace(tokens: TokenStream) -> syn::Result<TokenStream> {
    let specs: ReplaceSpecs = syn::parse2(tokens)?;

    Ok(for_each_replace(
        &specs.target_ident,
        &specs.replacements,
        &specs.body,
    ))
}

pub fn for_each_replace(
    target_ident: &Ident,
    replacements: &[Type],
    body: &TokenStream,
) -> TokenStream {
    replacements
        .iter()
        .map(|replacement| replace_stream(target_ident, replacement, body.clone()))
        .collect()
}

pub fn replace_stream(target_ident: &Ident, replacement: &Type, body: TokenStream) -> TokenStream {
    body.into_iter()
        .map(|tree| replace_tree(target_ident, replacement, tree))
        .collect()
}

pub fn replace_tree(target_ident: &Ident, replacement: &Type, body: TokenTree) -> TokenStream {
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
