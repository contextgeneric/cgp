use quote::ToTokens;
use syn::visit_mut::VisitMut;
use syn::{ItemImpl, ItemTrait, Type};

use crate::functions::parse_internal;
use crate::types::attributes::UseTypeAttribute;
use crate::types::attributes::use_type::type_predicates::{
    derive_use_type_predicates, forbid_duplicate_aliases, forbid_grounding_cycles,
};
use crate::visitors::SubstituteAbstractTypes;

#[derive(Default, Clone)]
pub struct UseTypeAttributes {
    pub attributes: Vec<UseTypeAttribute>,
}

impl UseTypeAttributes {
    /// Resolve every spec's groundable positions into fully-qualified form before
    /// they are used, so that the body substitution and the appended bounds agree
    /// on one grounded context and one grounded trait path.
    ///
    /// Grounding reaches both positions named by
    /// [`UseTypeAttribute::groundable_types`], because an alias left bare in
    /// either would end up inside an emitted `<Context as Trait<Args…>>::Assoc`
    /// path, resolving to nothing. An `in Context` suffix whose `Context` is
    /// itself imported by another spec — as in
    /// `#[use_type(HasTypes.Types, HasScalarType.Scalar in Types)]` — is rewritten
    /// from the bare alias `Types` to `<Self as HasTypes>::Types`, and a trait
    /// argument that names an alias is grounded the same way, so
    /// `#[use_type(HasDbType.Db, HasPoolType<Db>.Pool)]` projects against
    /// `HasPoolType<<Self as HasDbType>::Db>`. A position naming a real generic
    /// parameter or `Self` is left untouched.
    ///
    /// The pass iterates to a fixpoint so a chain of links resolves fully. Each
    /// pass grounds one more level of the dependency chain, so `attributes.len()`
    /// passes cover any acyclic graph over that many specs — which is every graph
    /// that reaches here, since `forbid_grounding_cycles` has already rejected the
    /// cyclic ones.
    fn grounded_specs(&self) -> Vec<UseTypeAttribute> {
        let mut grounded = self.attributes.clone();

        for _ in 0..grounded.len() {
            let snapshot = grounded.clone();
            let mut changed = false;

            for spec in grounded.iter_mut() {
                let mut visitor = SubstituteAbstractTypes::new(&snapshot);

                for ty in spec.groundable_types_mut() {
                    visitor.visit_type_mut(ty);
                }

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
        forbid_grounding_cycles(&self.attributes)?;

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
        forbid_grounding_cycles(&self.attributes)?;

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
