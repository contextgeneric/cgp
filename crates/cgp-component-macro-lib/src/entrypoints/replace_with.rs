use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;

use crate::for_each_replace::{replace_stream, ReplaceSpecs};

pub fn replace_with(tokens: TokenStream) -> syn::Result<TokenStream> {
    let specs: ReplaceSpecs = syn::parse2(tokens)?;

    let items: Punctuated<TokenStream, Comma> = specs.replacements.into_iter().collect();

    let tokens = quote! { [ #items ] };

    Ok(replace_stream(&specs.target_ident, &tokens, specs.body))
}
