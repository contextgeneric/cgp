use cgp_macro_core::types::delegate_and_check_components::ItemDelegateAndCheckComponents;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

/// Wire a context and check that wiring in one macro, for getting started and for plain Component: Provider tables. Wrong for an aggregate provider.
///
/// See [`delegate_and_check_components`](https://contextgeneric.dev/docs/reference/macros/delegate_and_check_components).
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
