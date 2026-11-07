use cgp_macro_core::types::cgp_data::ItemCgpRecord;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

/// Generate shared and mutable accessors for every field of a struct, keyed by type-level tags, which is what lets an implicit argument find a field.
///
/// See [`HasField`](https://contextgeneric.dev/docs/reference/derives/derive_has_field).
pub fn derive_has_field(input: TokenStream) -> syn::Result<TokenStream> {
    let item_struct: ItemStruct = syn::parse2(input)?;

    let record = ItemCgpRecord { item_struct };

    let item_impls = record.to_has_field_impls()?;

    Ok(quote! {
        #( #item_impls )*
    })
}
