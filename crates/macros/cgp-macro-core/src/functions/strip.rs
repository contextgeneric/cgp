use proc_macro2::{Group, TokenStream, TokenTree};

/// Strips the `::cgp::macro_prelude::` prefix from the [`TokenStream`] so the
/// error message shows the more readable, unqualified paths. The replacement is
/// done at the token level, recursing into nested groups.
pub fn strip_macro_prelude(body: TokenStream) -> TokenStream {
    // The prefix `::cgp::macro_prelude::` is made up of the following tokens.
    fn is_prefix(tokens: &[TokenTree]) -> bool {
        matches!(
            tokens,
            [
                TokenTree::Punct(p1),
                TokenTree::Punct(p2),
                TokenTree::Ident(cgp),
                TokenTree::Punct(p3),
                TokenTree::Punct(p4),
                TokenTree::Ident(prelude),
                TokenTree::Punct(p5),
                TokenTree::Punct(p6),
            ] if p1.as_char() == ':'
                && p2.as_char() == ':'
                && cgp == "cgp"
                && p3.as_char() == ':'
                && p4.as_char() == ':'
                && prelude == "macro_prelude"
                && p5.as_char() == ':'
                && p6.as_char() == ':'
        )
    }

    const PREFIX_LEN: usize = 8;

    let tokens: Vec<TokenTree> = body.into_iter().collect();
    let mut output = Vec::with_capacity(tokens.len());
    let mut i = 0;

    while i < tokens.len() {
        if is_prefix(&tokens[i..(i + PREFIX_LEN).min(tokens.len())]) {
            i += PREFIX_LEN;
        } else {
            match &tokens[i] {
                TokenTree::Group(group) => {
                    let inner = strip_macro_prelude(group.stream());
                    let mut new_group = Group::new(group.delimiter(), inner);
                    new_group.set_span(group.span());
                    output.push(TokenTree::Group(new_group));
                }
                other => output.push(other.clone()),
            }
            i += 1;
        }
    }

    output.into_iter().collect()
}
