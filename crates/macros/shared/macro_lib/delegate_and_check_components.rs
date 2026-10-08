use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

use crate::macro_core::types::delegate_and_check_components::ItemDelegateAndCheckComponents;

pub fn delegate_and_check_components(body: TokenStream) -> syn::Result<TokenStream> {
    let item: ItemDelegateAndCheckComponents = parse2(body)?;

    let check_table = item.to_check_components()?;

    let evaluated_table = item.table.eval()?;

    let check_items = check_table.to_items()?;

    Ok(quote! {
        #evaluated_table

        #( #check_items )*
    })
}
