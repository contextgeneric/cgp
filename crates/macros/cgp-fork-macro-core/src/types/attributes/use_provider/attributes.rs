use syn::{Generics, Type};

use crate::traits::AddTypeParamBounds;
use crate::types::attributes::UseProviderAttribute;

#[derive(Default)]
pub struct UseProviderAttributes {
    pub attributes: Vec<UseProviderAttribute>,
}

impl AddTypeParamBounds for UseProviderAttributes {
    fn add_type_param_bounds(&self, self_type: &Type, generics: &mut Generics) -> syn::Result<()> {
        if self.attributes.is_empty() {
            return Ok(());
        }

        let where_clause = generics.make_where_clause();

        for use_provider in &self.attributes {
            let predicate = use_provider.to_provider_bounds(self_type)?;
            where_clause.predicates.push(predicate);
        }

        Ok(())
    }
}
