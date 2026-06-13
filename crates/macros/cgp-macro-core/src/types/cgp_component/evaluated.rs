use syn::{ItemImpl, ItemTrait};

use crate::types::attributes::CgpComponentAttributes;
use crate::types::cgp_component::CgpComponentArgs;
use crate::types::empty_struct::EmptyStruct;

pub struct EvaluatedCgpComponent {
    pub component_struct: EmptyStruct,
    pub consumer_trait: ItemTrait,
    pub consumer_impl: ItemImpl,
    pub provider_trait: ItemTrait,
    pub provider_impl: ItemImpl,
    pub args: CgpComponentArgs,
    pub attributes: CgpComponentAttributes,
}
