use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse2;

pub fn override_span<T>(span: &Span, body: &T) -> syn::Result<T>
where
    T: Parse + ToTokens,
{
    parse2(
        body.to_token_stream()
            .into_iter()
            .map(|mut tree| {
                tree.set_span(*span);
                tree
            })
            .collect(),
    )
}
