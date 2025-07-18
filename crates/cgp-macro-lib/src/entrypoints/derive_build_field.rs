use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Ident, ItemStruct};

use crate::derive_builder::{
    derive_build_field_impls, derive_builder_struct, derive_finalize_build_impl,
    derive_has_builder_impl, derive_has_field_impls, derive_into_builder_impl,
    derive_take_field_impls, derive_transform_map_impl,
};

pub fn derive_build_field(body: TokenStream) -> syn::Result<TokenStream> {
    let context_struct: ItemStruct = parse2(body)?;

    let context_ident = &context_struct.ident;
    let builder_ident = Ident::new(&format!("Partial{context_ident}"), context_ident.span());

    let builder_struct = derive_builder_struct(&context_struct, &builder_ident)?;

    let has_builder_impl = derive_has_builder_impl(&context_struct, &builder_ident)?;

    let into_builder_impl = derive_into_builder_impl(&context_struct, &builder_ident)?;

    let build_field_impls = derive_build_field_impls(&context_struct, &builder_ident)?;

    let has_field_impls = derive_has_field_impls(&context_struct, &builder_ident)?;

    let take_field_impls = derive_take_field_impls(&context_struct, &builder_ident)?;

    let finalize_build_impl = derive_finalize_build_impl(&context_struct, &builder_ident)?;

    let transform_map_impl = derive_transform_map_impl(&context_struct, &builder_ident)?;

    let out = quote! {
        #builder_struct

        #has_builder_impl

        #into_builder_impl

        #(#build_field_impls)*

        #(#has_field_impls)*

        #(#take_field_impls)*

        #finalize_build_impl

        #transform_map_impl
    };

    Ok(out)
}
