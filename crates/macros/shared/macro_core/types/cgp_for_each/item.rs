use proc_macro2::{Group, TokenStream, TokenTree};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Ident, Type, braced, bracketed};

use crate::macro_core::traits::PeekKeyword;
use crate::macro_core::types::keyword::Keyword;
use crate::macro_core::types::keywords::Exclude;

/// `cgp_for_each! { [A, B], exclude [A], Name => { body } }` repeats `body` once
/// per component, substituting `Name` and dropping anything in `exclude`.
pub struct CgpForEach {
    pub components: Vec<Type>,
    pub exclude: Vec<Type>,
    pub placeholder: Ident,
    pub body: TokenStream,
}

impl Parse for CgpForEach {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let components = {
            let content;
            bracketed!(content in input);
            Punctuated::<Type, Comma>::parse_terminated(&content)?
                .into_iter()
                .collect()
        };

        let _: Comma = input.parse()?;

        let exclude = if input.peek_keyword::<Exclude>() {
            let _: Keyword<Exclude> = input.parse()?;
            let content;
            bracketed!(content in input);
            let exclude = Punctuated::<Type, Comma>::parse_terminated(&content)?
                .into_iter()
                .collect();
            let _: Comma = input.parse()?;
            exclude
        } else {
            Vec::new()
        };

        let placeholder = input.parse()?;
        let _: syn::Token![=>] = input.parse()?;

        let body = {
            let content;
            braced!(content in input);
            content.parse()?
        };

        Ok(Self {
            components,
            exclude,
            placeholder,
            body,
        })
    }
}

impl CgpForEach {
    pub fn expand(&self) -> TokenStream {
        let excluded: Vec<String> = self
            .exclude
            .iter()
            .map(|ty| ty.to_token_stream().to_string())
            .collect();

        let mut out = TokenStream::new();

        for component in &self.components {
            if excluded.contains(&component.to_token_stream().to_string()) {
                continue;
            }

            out.extend(replace_ident(
                &self.placeholder,
                component.to_token_stream(),
                self.body.clone(),
            ));
        }

        out
    }
}

/// Substitute `placeholder` with `replacement`, keeping every other token's span
/// so a component name written in a preset still resolves where the preset was
/// defined.
fn replace_ident(placeholder: &Ident, replacement: TokenStream, body: TokenStream) -> TokenStream {
    body.into_iter()
        .map(|tree| match tree {
            TokenTree::Group(group) => {
                let inner = replace_ident(placeholder, replacement.clone(), group.stream());
                TokenTree::Group(Group::new(group.delimiter(), inner)).into()
            }
            TokenTree::Ident(ident) if ident == *placeholder => replacement.clone(),
            other => other.into(),
        })
        .collect()
}
