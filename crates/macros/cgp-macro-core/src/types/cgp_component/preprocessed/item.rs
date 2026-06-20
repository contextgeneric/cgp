use syn::ItemTrait;

use crate::types::attributes::CgpComponentAttributes;
use crate::types::cgp_component::{CgpComponentArgs, EvaluatedCgpComponent};
use crate::types::empty_struct::EmptyStruct;

pub struct PreprocessedCgpComponent {
    pub args: CgpComponentArgs,
    pub item_trait: ItemTrait,
    pub attributes: CgpComponentAttributes,
}

impl PreprocessedCgpComponent {
    pub fn to_component_struct(&self) -> EmptyStruct {
        let component_name = &self.args.component_name;
        EmptyStruct {
            ident: component_name.ident.clone(),
            generics: component_name.type_generics.to_generics(),
        }
    }

    pub fn eval(&self) -> syn::Result<EvaluatedCgpComponent> {
        let component_struct = self.to_component_struct();

        let (provider_trait, provider_impl) = self.to_provider_trait_and_blanket_impl()?;

        let consumer_impl = self.to_consumer_item_impl()?;

        Ok(EvaluatedCgpComponent {
            component_struct,
            provider_trait,
            provider_impl,
            consumer_impl,
            consumer_trait: self.item_trait.clone(),
            attributes: self.attributes.clone(),
            args: self.args.clone(),
        })
    }
}
