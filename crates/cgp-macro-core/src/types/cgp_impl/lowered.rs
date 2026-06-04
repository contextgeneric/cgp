use syn::ItemImpl;

use crate::types::cgp_impl::ImplArgs;

pub struct LoweredCgpImpl {
    pub args: ImplArgs,
    pub item_impl: ItemImpl,
}
