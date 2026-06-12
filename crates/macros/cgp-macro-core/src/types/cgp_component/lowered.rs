use syn::ItemTrait;

use crate::types::cgp_component::CgpComponentArgs;
use crate::types::empty_struct::EmptyStruct;

pub struct LoweredCgpComponent {
    pub args: CgpComponentArgs,
    pub item_trait: ItemTrait,
}

impl LoweredCgpComponent {
    pub fn to_component_struct(&self) -> EmptyStruct {
        let component_name = &self.args.component_name;
        EmptyStruct {
            ident: component_name.ident.clone(),
            generics: component_name.type_generics.generics.clone(),
        }
    }
}
