use quote::ToTokens;
use syn::visit_mut::VisitMut;
use syn::{ItemImpl, ItemTrait, Type};

use crate::functions::parse_internal;
use crate::types::attributes::UseTypeAttribute;
use crate::types::attributes::use_type::grounding::ground_specs;
use crate::types::attributes::use_type::type_predicates::{
    derive_use_type_predicates, forbid_duplicate_aliases,
};
use crate::visitors::SubstituteAbstractTypes;

#[derive(Default, Clone)]
pub struct UseTypeAttributes {
    pub attributes: Vec<UseTypeAttribute>,
}

impl UseTypeAttributes {
    /// Validate the import list and resolve every spec into fully-qualified form,
    /// the two steps that must precede any substitution.
    ///
    /// Aliases are checked for uniqueness first, because [`ground_specs`] resolves a
    /// reference through the spec that owns the alias and so needs one owner per
    /// name; it then grounds each spec against its dependencies and rejects a cycle.
    fn resolved_specs(&self) -> syn::Result<Vec<UseTypeAttribute>> {
        forbid_duplicate_aliases(&self.attributes)?;

        ground_specs(&self.attributes)
    }

    pub fn transform_item_trait(&self, item_trait: &mut ItemTrait) -> syn::Result<()> {
        if self.attributes.is_empty() {
            return Ok(());
        }

        let grounded = self.resolved_specs()?;

        SubstituteAbstractTypes::new(&grounded).visit_item_trait_mut(item_trait);

        let self_type: Type = parse_internal! { Self };

        for use_type in grounded.iter() {
            let trait_path = &use_type.trait_path;

            if use_type.context_type == self_type {
                // A `Self`-context import becomes a supertrait of the generated
                // trait, so the abstract type is available to every signature.
                item_trait
                    .supertraits
                    .push(parse_internal(trait_path.to_token_stream())?);
            } else {
                // A foreign `in Context` import rewrites signatures to name
                // `<Context as Trait>::Assoc`, so the trait must require
                // `Context: Trait` for those paths to be well-formed. Without
                // this bound the constraint would be silently dropped, leaving a
                // signature that only compiles when `Context`'s bound happens to
                // be supplied elsewhere. The type-equality (`= T`) form is an
                // impl-side pin and is deliberately *not* added here.
                let context_type = &use_type.context_type;

                item_trait
                    .generics
                    .make_where_clause()
                    .predicates
                    .push(parse_internal! {
                        #context_type: #trait_path
                    });
            }
        }

        Ok(())
    }

    pub fn transform_item_impl(&self, item_impl: &mut ItemImpl) -> syn::Result<()> {
        if self.attributes.is_empty() {
            return Ok(());
        }

        let grounded = self.resolved_specs()?;

        SubstituteAbstractTypes::new(&grounded).visit_item_impl_mut(item_impl);

        let predicates = derive_use_type_predicates(&grounded)?;

        item_impl
            .generics
            .make_where_clause()
            .predicates
            .extend(predicates);

        Ok(())
    }
}
