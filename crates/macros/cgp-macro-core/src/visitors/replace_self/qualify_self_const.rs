use itertools::Itertools;
use proc_macro2::{Group, TokenStream, TokenTree};
use syn::visit_mut::{self, VisitMut};
use syn::{ExprPath, Ident, Item, Macro, parse_quote};

/// Rewrites `Self::CONST`, for a const the `#[cgp_impl]` block declares, to the
/// qualified `<Provider as ProviderTrait<Context>>::CONST`.
///
/// Once the block's `Self` is the provider, a bare `Self::CONST` is ambiguous:
/// the consumer trait's blanket impl gives the provider the same const.
pub struct QualifySelfConstVisitor<'a> {
    /// The consts the block declares.
    pub local_consts: &'a Vec<Ident>,
    /// The qualified self, `<Provider as ProviderTrait<Context>>`.
    pub qualified_self: &'a TokenStream,
}

impl QualifySelfConstVisitor<'_> {
    fn local_const<'p>(&self, expr: &'p ExprPath) -> Option<&'p Ident> {
        let segments = &expr.path.segments;
        if expr.qself.is_some() || expr.path.leading_colon.is_some() || segments.len() != 2 {
            return None;
        }
        let (first, second) = (&segments[0], &segments[1]);
        (first.ident == "Self"
            && first.arguments.is_none()
            && second.arguments.is_none()
            && self.local_consts.contains(&second.ident))
        .then_some(&second.ident)
    }
}

impl VisitMut for QualifySelfConstVisitor<'_> {
    fn visit_expr_path_mut(&mut self, expr: &mut ExprPath) {
        if let Some(ident) = self.local_const(expr) {
            let qualified_self = self.qualified_self;
            let mut qualified: ExprPath = parse_quote!(#qualified_self :: #ident);
            qualified.attrs = core::mem::take(&mut expr.attrs);
            *expr = qualified;
            return;
        }
        visit_mut::visit_expr_path_mut(self, expr);
    }

    fn visit_macro_mut(&mut self, mac: &mut Macro) {
        mac.tokens = qualify_self_const_in_token_stream(
            core::mem::take(&mut mac.tokens),
            self.qualified_self,
            self.local_consts,
        );
    }

    fn visit_item_mut(&mut self, _item: &mut Item) {
        // A block-nested item has its own `Self`; leave it untouched.
    }
}

/// The token-level form of [`QualifySelfConstVisitor`], for macro bodies.
pub fn qualify_self_const_in_token_stream(
    stream: TokenStream,
    qualified_self: &TokenStream,
    local_consts: &Vec<Ident>,
) -> TokenStream {
    let mut result_stream: Vec<TokenTree> = Vec::new();

    let mut token_iter = stream.into_iter().multipeek();

    while let Some(tree) = token_iter.next() {
        match tree {
            TokenTree::Ident(ident) if ident == "Self" => {
                let names_local_const = matches!(token_iter.peek(), Some(TokenTree::Punct(p)) if p.as_char() == ':')
                    && matches!(token_iter.peek(), Some(TokenTree::Punct(p)) if p.as_char() == ':')
                    && matches!(token_iter.peek(), Some(TokenTree::Ident(name)) if local_consts.contains(name));

                if names_local_const {
                    result_stream.extend(qualified_self.clone());
                } else {
                    result_stream.push(TokenTree::Ident(ident));
                }
            }
            TokenTree::Group(group) => {
                let replaced_stream = qualify_self_const_in_token_stream(
                    group.stream(),
                    qualified_self,
                    local_consts,
                );
                let mut replaced_group = Group::new(group.delimiter(), replaced_stream);
                replaced_group.set_span(group.span());

                result_stream.push(TokenTree::Group(replaced_group));
            }
            tree => result_stream.push(tree),
        }
    }

    result_stream.into_iter().collect()
}
