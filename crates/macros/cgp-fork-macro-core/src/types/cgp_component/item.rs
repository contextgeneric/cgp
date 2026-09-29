use syn::ItemTrait;

use crate::types::attributes::CgpComponentAttributes;
use crate::types::cgp_component::{CgpComponentArgs, PreprocessedCgpComponent};

/// Raw input stage: the parsed attribute args and trait, before CGP attributes
/// are stripped. First stage of the `#[cgp_component]` pipeline.
pub struct ItemCgpComponent {
    pub args: CgpComponentArgs,
    pub item_trait: ItemTrait,
}

impl ItemCgpComponent {
    /// Split the CGP modifier attributes off the trait, yielding the next stage.
    pub fn preprocess(&self) -> syn::Result<PreprocessedCgpComponent> {
        let (attributes, item_trait) = CgpComponentAttributes::preprocess(&self.item_trait)?;

        Ok(PreprocessedCgpComponent {
            args: self.args.clone(),
            item_trait,
            attributes,
        })
    }
}
