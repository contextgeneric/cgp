use syn::ItemTrait;

use crate::types::attributes::CgpComponentAttributes;
use crate::types::cgp_component::{CgpComponentArgs, PreprocessedCgpComponent};

pub struct ItemCgpComponent {
    pub args: CgpComponentArgs,
    pub item_trait: ItemTrait,
}

impl ItemCgpComponent {
    pub fn preprocess(&self) -> syn::Result<PreprocessedCgpComponent> {
        let (attributes, item_trait) = CgpComponentAttributes::preprocess(&self.item_trait)?;

        Ok(PreprocessedCgpComponent {
            args: self.args.clone(),
            item_trait,
            attributes,
        })
    }
}
