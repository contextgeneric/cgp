use syn::ItemImpl;

use crate::types::cgp_component::CgpComponentArgs;

pub struct ItemCgpComponent {
    pub args: CgpComponentArgs,
    pub item_impl: ItemImpl,
}
