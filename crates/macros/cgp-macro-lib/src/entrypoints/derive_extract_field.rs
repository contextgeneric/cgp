use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemEnum, parse2};

use crate::derive_extractor::{
    derive_extract_field_impls, derive_extractor_enum, derive_extractor_enum_ref,
    derive_finalize_extract_impl, derive_has_extractor_impl, derive_has_extractor_mut_impl,
    derive_has_extractor_ref_impl, derive_partial_data_impl,
};

pub fn derive_extract_field(body: TokenStream) -> syn::Result<TokenStream> {
    let context_enum: ItemEnum = parse2(body)?;
    derive_extract_field_from_enum(&context_enum)
}

pub fn derive_extract_field_from_enum(context_enum: &ItemEnum) -> syn::Result<TokenStream> {
    let context_ident = &context_enum.ident;

    let extractor_ident = Ident::new(&format!("__Partial{context_ident}"), context_ident.span());
    let extractor_enum = derive_extractor_enum(context_enum, &extractor_ident)?;

    let extractor_ref_ident = Ident::new(
        &format!("__PartialRef{context_ident}"),
        context_ident.span(),
    );
    let extractor_ref_enum = derive_extractor_enum_ref(context_enum, &extractor_ref_ident)?;

    let has_extractor_impl = derive_has_extractor_impl(context_enum, &extractor_ident)?;

    let has_extractor_ref_impl = derive_has_extractor_ref_impl(context_enum, &extractor_ref_ident)?;

    let has_extractor_mut_impl = derive_has_extractor_mut_impl(context_enum, &extractor_ref_ident)?;

    let finalize_extract_impl =
        derive_finalize_extract_impl(context_enum, &extractor_ident, false)?;

    let finalize_extract_ref_impl =
        derive_finalize_extract_impl(context_enum, &extractor_ref_ident, true)?;

    let partial_data_impl = derive_partial_data_impl(context_enum, &extractor_ident, false)?;
    let partial_ref_data_impl = derive_partial_data_impl(context_enum, &extractor_ref_ident, true)?;

    let extractor_impls = derive_extract_field_impls(context_enum, &extractor_ident, false)?;
    let extractor_ref_impls = derive_extract_field_impls(context_enum, &extractor_ref_ident, true)?;

    let out = quote! {
        #extractor_enum
        #extractor_ref_enum

        #partial_data_impl
        #partial_ref_data_impl

        #has_extractor_impl
        #has_extractor_ref_impl
        #has_extractor_mut_impl

        #finalize_extract_impl
        #finalize_extract_ref_impl

        #(#extractor_impls)*
        #(#extractor_ref_impls)*
    };

    Ok(out)
}
