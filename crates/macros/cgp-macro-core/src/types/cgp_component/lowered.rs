use syn::ItemTrait;

use crate::types::cgp_component::CgpComponentArgs;

pub struct LoweredCgpComponent {
    pub args: CgpComponentArgs,
    pub item_trait: ItemTrait,
}

impl LoweredCgpComponent {
    // pub fn
}
