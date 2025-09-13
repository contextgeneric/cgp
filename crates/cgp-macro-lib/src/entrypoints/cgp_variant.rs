use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemEnum};

use crate::derive_has_fields::derive_has_fields_impls_from_enum;
use crate::{derive_extract_field_from_enum, derive_from_variant_from_enum};

pub fn derive_cgp_variant(body: TokenStream) -> syn::Result<TokenStream> {
    let item_enum = parse2(body)?;
    derive_cgp_variant_from_enum(&item_enum)
}

pub fn derive_cgp_variant_from_enum(item_enum: &ItemEnum) -> syn::Result<TokenStream> {
    let has_fields = derive_has_fields_impls_from_enum(item_enum)?;
    let extract_field = derive_extract_field_from_enum(item_enum)?;
    let from_variant = derive_from_variant_from_enum(item_enum)?;

    Ok(quote! {
        #( #has_fields )*
        #extract_field
        #from_variant
    })
}
