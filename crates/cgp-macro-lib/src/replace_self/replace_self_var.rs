use proc_macro2::{Group, TokenStream, TokenTree};
use quote::format_ident;
use syn::Ident;

pub fn replace_self_var(stream: TokenStream, replaced_ident: &Ident) -> TokenStream {
    let self_ident = format_ident!("self");

    let mut result_stream: Vec<TokenTree> = Vec::new();

    let token_iter = stream.into_iter();

    for tree in token_iter {
        match tree {
            TokenTree::Ident(ident) => {
                if ident == self_ident {
                    result_stream.push(TokenTree::Ident(replaced_ident.clone()));
                } else {
                    result_stream.push(TokenTree::Ident(ident));
                }
            }
            TokenTree::Group(group) => {
                let replaced_stream = replace_self_var(group.stream(), replaced_ident);
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
