use syn::ItemImpl;

use crate::types::attributes::ImplAttributes;
use crate::types::cgp_impl::{CgpImplWithParsedAttributes, ImplArgs};

pub struct ItemCgpImpl {
    pub args: ImplArgs,
    pub item_impl: ItemImpl,
}

impl ItemCgpImpl {
    pub fn lower(&self) -> syn::Result<CgpImplWithParsedAttributes> {
        let mut item_impl = self.item_impl.clone();

        let attributes = ImplAttributes::parse(&item_impl.attrs)?;
        item_impl.attrs = attributes.raw_attributes;

        todo!()
    }
}
