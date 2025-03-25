use itertools::Itertools;
use proc_macro2::{Group, TokenStream, TokenTree};
use quote::format_ident;

pub fn remove_self_path(stream: TokenStream) -> TokenStream {
    let self_type = format_ident!("Self");

    let mut result_stream: Vec<TokenTree> = Vec::new();

    let mut token_iter = stream.into_iter().multipeek();

    while let Some(tree) = token_iter.next() {
        match tree {
            TokenTree::Ident(ident) => {
                if &ident == &self_type {
                    let m_colon_1 = token_iter.peek().cloned();
                    let m_colon_2 = token_iter.peek();

                    match (m_colon_1, m_colon_2) {
                        (Some(TokenTree::Punct(colon_1)), Some(TokenTree::Punct(colon_2)))
                            if colon_1.as_char() == ':' && colon_2.as_char() == ':' =>
                        {
                            token_iter.next();
                            token_iter.next();
                        }
                        _ => {
                            result_stream.push(TokenTree::Ident(ident));
                        }
                    }
                } else {
                    result_stream.push(TokenTree::Ident(ident));
                }
            }
            TokenTree::Group(group) => {
                let replaced_stream = remove_self_path(group.stream());
                let replaced_group = Group::new(group.delimiter(), replaced_stream);

                result_stream.push(TokenTree::Group(replaced_group));
            }
            TokenTree::Punct(punct) => {
                result_stream.push(TokenTree::Punct(punct));
            }
            TokenTree::Literal(lit) => result_stream.push(TokenTree::Literal(lit)),
        }
    }

    result_stream.into_iter().collect()
}
