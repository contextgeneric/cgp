use proc_macro2::{Group, TokenStream, TokenTree};
use syn::parse::{Parse, ParseStream};
use syn::token::{Async, Await, Dot, Fn};

pub struct AsyncStripper {
    pub stream: TokenStream,
}

impl Parse for AsyncStripper {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut tokens: Vec<TokenTree> = Vec::new();

        while !input.is_empty() {
            if input.peek(Async) && input.peek2(Fn) {
                Async::parse(input)?;
            } else if input.peek(Dot) && input.peek2(Await) {
                Dot::parse(input)?;
                Await::parse(input)?;
            } else {
                let tree = TokenTree::parse(input)?;

                match tree {
                    TokenTree::Group(group) => {
                        let stripped: AsyncStripper = syn::parse2(group.stream())?;
                        let stripped_group = Group::new(group.delimiter(), stripped.stream);
                        tokens.push(TokenTree::Group(stripped_group));
                    }
                    TokenTree::Ident(i) => {
                        tokens.push(TokenTree::Ident(i));
                    }
                    TokenTree::Punct(p) => {
                        tokens.push(TokenTree::Punct(p));
                    }
                    TokenTree::Literal(l) => {
                        tokens.push(TokenTree::Literal(l));
                    }
                }
            }
        }

        Ok(AsyncStripper {
            stream: tokens.into_iter().collect(),
        })
    }
}

pub fn strip_async(stream: TokenStream) -> TokenStream {
    let stripped: AsyncStripper = syn::parse2(stream).unwrap();
    stripped.stream
}
