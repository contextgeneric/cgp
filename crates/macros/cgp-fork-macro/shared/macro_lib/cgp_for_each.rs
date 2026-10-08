use proc_macro2::TokenStream;
use syn::parse2;

use crate::macro_core::types::cgp_for_each::CgpForEach;

/// Repeat a token body once per component type. Preset `with_components!`
/// macros expand through this so one preset entry becomes many impls.
pub fn cgp_for_each(body: TokenStream) -> syn::Result<TokenStream> {
    let item: CgpForEach = parse2(body)?;
    Ok(item.expand())
}
