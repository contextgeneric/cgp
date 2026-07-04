use proc_macro2::{Group, Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse2;

/// Force every top-level token of `body` onto `span`, unconditionally, and
/// re-parse it into `T`.
///
/// This clobbers the span of *user-written* tokens too, so it is only for the
/// case where that is the intent: `check_components!` re-spans the context type
/// — a single user token, reused once per checked component — onto each
/// component in turn, so an unsatisfied-bound error is reported on the component
/// the user listed rather than on the one shared context token. To re-span a
/// *generated* item while leaving the user's tokens navigable, use
/// [`override_synthesized_span`] instead.
pub fn override_span<T>(span: Span, body: &T) -> syn::Result<T>
where
    T: Parse + ToTokens,
{
    parse2(
        body.to_token_stream()
            .into_iter()
            .map(|mut tree| {
                tree.set_span(span);
                tree
            })
            .collect(),
    )
}

/// Re-span the macro-synthesized tokens of `body` onto `span` — leaving tokens
/// that came from user input at their original spans — and re-parse into `T`.
///
/// A macro builds an item from quasi-quoted tokens (`quote!`/`parse_internal!`),
/// which stamp every *structural* token they emit — the `impl` keyword, a trait
/// reference, a fully-qualified `exports` path, the reserved `__Context__`
/// generics — with the macro's `call_site` span, i.e. the whole invocation. Only
/// the *interpolated* fragments the user wrote (the self type, the wired provider,
/// a per-entry generic) keep a narrower span. A compiler error on the item's
/// header (a coherence conflict `E0119`, an unsatisfied bound) therefore
/// underlines the entire macro block instead of the entry the user wrote.
///
/// Re-spanning only the `call_site`-stamped tokens onto `span` — the originating
/// token — aims that diagnostic at the entry while keeping the user's own tokens
/// where they are. That distinction matters for the IDE: an editor resolves
/// go-to-definition on a type written inside a macro (a provider in
/// `delegate_components!`, say) by mapping the user's token to the same-spanned
/// token in the expansion, so clobbering a user token's span onto the entry
/// breaks the jump. Preserving user spans keeps navigation working; overriding
/// only the synthesized scaffolding keeps the compile-fail carets on the entry.
///
/// A synthesized token is recognized by its span carrying the same source text as
/// `Span::call_site()` — the whole macro invocation — which every `quote!`-stamped
/// token shares and which no narrower user token can equal. The walk recurses into
/// delimiter groups (an impl body) so a synthesized token nested inside one is
/// re-spanned too, while a user token nested there stays put.
pub fn override_synthesized_span<T>(span: Span, body: &T) -> syn::Result<T>
where
    T: Parse + ToTokens,
{
    let call_site_text = Span::call_site().source_text();
    parse2(respan_synthesized(
        span,
        &call_site_text,
        body.to_token_stream(),
    ))
}

/// Re-span each `call_site`-stamped token of `stream` onto `span`, recursing into
/// delimiter groups; see [`override_synthesized_span`].
fn respan_synthesized(
    span: Span,
    call_site_text: &Option<String>,
    stream: TokenStream,
) -> TokenStream {
    stream
        .into_iter()
        .map(|tree| match tree {
            TokenTree::Group(group) => {
                let mut new_group = Group::new(
                    group.delimiter(),
                    respan_synthesized(span, call_site_text, group.stream()),
                );
                new_group.set_span(if is_synthesized(group.span(), call_site_text) {
                    span
                } else {
                    group.span()
                });
                TokenTree::Group(new_group)
            }
            mut tree => {
                if is_synthesized(tree.span(), call_site_text) {
                    tree.set_span(span);
                }
                tree
            }
        })
        .collect()
}

/// Whether `token_span` was stamped by `quote!`/`parse_internal!` rather than
/// carried in from user input. A synthesized token carries the macro's
/// `call_site` span, whose source text is the entire invocation; a user token's
/// source text is just that token, so the two never coincide. Comparing the
/// source text works because a synthesized token *is* `call_site`, so it yields
/// the exact same result as `call_site_text` — even when that is `None`.
fn is_synthesized(token_span: Span, call_site_text: &Option<String>) -> bool {
    token_span.source_text() == *call_site_text
}
