use proc_macro2::TokenStream;
use syn::{Ident, ItemFn, parse2};

use crate::extra_macro_core::types::cgp_computer::ItemCgpComputer;
use crate::extra_macro_lib::handler_fn::lower_handler_fn;

pub fn cgp_computer(attr: TokenStream, body: TokenStream) -> syn::Result<TokenStream> {
    let item_fn: ItemFn = parse2(body)?;

    let ident: Option<Ident> = parse2(attr)?;

    let evaluated = ItemCgpComputer { ident, item_fn }.preprocess()?.eval()?;

    lower_handler_fn(&evaluated)
}
