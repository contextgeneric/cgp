use syn::spanned::Spanned;
use syn::{Generics, ItemImpl, Type};

use crate::exports::{DelegateComponent, IsProviderFor};
use crate::functions::{merge_generics, override_span, parse_internal};

/// The flat form every key, value, and statement collapses to; it renders the
/// impl pair for one wiring entry.
pub struct EvaluatedDelegateEntry {
    pub table_type: Type,
    pub generics: Generics,
    pub key: Type,
    pub value: Type,
}

/// Lower a construct into a single evaluated entry.
pub trait EvalDelegateEntry {
    fn eval_entry(&self, table_type: &Type) -> syn::Result<EvaluatedDelegateEntry>;
}

/// Lower a construct into a flat list of evaluated entries (a key or statement
/// may expand to several).
pub trait EvalDelegateEntries {
    fn eval_entries(&self, table_type: &Type) -> syn::Result<Vec<EvaluatedDelegateEntry>>;
}

impl EvaluatedDelegateEntry {
    /// Emit `impl DelegateComponent<Key> for TableType { type Delegate = Value; }`.
    pub fn build_delegate_component_impl(
        &self,
        outer_generics: &Generics,
    ) -> syn::Result<ItemImpl> {
        let table_type = &self.table_type;

        let generics = merge_generics(outer_generics, &self.generics);

        let key = &self.key;
        let value = &self.value;

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let item_impl = parse_internal! {
            impl #impl_generics
                #DelegateComponent< #key >
                for #table_type
            #where_clause
            {
                type Delegate = #value;
            }
        };

        self.respan_impl(item_impl)
    }

    /// Emit the forwarding `IsProviderFor<Key, __Context__, __Params__>` impl,
    /// bounded on the value being a provider for the same key so a missing
    /// transitive dependency stays diagnosable.
    pub fn build_is_provider_for_impl(&self, outer_generics: &Generics) -> syn::Result<ItemImpl> {
        let table_type = &self.table_type;

        let mut generics = merge_generics(outer_generics, &self.generics);

        let key = &self.key;
        let value = &self.value;

        generics.params.push(parse_internal!(__Context__));
        generics.params.push(parse_internal!(__Params__));

        generics
            .make_where_clause()
            .predicates
            .push(parse_internal! {
                #value: #IsProviderFor<#key, __Context__, __Params__>
            });

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let item_impl = parse_internal! {
            impl #impl_generics
                #IsProviderFor< #key, __Context__, __Params__ >
                for #table_type
            #where_clause
            {}
        };

        self.respan_impl(item_impl)
    }

    /// Re-span a generated impl onto the entry's key so a coherence conflict
    /// (`E0119`) between two entries mapping the same key is reported on the
    /// offending entry rather than on the whole `delegate_components!` block —
    /// the impl's `impl`/trait-ref/self-type tokens otherwise carry the macro's
    /// `call_site` span, which spans the entire invocation.
    ///
    /// The impl's generics keep their original spans, restored after re-spanning,
    /// so a diagnostic about a per-entry generic (an unconstrained parameter's
    /// `E0207`, say) still points at the generic the user wrote rather than the key.
    fn respan_impl(&self, item_impl: ItemImpl) -> syn::Result<ItemImpl> {
        let generics = item_impl.generics.clone();
        let mut item_impl = override_span(self.key.span(), &item_impl)?;
        item_impl.generics = generics;

        Ok(item_impl)
    }

    /// Emit `impl namespace_trait for Key { type Delegate = Value; }`, used by
    /// the namespace preset machinery instead of a direct `DelegateComponent` impl.
    pub fn build_namespace_impl(
        &self,
        namespace_trait: &Type,
        outer_generics: &Generics,
    ) -> syn::Result<ItemImpl> {
        let generics = merge_generics(outer_generics, &self.generics);

        let key = &self.key;
        let value = &self.value;

        let (impl_generics, _, where_clause) = generics.split_for_impl();

        let item_impl = parse_internal! {
            impl #impl_generics
                #namespace_trait
                for #key
            #where_clause
            {
                type Delegate = #value;
            }
        };

        Ok(item_impl)
    }
}
