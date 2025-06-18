use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Ident, ItemEnum};

use crate::derive_extractor::derive_extractor_enum;

pub fn derive_extract_field(body: TokenStream) -> syn::Result<TokenStream> {
    let context_enum: ItemEnum = parse2(body)?;

    let context_ident = &context_enum.ident;
    let extractor_ident = Ident::new(&format!("Partial{}", context_ident), context_ident.span());

    let extractor_enum = derive_extractor_enum(&context_enum, &extractor_ident)?;

    let out = quote! {
        #extractor_enum
    };

    Ok(out)
}
