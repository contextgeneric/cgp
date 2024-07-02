use itertools::Itertools;
use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::{format_ident, ToTokens};
use syn::parse::Parse;

pub fn iter_parse_and_replace_self_type<I, T>(
    vals: I,
    replaced_ident: &Ident,
    local_assoc_types: &Vec<Ident>,
) -> syn::Result<I>
where
    I: IntoIterator<Item = T> + FromIterator<T>,
    T: ToTokens + Parse,
{
    vals.into_iter()
        .map(|val| parse_and_replace_self_type(&val, replaced_ident, local_assoc_types))
        .collect()
}

pub fn parse_and_replace_self_type<T>(
    val: &T,
    replaced_ident: &Ident,
    local_assoc_types: &Vec<Ident>,
) -> syn::Result<T>
where
    T: ToTokens + Parse,
{
    let stream = replace_self_type(val.to_token_stream(), replaced_ident, local_assoc_types);
    syn::parse2(stream)
}

pub fn replace_self_type(
    stream: TokenStream,
    replaced_ident: &Ident,
    local_assoc_types: &Vec<Ident>,
) -> TokenStream {
    let self_type = format_ident!("Self");

    let mut result_stream: Vec<TokenTree> = Vec::new();

    let mut token_iter = stream.into_iter().multipeek();

    while let Some(tree) = token_iter.next() {
        match tree {
            TokenTree::Ident(ident) => {
                if ident == self_type {
                    let replaced_ident = replaced_ident.clone();

                    // Do not replace self if it is an associated type expression that refers to local associated type
                    let replaced = match token_iter.peek() {
                        Some(TokenTree::Punct(p)) if p.as_char() == ':' => {
                            match token_iter.peek() {
                                Some(TokenTree::Punct(p)) if p.as_char() == ':' => {
                                    match token_iter.peek() {
                                        Some(TokenTree::Ident(assoc_type))
                                            if local_assoc_types.contains(assoc_type) =>
                                        {
                                            ident
                                        }
                                        _ => replaced_ident,
                                    }
                                }
                                _ => replaced_ident,
                            }
                        }
                        _ => replaced_ident,
                    };

                    result_stream.push(TokenTree::Ident(replaced));
                } else {
                    result_stream.push(TokenTree::Ident(ident));
                }
            }
            TokenTree::Group(group) => {
                let replaced_stream =
                    replace_self_type(group.stream(), replaced_ident, local_assoc_types);
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
