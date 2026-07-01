use cgp_macro_core::types::cgp_component::{CgpComponentArgs, ItemCgpComponent};
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemTrait;

/// `#[cgp_component]` entry point: parse the attribute args and the trait, then
/// run the `preprocess → eval → to_items` pipeline and emit the derived items.
pub fn cgp_component(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let args: CgpComponentArgs = syn::parse2(attr)?;
    let item_trait: ItemTrait = syn::parse2(item)?;

    let item_cgp_component = ItemCgpComponent { args, item_trait };

    let derived = item_cgp_component.preprocess()?.eval()?.to_items()?;

    Ok(quote! {
        #( #derived )*
    })
}
