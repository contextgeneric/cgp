use cgp_macro_core::types::cgp_data::ItemCgpRecord;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub fn derive_has_field(input: TokenStream) -> syn::Result<TokenStream> {
    let item_struct: ItemStruct = syn::parse2(input)?;

    let record = ItemCgpRecord { item_struct };

    let item_impls = record.to_has_field_impls()?;

    Ok(quote! {
        #( #item_impls )*
    })
}
