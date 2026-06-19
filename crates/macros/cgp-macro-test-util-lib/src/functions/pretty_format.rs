use cgp_macro_core::functions::strip_macro_prelude;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use syn::parse2;

pub fn pretty_format(body: TokenStream) -> syn::Result<String> {
    let parsed = parse2(strip_macro_prelude(body))?;
    let formatted = unparse(&parsed);
    Ok(formatted)
}
