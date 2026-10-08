use syn::{Error, ItemTrait, TraitItem};

use crate::extra_macro_core::types::cgp_auto_dispatch::{
    DispatchMethod, PreprocessedCgpAutoDispatch,
};

/// Raw input stage: the annotated trait. First stage of the
/// `#[cgp_auto_dispatch]` pipeline.
pub struct ItemCgpAutoDispatch {
    pub item_trait: ItemTrait,
}

impl ItemCgpAutoDispatch {
    /// Check that every trait item is a method the macro can dispatch, yielding
    /// the next stage with one [`DispatchMethod`] per method.
    pub fn preprocess(&self) -> syn::Result<PreprocessedCgpAutoDispatch> {
        let mut methods = Vec::new();

        for item in self.item_trait.items.iter() {
            match item {
                TraitItem::Fn(method) => {
                    methods.push(DispatchMethod::new(method)?);
                }
                _ => {
                    return Err(Error::new_spanned(
                        item,
                        "Only function items are allowed in a dispatch trait",
                    ));
                }
            }
        }

        Ok(PreprocessedCgpAutoDispatch {
            item_trait: self.item_trait.clone(),
            methods,
        })
    }
}
