use proc_macro2::TokenStream;
use syn::ItemFn;

use crate::cgp_fn::extract_implicits_args;

pub fn derive_cgp_fn(item_fn: &mut ItemFn) -> syn::Result<TokenStream> {
    let implicit_args = extract_implicits_args(&mut item_fn.sig.inputs)?;

    todo!()
}
