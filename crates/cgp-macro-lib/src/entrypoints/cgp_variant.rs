use proc_macro2::TokenStream;
use quote::quote;

use crate::entrypoints::{derive_extract_field, derive_from_variant, derive_has_fields};

pub fn cgp_variant(body: TokenStream) -> syn::Result<TokenStream> {
    let has_fields = derive_has_fields(body.clone())?;
    let extract_field = derive_extract_field(body.clone())?;
    let from_variant = derive_from_variant(body)?;

    Ok(quote! {
        #has_fields
        #extract_field
        #from_variant
    })
}
