use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse2;

/// Re-span every top-level token of `body` to `span` and re-parse it into `T`.
///
/// A macro builds its output from quasi-quoted tokens, which carry the whole
/// invocation's `call_site` span; a compiler error on such an item therefore
/// highlights the entire macro block. Re-spanning the tokens onto the specific
/// user-written token that produced the item (a component key, a wiring entry)
/// makes the diagnostic point there instead. `check_components!` uses this to
/// aim an unsatisfied-bound error at the checked component, and
/// `delegate_components!` to aim a coherence conflict at the offending entry.
///
/// Only top-level tokens are re-spanned; tokens nested inside a delimiter group
/// (an impl body, say) keep their spans, which is why callers that must preserve
/// an inner span — the generic parameters of a generated impl, for instance —
/// restore it after re-spanning rather than relying on this to skip it.
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
