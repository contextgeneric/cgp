use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, ItemStruct};

use crate::derive_build_field_from_struct;
use crate::derive_has_fields::derive_has_fields_impls_from_struct;
use crate::field::derive_has_field_impls_from_struct;

pub fn derive_cgp_record(body: TokenStream) -> syn::Result<TokenStream> {
    let item_struct = parse2(body)?;
    derive_cgp_record_from_struct(&item_struct)
}

pub fn derive_cgp_record_from_struct(item_struct: &ItemStruct) -> syn::Result<TokenStream> {
    let has_field_impls = derive_has_field_impls_from_struct(item_struct);
    let has_fields_impls = derive_has_fields_impls_from_struct(item_struct)?;
    let build_field_impls = derive_build_field_from_struct(item_struct)?;

    Ok(quote! {
        #( #has_field_impls )*
        #( #has_fields_impls )*
        #build_field_impls
    })
}
