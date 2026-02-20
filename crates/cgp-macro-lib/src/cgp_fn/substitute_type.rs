use proc_macro2::{Group, TokenStream, TokenTree};
use quote::quote;

use crate::cgp_fn::UseTypeSpec;

pub fn substitute_abstract_types(type_specs: &[UseTypeSpec], body: TokenStream) -> TokenStream {
    let mut out = body;
    for spec in type_specs.iter().rev() {
        out = substitute_abstract_type(spec, out);
    }
    out
}

pub fn substitute_abstract_type(type_specs: &UseTypeSpec, body: TokenStream) -> TokenStream {
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
                let new_stream = substitute_abstract_type(type_specs, group.stream());
                out.extend([TokenTree::Group(Group::new(group.delimiter(), new_stream))]);
            }
            TokenTree::Ident(ident) => {
                if !last_two_tokens_was_colon
                    && let Some(replacement_ident) = type_specs.replace_ident(&ident)
                {
                    let trait_path = &type_specs.trait_path;
                    let context_type = &type_specs.context_type;

                    out.extend(quote! {
                        < #context_type as #trait_path > :: #replacement_ident
                    });
                } else {
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
