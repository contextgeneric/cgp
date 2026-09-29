use itertools::Itertools;
use proc_macro2::{Group, TokenStream, TokenTree};
use quote::format_ident;
use syn::visit_mut::{self, VisitMut};
use syn::{Expr, Ident, Item, Macro, Path};

/// Rewrites every `self` *value* expression to the context identifier, stopping
/// at nested items (which introduce their own `self` and do not capture the
/// outer one) and leaving `self::` module paths inside macro bodies intact.
pub struct ReplaceSelfValueVisitor<'a> {
    pub replaced_ident: &'a Ident,
}

impl VisitMut for ReplaceSelfValueVisitor<'_> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Path(expr_path)
                if expr_path.qself.is_none() && expr_path.path.is_ident("self") =>
            {
                expr_path.path = Path::from(self.replaced_ident.clone());
            }
            _ => visit_mut::visit_expr_mut(self, expr),
        }
    }

    fn visit_macro_mut(&mut self, mac: &mut Macro) {
        // Macro bodies are opaque to VisitMut, so fall back to token-level replacement.
        mac.tokens = replace_self_value_in_token_stream(mac.tokens.clone(), self.replaced_ident);
    }

    fn visit_item_mut(&mut self, _item: &mut Item) {
        // A block-nested item (`fn`, `impl`, `trait`, …) does not capture `self`
        // from the outer scope; a nested `impl`/`trait` introduces its own
        // receiver. Stop before descending into it.
    }
}

pub fn replace_self_value_in_token_stream(
    stream: TokenStream,
    replaced_ident: &Ident,
) -> TokenStream {
    let self_ident = format_ident!("self");

    let mut result_stream: Vec<TokenTree> = Vec::new();

    let mut token_iter = stream.into_iter().multipeek();

    while let Some(tree) = token_iter.next() {
        match tree {
            TokenTree::Ident(ident) => {
                // Rewrite `self` as a value expression, but leave a `self::` module
                // path intact — its leading `self` is the current module, not the
                // receiver. A value `self` is never followed by `::`, so a trailing
                // `::` unambiguously marks the path form.
                let is_module_path = matches!(token_iter.peek(), Some(TokenTree::Punct(p)) if p.as_char() == ':')
                    && matches!(token_iter.peek(), Some(TokenTree::Punct(p)) if p.as_char() == ':');

                if ident == self_ident && !is_module_path {
                    result_stream.push(TokenTree::Ident(replaced_ident.clone()));
                } else {
                    result_stream.push(TokenTree::Ident(ident));
                }
            }
            TokenTree::Group(group) => {
                let replaced_stream =
                    replace_self_value_in_token_stream(group.stream(), replaced_ident);
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
