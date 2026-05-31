use syn::ItemImpl;

use crate::types::cgp_impl::ImplArgs;

pub struct CgpImplWithParsedAttributes {
    pub args: ImplArgs,
    pub item_impl: ItemImpl,
}
