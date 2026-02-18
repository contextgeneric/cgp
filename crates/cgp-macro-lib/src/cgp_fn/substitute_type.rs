use proc_macro2::{Group, TokenStream, TokenTree};
use quote::quote;

use crate::cgp_fn::UseTypeSpec;

pub fn substitute_abstract_type(
    context_type: &TokenStream,
    type_specs: &[UseTypeSpec],
    body: TokenStream,
) -> TokenStream {
    let mut out = TokenStream::new();
    let mut last_token_was_colon = false;
    let mut last_two_tokens_was_colon = false;

    for token_tree in body.into_iter() {
        let token_is_colon = if let TokenTree::Punct(punct) = &token_tree
            && punct.as_char() == ':'
        {
            true
        } else {
            false
        };

        match token_tree {
            TokenTree::Group(group) => {
                let new_stream = substitute_abstract_type(context_type, type_specs, group.stream());
                out.extend([TokenTree::Group(Group::new(group.delimiter(), new_stream))]);
            }
            TokenTree::Ident(ident) => {
                let mut replaced_ident = false;

                for type_spec in type_specs {
                    if !last_two_tokens_was_colon
                        && let Some(replacement_ident) = type_spec.replace_ident(&ident)
                    {
                        let trait_path = &type_spec.trait_path;

                        out.extend(quote! {
                            < #context_type as #trait_path > :: #replacement_ident
                        });

                        replaced_ident = true;
                        break;
                    }
                }

                if !replaced_ident {
                    out.extend([TokenTree::Ident(ident)]);
                }
            }
            TokenTree::Punct(punct) => {
                out.extend([TokenTree::Punct(punct)]);
            }
            TokenTree::Literal(literal) => {
                out.extend([TokenTree::Literal(literal)]);
            }
        }

        if token_is_colon {
            last_two_tokens_was_colon = last_token_was_colon;
            last_token_was_colon = true;
        } else {
            last_two_tokens_was_colon = false;
            last_token_was_colon = false;
        }
    }

    out
}
