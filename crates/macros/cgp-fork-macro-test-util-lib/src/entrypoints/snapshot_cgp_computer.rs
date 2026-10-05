use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{ItemFn, parse2};

use crate::keywords::CgpComputer;
use crate::types::AttributeMacroSnapshot;

pub fn snapshot_cgp_computer(body: TokenStream) -> syn::Result<TokenStream> {
    let item: AttributeMacroSnapshot<CgpComputer, ItemFn> = parse2(body)?;

    let output = cgp_fork_extra_macro_lib::cgp_computer(item.attr, item.body.to_token_stream())?;

    item.snapshot.wrap_output(output)
}
