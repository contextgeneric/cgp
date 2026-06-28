use proc_macro2::TokenStream;
use quote::quote;
use syn::{Item, parse2};

use crate::keywords::CgpData;
use crate::types::DeriveMacroSnapshot;

pub fn snapshot_derive_cgp_data(body: TokenStream) -> syn::Result<TokenStream> {
    let item: DeriveMacroSnapshot<CgpData, Item> = parse2(body)?;

    let body = &item.body;

    let output = cgp_macro_lib::derive_cgp_data(quote! {
        #[derive(CgpData)]
        #body
    })?;

    let wrapped = item.snapshot.wrap_output(output)?;

    Ok(quote! {
        #body

        #wrapped
    })
}
