use cgp_fork_macro_core::types::cgp_preset::ItemCgpPreset;
use proc_macro2::TokenStream;
use syn::parse2;

/// `cgp_preset!` entry point: a preset module whose `Provider` delegates each
/// entry, plus `with_components!` so another preset or `delegate_components!`
/// can replay those entries.
pub fn cgp_preset(body: TokenStream) -> syn::Result<TokenStream> {
    let item: ItemCgpPreset = parse2(body)?;
    item.expand()
}
