use quote::ToTokens;
use syn::visit_mut::VisitMut;
use syn::{ItemImpl, ItemTrait, Type};

use crate::functions::parse_internal;
use crate::types::attributes::UseTypeAttribute;
use crate::types::attributes::use_type::type_predicates::{
    derive_use_type_predicates, forbid_duplicate_aliases,
};
use crate::visitors::SubstituteAbstractTypes;

#[derive(Default, Clone)]
pub struct UseTypeAttributes {
    pub attributes: Vec<UseTypeAttribute>,
}

impl UseTypeAttributes {
    /// Resolve every spec's context type into fully-qualified form before it is
    /// used, so that both the body substitution and the appended bounds agree on
    /// one grounded context.
    ///
    /// An `in Context` suffix whose `Context` is itself imported by another spec —
    /// as in `#[use_type(HasTypes.Types, HasScalarType.Scalar in Types)]` — is
    /// rewritten from the bare alias `Types` to `<Self as HasTypes>::Types`.
    /// Contexts that name a real generic parameter or `Self` are left untouched.
    /// The pass iterates to a fixpoint so a chain of links resolves fully; each
    /// pass grounds one more level, so `attributes.len()` passes cover any
    /// acyclic chain, and a cyclic reference simply stops making progress and
    /// surfaces later as an ordinary unresolved-type error rather than looping.
    fn grounded_specs(&self) -> Vec<UseTypeAttribute> {
        let mut grounded = self.attributes.clone();

        for _ in 0..grounded.len() {
            let snapshot = grounded.clone();
            let mut changed = false;

            for spec in grounded.iter_mut() {
                let mut visitor = SubstituteAbstractTypes::new(&snapshot);
                visitor.visit_type_mut(&mut spec.context_type);
                changed |= visitor.is_changed;
            }

            if !changed {
                break;
            }
        }

        grounded
    }

    pub fn transform_item_trait(&self, item_trait: &mut ItemTrait) -> syn::Result<()> {
        if self.attributes.is_empty() {
            return Ok(());
        }

        forbid_duplicate_aliases(&self.attributes)?;

        let grounded = self.grounded_specs();

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

        forbid_duplicate_aliases(&self.attributes)?;

        let grounded = self.grounded_specs();

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
