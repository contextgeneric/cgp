use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Ident, ItemEnum};

use crate::derive_extractor::{
    derive_extract_field_impls, derive_extractor_enum, derive_finalize_extract_impl,
    derive_has_extractor_impl, derive_has_extractor_ref_impl,
};

pub fn derive_extract_field(body: TokenStream) -> syn::Result<TokenStream> {
    let context_enum: ItemEnum = parse2(body)?;

    let context_ident = &context_enum.ident;

    let extractor_ident = Ident::new(&format!("Partial{context_ident}"), context_ident.span());
    let extractor_enum = derive_extractor_enum(&context_enum, &extractor_ident, false)?;

    let extractor_ref_ident =
        Ident::new(&format!("PartialRef{context_ident}"), context_ident.span());
    let extractor_ref_enum = derive_extractor_enum(&context_enum, &extractor_ref_ident, true)?;

    let has_extractor_impl = derive_has_extractor_impl(&context_enum, &extractor_ident)?;
    let has_extractor_ref_impl =
        derive_has_extractor_ref_impl(&context_enum, &extractor_ref_ident)?;

    let finalize_extract_impl =
        derive_finalize_extract_impl(&context_enum, &extractor_ident, false)?;
    let finalize_extract_ref_impl =
        derive_finalize_extract_impl(&context_enum, &extractor_ref_ident, true)?;

    let extractor_impls = derive_extract_field_impls(&context_enum, &extractor_ident, false)?;
    let extractor_ref_impls =
        derive_extract_field_impls(&context_enum, &extractor_ref_ident, true)?;

    let out = quote! {
        #extractor_enum
        #extractor_ref_enum

        #has_extractor_impl
        #has_extractor_ref_impl

        #finalize_extract_impl
        #finalize_extract_ref_impl

        #(#extractor_impls)*
        #(#extractor_ref_impls)*
    };

    Ok(out)
}
