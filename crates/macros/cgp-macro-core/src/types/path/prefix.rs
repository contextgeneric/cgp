use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Type;
use syn::punctuated::Punctuated;
use syn::token::Dot;

use crate::exports::PathCons;
use crate::types::path::PathElement;

/// A path prefix with a suffix appended.
#[derive(Debug, Clone)]
pub struct PrefixPath {
    /// The path's elements.
    pub elements: Punctuated<PathElement, Dot>,
    /// The type appended after the elements.
    pub suffix: Type,
}

impl ToTokens for PrefixPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let out = self.elements.iter().rev().fold(
            self.suffix.to_token_stream(),
            |acc, current| quote!( #PathCons < #current, #acc > ),
        );

        tokens.extend(out)
    }
}
