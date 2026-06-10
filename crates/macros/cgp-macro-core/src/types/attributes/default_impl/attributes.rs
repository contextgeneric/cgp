use syn::{Generics, ItemImpl, Type};

use crate::types::attributes::DefaultImplAttribute;

#[derive(Default)]
pub struct DefaultImplAttributes {
    pub attributes: Vec<DefaultImplAttribute>,
}

impl DefaultImplAttributes {
    pub fn to_item_impls(
        &self,
        provider_generics: &Generics,
        provider_type: &Type,
    ) -> syn::Result<Vec<ItemImpl>> {
        let mut item_impls = Vec::new();

        for attribute in &self.attributes {
            let item_impl = attribute.to_item_impl(provider_generics, provider_type)?;
            item_impls.push(item_impl);
        }

        Ok(item_impls)
    }
}
