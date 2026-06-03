use syn::visit_mut::VisitMut;
use syn::{ItemImpl, ItemTrait};

use crate::types::attributes::UseTypeAttribute;
use crate::visitors::SubstituteAbstractType;

#[derive(Default)]
pub struct UseTypeAttributes {
    pub attributes: Vec<UseTypeAttribute>,
}

impl UseTypeAttributes {
    pub fn substitute_abstract_types_in_item_trait(&self, item_trait: &mut ItemTrait) {
        for type_spec in self.attributes.iter().rev() {
            SubstituteAbstractType { type_spec }.visit_item_trait_mut(item_trait);
        }
    }

    pub fn substitute_abstract_types_in_item_impl(&self, item_impl: &mut ItemImpl) {
        for type_spec in self.attributes.iter().rev() {
            SubstituteAbstractType { type_spec }.visit_item_impl_mut(item_impl);
        }
    }

    // pub fn transform_item_impl(&self, item_impl: &mut ItemImpl) {

    // }
}
