use quote::ToTokens;
use syn::visit_mut::VisitMut;
use syn::{ItemImpl, ItemTrait, parse_quote, parse2};

use crate::types::attributes::UseTypeAttribute;
use crate::types::attributes::use_type::type_predicates::derive_use_type_predicates;
use crate::visitors::SubstituteAbstractType;

#[derive(Default, Clone)]
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

    pub fn transform_item_trait(&self, item_trait: &mut ItemTrait) -> syn::Result<()> {
        if self.attributes.is_empty() {
            return Ok(());
        }

        self.substitute_abstract_types_in_item_trait(item_trait);

        for use_type in self.attributes.iter() {
            if use_type.context_type != parse_quote! { Self } {
                continue;
            }

            item_trait
                .supertraits
                .push(parse2(use_type.trait_path.to_token_stream())?);
        }

        Ok(())
    }

    pub fn transform_item_impl(&self, item_impl: &mut ItemImpl) -> syn::Result<()> {
        if self.attributes.is_empty() {
            return Ok(());
        }

        self.substitute_abstract_types_in_item_impl(item_impl);

        let predicates = derive_use_type_predicates(&self.attributes)?;

        item_impl
            .generics
            .make_where_clause()
            .predicates
            .extend(predicates);

        Ok(())
    }
}
