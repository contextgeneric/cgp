use proc_macro2::Span;
use syn::{Generics, ItemImpl, Type};

use crate::exports::{DelegateComponent, IsProviderFor};
use crate::functions::{merge_generics, override_item_span, parse_internal};

/// The flat form every key, value, and statement collapses to; it renders the
/// impl pair for one wiring entry.
pub struct EvaluatedDelegateEntry {
    pub table_type: Type,
    pub generics: Generics,
    pub key: Type,
    pub value: Type,
    /// The span the entry's impls are re-spanned onto, so a coherence conflict is
    /// reported on the entry the user wrote. Sourced from the key's original token
    /// rather than the (possibly synthesized) `key` type — see `respan_impl`.
    pub span: Span,
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

    /// Re-span the generated impl onto the entry's own span (`self.span`, the
    /// key's source token) so a coherence conflict (`E0119`) between two entries
    /// mapping the same key is reported on the offending entry rather than on the
    /// whole `delegate_components!` block — the impl's `impl`/`{ … }` boundary
    /// otherwise carries the macro's `call_site` span, which spans the entire
    /// invocation. Using the carried span rather than `self.key.span()` keeps a
    /// synthesized key (an `@`-path's `PathCons<..>` nest, whose tokens are all
    /// `call_site`-spanned) pointing at what the user wrote.
    ///
    /// [`override_item_span`] moves only the impl's boundary tokens, so every
    /// interior token — the wired provider (`self.value`), the target type, a
    /// per-entry generic, and each synthesized reference like `IsProviderFor` —
    /// keeps its own span. That keeps the user's tokens navigable in an IDE
    /// (an editor maps by source range, so a synthesized reference re-spanned onto
    /// the key would hijack go-to-definition on that key) and keeps a per-entry
    /// generic's unconstrained-parameter `E0207` pointing at the `<T>` the user
    /// wrote rather than the key.
    fn respan_impl(&self, item_impl: ItemImpl) -> syn::Result<ItemImpl> {
        override_item_span(self.span, &item_impl)
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

        // Re-span like the `DelegateComponent`/`IsProviderFor` impls, so a
        // coherence conflict between two namespace entries mapping the same key
        // is reported on the offending entry rather than the whole
        // `cgp_namespace!` block — critical for a path key, whose synthesized
        // `PathCons<..>` key type otherwise carries the macro's `call_site` span.
        self.respan_impl(item_impl)
    }
}
