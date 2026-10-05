use cgp_fork_macro_core::exports::HasExtractor;
use cgp_fork_macro_core::functions::override_item_span;
use cgp_fork_macro_core::parse_internal;
use syn::{ItemImpl, ItemTrait};

use crate::types::cgp_auto_dispatch::{DispatchMethod, EvaluatedCgpAutoDispatch};

/// The checked `#[cgp_auto_dispatch]` input: the trait and one
/// [`DispatchMethod`] per method.
pub struct PreprocessedCgpAutoDispatch {
    pub item_trait: ItemTrait,
    pub methods: Vec<DispatchMethod>,
}

impl PreprocessedCgpAutoDispatch {
    /// Evaluate into the dispatch IR: the enum-level blanket impl, and one
    /// `#[cgp_computer]` input per method, left for the entrypoint to lower.
    pub fn eval(&self) -> syn::Result<EvaluatedCgpAutoDispatch> {
        Ok(EvaluatedCgpAutoDispatch {
            item_trait: self.item_trait.clone(),
            blanket_impl: self.to_blanket_impl()?,
            computers: self
                .methods
                .iter()
                .map(|method| method.to_computer(&self.item_trait))
                .collect::<syn::Result<_>>()?,
        })
    }

    /// The impl of the trait for any `__Variants__`, each method running a
    /// value-handler matcher over that method's per-variant computer.
    pub fn to_blanket_impl(&self) -> syn::Result<ItemImpl> {
        let item_trait = &self.item_trait;
        let trait_ident = &item_trait.ident;

        let mut generics = item_trait.generics.clone();
        // Insert the enum parameter as the leading impl generic. Position 0 is safe
        // with a lifetime present because `syn::Generics::to_tokens` emits
        // lifetimes first. See cgp-knowledge-base/cgp/implementation/README.md,
        // "Generic-parameter insertion and lifetime ordering".
        generics.params.insert(0, parse_internal!(__Variants__));

        let mut impl_items = Vec::new();

        for method in self.methods.iter() {
            let (impl_item, predicate) = method.to_blanket_impl_item()?;

            impl_items.push(impl_item);
            generics.make_where_clause().predicates.push(predicate);
        }

        generics
            .make_where_clause()
            .predicates
            .push(parse_internal!(__Variants__: #HasExtractor));

        // The impl must meet the trait's supertraits, which it can only require of
        // the enum: implementing the trait for every `__Variants__` would otherwise
        // need each supertrait to hold for every type.
        let supertraits = &item_trait.supertraits;
        if !supertraits.is_empty() {
            generics
                .make_where_clause()
                .predicates
                .push(parse_internal!(__Variants__: #supertraits));
        }

        let ty_generics = item_trait.generics.split_for_impl().1;
        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let blanket_impl: ItemImpl = parse_internal! {
            impl #impl_generics #trait_ident #ty_generics for __Variants__
                #where_clause
            {
                #(#impl_items)*
            }
        };

        // Re-span the impl's boundary tokens onto the trait name, so an error on
        // the blanket impl (a conflict with a hand-written impl of the trait, say)
        // points at the trait rather than the whole attribute. See
        // cgp-knowledge-base/cgp/implementation/README.md, "Spans".
        override_item_span(trait_ident.span(), &blanket_impl)
    }
}
