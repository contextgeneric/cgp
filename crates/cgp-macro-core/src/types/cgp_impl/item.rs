use syn::ItemImpl;

use crate::types::cgp_impl::ImplArgs;

pub struct ItemCgpImpl {
    pub args: ImplArgs,
    pub item_impl: ItemImpl,
}
