use proc_macro2::{Span, TokenTree};
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
/// *generated item* while leaving the user's tokens navigable, use
/// [`override_item_span`] instead.
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

/// Re-span a generated item's *boundary* tokens onto `span` — its leading token
/// and its trailing token, nothing in between — and re-parse into `T`.
///
/// A macro builds an item from quasi-quoted tokens (`quote!`/`parse_internal!`),
/// which stamp every token they emit — the `impl` keyword, the trait reference,
/// the fully-qualified `exports` path, the `{ … }` body — with the macro's
/// `call_site` span, the whole invocation. A compiler error on the item's header
/// (a coherence conflict `E0119`, an unsatisfied bound) then underlines the
/// entire macro block instead of the entry the user wrote.
///
/// The compiler derives a generated item's span — and therefore that caret —
/// from its *first and last* tokens joined together (`first.to(last)`): with the
/// whole item at `call_site` the caret is the whole invocation, and re-spanning
/// just those two boundary tokens onto `span` collapses it onto the entry. The
/// interior tokens are deliberately left alone, and that is what keeps an editor
/// working: rust-analyzer maps a source token to an expanded one purely by source
/// range (hygiene is ignored), so a synthesized identifier — `IsProviderFor`,
/// `DelegateComponent` — re-spanned onto the entry would collide with the key the
/// user wrote, and go-to-definition on that key would then offer every collided
/// construct. Leaving the interior untouched keeps each synthesized reference at
/// `call_site` (never coinciding with a narrower user token) and each user token
/// — the wired provider, the target type, a per-entry generic — at its own span,
/// so navigation resolves to the one right definition.
///
/// The two boundary tokens re-spanned here are structural (a keyword and a
/// delimiter group), never references, so moving them onto the entry cannot
/// mislead the editor. Each is re-spanned only if it is itself synthesized, so a
/// hand-assembled item whose boundary token the user wrote is left untouched.
pub fn override_item_span<T>(span: Span, body: &T) -> syn::Result<T>
where
    T: Parse + ToTokens,
{
    let call_site_text = Span::call_site().source_text();
    let mut trees: Vec<TokenTree> = body.to_token_stream().into_iter().collect();

    if let Some(first) = trees.first_mut()
        && is_synthesized(first.span(), &call_site_text)
    {
        first.set_span(span);
    }
    if let Some(last) = trees.last_mut()
        && is_synthesized(last.span(), &call_site_text)
    {
        last.set_span(span);
    }

    parse2(trees.into_iter().collect())
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
