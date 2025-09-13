use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Ident, ItemStruct};

use crate::derive_builder::{
    derive_builder_struct, derive_finalize_build_impl, derive_has_builder_impl,
    derive_has_field_impls, derive_into_builder_impl, derive_partial_data_impl,
    derive_update_field_impls,
};

pub fn derive_build_field(body: TokenStream) -> syn::Result<TokenStream> {
    let context_struct: ItemStruct = parse2(body)?;
    derive_build_field_from_struct(&context_struct)
}

pub fn derive_build_field_from_struct(context_struct: &ItemStruct) -> syn::Result<TokenStream> {
    let context_ident = &context_struct.ident;
    let builder_ident = Ident::new(&format!("Partial{context_ident}"), context_ident.span());

    let builder_struct = derive_builder_struct(context_struct, &builder_ident)?;

    let has_builder_impl = derive_has_builder_impl(context_struct, &builder_ident)?;

    let into_builder_impl = derive_into_builder_impl(context_struct, &builder_ident)?;

    let partial_data_impl = derive_partial_data_impl(context_struct, &builder_ident)?;

    let update_field_impls = derive_update_field_impls(context_struct, &builder_ident)?;

    let has_field_impls = derive_has_field_impls(context_struct, &builder_ident)?;

    let finalize_build_impl = derive_finalize_build_impl(context_struct, &builder_ident)?;

    let out = quote! {
        #builder_struct

        #has_builder_impl

        #into_builder_impl

        #partial_data_impl

        #(#update_field_impls)*

        #(#has_field_impls)*

        #finalize_build_impl
    };

    Ok(out)
}
