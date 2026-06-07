use itertools::Itertools;
use proc_macro2::{Group, TokenStream, TokenTree};
use quote::format_ident;
use syn::Ident;

pub fn remove_self_path(stream: TokenStream, assoc_idents: &Vec<Ident>) -> TokenStream {
    let self_type = format_ident!("Self");

    let mut result_stream: Vec<TokenTree> = Vec::new();

    let mut token_iter = stream.into_iter().multipeek();

    while let Some(tree) = token_iter.next() {
        match tree {
            TokenTree::Ident(ident) => {
                if ident == self_type {
                    let m_colon_1 = token_iter.peek().cloned();
                    let m_colon_2 = token_iter.peek().cloned();
                    let assoc_ident = token_iter.peek().cloned();

                    match (m_colon_1, m_colon_2, assoc_ident) {
                        (
                            Some(TokenTree::Punct(colon_1)),
                            Some(TokenTree::Punct(colon_2)),
                            Some(TokenTree::Ident(assoc_ident)),
                        ) if colon_1.as_char() == ':'
                            && colon_2.as_char() == ':'
                            && assoc_idents.contains(&assoc_ident) =>
                        {
                            token_iter.next();
                            token_iter.next();
                            token_iter.next();

                            result_stream.push(TokenTree::Ident(assoc_ident));
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
                let replaced_stream = remove_self_path(group.stream(), assoc_idents);
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
