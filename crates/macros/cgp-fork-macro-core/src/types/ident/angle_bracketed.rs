use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Gt, Lt};

/// Parse an optional `< T, T, ... >` comma-separated list. An absent leading
/// `<` yields an empty list, matching how both the type-argument and the
/// type-generic-parameter lists treat the bare (no angle brackets) case.
pub fn parse_angle_bracketed<T: Parse>(input: ParseStream) -> syn::Result<Punctuated<T, Comma>> {
    let mut items = Punctuated::new();

    if !input.peek(Lt) {
        return Ok(items);
    }

    let _: Lt = input.parse()?;

    while !input.peek(Gt) {
        items.push_value(input.parse()?);

        if input.peek(Gt) {
            break;
        }

        items.push_punct(input.parse()?);
    }

    let _: Gt = input.parse()?;

    Ok(items)
}

/// Emit a `< T, T, ... >` list, or nothing when the list is empty. The inverse
/// of [`parse_angle_bracketed`].
pub fn to_tokens_angle_bracketed<T: ToTokens>(
    items: &Punctuated<T, Comma>,
    tokens: &mut TokenStream,
) {
    if !items.is_empty() {
        tokens.extend(quote! { < #items > });
    }
}
