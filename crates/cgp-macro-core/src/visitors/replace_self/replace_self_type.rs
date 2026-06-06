use itertools::Itertools;
use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::{ToTokens, format_ident};
use syn::visit_mut::{self, VisitMut};
use syn::{Macro, Path, Type};

pub struct ReplaceSelfTypeVisitor<'a> {
    pub replaced_type: &'a Type,
    pub skip_assoc_types: &'a Vec<Ident>,
}

impl<'a> ReplaceSelfTypeVisitor<'a> {
    fn replace_self_in_path(&self, path: &mut Path) {
        let Some(first) = path.segments.first() else {
            return;
        };
        if first.ident != "Self" {
            return;
        }
        if path.segments.len() >= 2 && self.skip_assoc_types.contains(&path.segments[1].ident) {
            return;
        }
        if let Type::Path(replaced) = self.replaced_type
            && replaced.qself.is_none()
        {
            let rest: Vec<_> = path.segments.iter().skip(1).cloned().collect();
            let mut new_path = replaced.path.clone();
            new_path.segments.extend(rest);
            *path = new_path;
        }
    }
}

impl VisitMut for ReplaceSelfTypeVisitor<'_> {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        // Handle standalone `Self` type — replaced_type may not be a path (e.g. a reference),
        // so we must replace the whole Type node here rather than going through visit_path_mut.
        if let Type::Path(type_path) = ty
            && type_path.qself.is_none()
            && type_path.path.segments.len() == 1
            && type_path.path.segments[0].ident == "Self"
        {
            *ty = self.replaced_type.clone();
            return;
        }
        visit_mut::visit_type_mut(self, ty);
    }

    fn visit_path_mut(&mut self, path: &mut Path) {
        // Handles Self::Foo in type paths (multi-segment) and Self in expression/struct paths.
        // Single-segment Self in type position is already handled by visit_type_mut above.
        self.replace_self_in_path(path);
        visit_mut::visit_path_mut(self, path);
    }

    fn visit_macro_mut(&mut self, mac: &mut Macro) {
        mac.tokens = replace_self_type_in_token_stream(
            core::mem::take(&mut mac.tokens),
            self.replaced_type.to_token_stream(),
            self.skip_assoc_types,
        );
    }
}

pub fn replace_self_type_in_token_stream(
    stream: TokenStream,
    replaced_ident: TokenStream,
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
                                            ident.to_token_stream()
                                        }
                                        _ => replaced_ident,
                                    }
                                }
                                _ => replaced_ident,
                            }
                        }
                        _ => replaced_ident,
                    };

                    result_stream.extend(replaced);
                } else {
                    result_stream.push(TokenTree::Ident(ident));
                }
            }
            TokenTree::Group(group) => {
                let replaced_stream = replace_self_type_in_token_stream(
                    group.stream(),
                    replaced_ident.clone(),
                    local_assoc_types,
                );
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
