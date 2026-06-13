use syn::ItemTrait;

use crate::types::attributes::CgpComponentAttributes;
use crate::types::cgp_component::{CgpComponentArgs, PreprocessedCgpComponent};

pub struct ItemCgpComponent {
    pub args: CgpComponentArgs,
    pub item_trait: ItemTrait,
}

impl ItemCgpComponent {
    pub fn preprocess(&self) -> syn::Result<PreprocessedCgpComponent> {
        let mut item_trait = self.item_trait.clone();

        let attributes = CgpComponentAttributes::parse(&mut item_trait.attrs)?;

        item_trait.supertraits.extend(attributes.extend.clone());

        attributes.use_type.transform_item_trait(&mut item_trait)?;

        Ok(PreprocessedCgpComponent {
            args: self.args.clone(),
            item_trait,
            attributes,
        })
    }
}
