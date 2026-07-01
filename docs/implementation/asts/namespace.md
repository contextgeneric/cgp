# The `namespace` AST stack

The `namespace` stack is the pair of AST types that `cgp_namespace!` parses into and renders from — `NamespaceTable`, which holds the parsed header and entries and carries all the derivation logic, and `EvaluatedNamespaceTable`, the bag of finished `syn` items it produces. A supporting type, `InheritNamespaceStatement`, lowers a parent-namespace clause into the inheritance entry. The data flows in one direction: the macro body parses into `NamespaceTable`, whose `eval` builds an `EvaluatedNamespaceTable` that a `ToTokens` impl renders. The [entrypoint document](../entrypoints/cgp_namespace.md) covers what each generated item looks like; this document covers the types.

## `NamespaceTable`

`NamespaceTable` is the parsed form of the whole macro body and the type that does the work. Its `Parse` impl reads, in order, an optional generic list, an optional `new` keyword, the namespace name (an identifier with optional type arguments), an optional `: parent` clause (parsed as a path with type arguments), and a brace-delimited entry table:

```rust
pub struct NamespaceTable {
    pub impl_generics: ImplGenerics,
    pub new: Option<Keyword<New>>,
    pub namespace: IdentWithTypeArgs,
    pub parent_namespace: Option<(Colon, PathWithTypeArgs)>,
    pub entries: DelegateEntries,
}
```

The `entries` field is a `DelegateEntries` — the same type `delegate_components!` parses its table into — so a namespace body accepts every mapping form, array key, and statement that a delegation table does. `NamespaceTable` carries a family of `build_*` methods, one per generated item: `build_item_trait` emits the `{Namespace}<__Table__>` lookup trait only when `new` is set, `build_namespace_struct` the `__{Namespace}Components` marker only when `new` is set, `build_item_impls` one impl per evaluated entry (each entry evaluated against the shared `__Table__` type), and `build_parent_namespace_impl` the inheritance impl when a parent is named. Its `eval` method runs all four and packages the results, inserting the inheritance impl ahead of the entry impls so it wins during resolution.

## `InheritNamespaceStatement`

`InheritNamespaceStatement` is the intermediary that turns a `: parent` clause into an inheritance entry; the user never writes it. It pairs the parent namespace path with the child's own marker-struct identifier, and its `eval_for_entry` builds a for-entry — the same `EvaluatedForEntry` shape `delegate_components!` uses — whose `where` clause bounds a key on the parent namespace over the child's local table. That for-entry then evaluates to the blanket impl that reads each parent entry and re-emits it under the child (the `__Key__`/`__Value__`/`DefaultNamespace` shape shown in the entrypoint document). Routing inheritance through the shared for-entry machinery is what keeps a namespace's parent lookup consistent with an ordinary delegation table's.

## `EvaluatedNamespaceTable`

`EvaluatedNamespaceTable` is the final stage — a bag holding the optional trait, the optional struct, and the vector of impls:

```rust
pub struct EvaluatedNamespaceTable {
    pub item_impls: Vec<ItemImpl>,
    pub item_trait: Option<ItemTrait>,
    pub item_struct: Option<ItemStruct>,
}
```

Its only behavior is a `ToTokens` impl that renders the items in a fixed order — struct first, then trait, then every impl — which is the order the canonical snapshots pin. Because the inheritance impl was inserted at the front of `item_impls` during `eval`, it renders ahead of the per-entry impls.

## Tests

- The stack is exercised end-to-end by the expansion snapshots and behavioral tests indexed in the [entrypoint document](../entrypoints/cgp_namespace.md), which pin each entry form (redirect, direct mapping, array key, inheritance, and path-prefix rewrite) and the wiring they produce.

## Source

- The stack lives in [cgp-macro-core/src/types/namespace/](../../../crates/macros/cgp-macro-core/src/types/namespace/): `NamespaceTable` and its `build_*`/`eval` methods in `table.rs`, `InheritNamespaceStatement` in `inherit.rs`, and `EvaluatedNamespaceTable` in `eval.rs`.
- The entry table and the for-entry machinery it reuses live in [cgp-macro-core/src/types/delegate_component/](../../../crates/macros/cgp-macro-core/src/types/delegate_component/).
- The entrypoint that drives the stack is documented in [entrypoints/cgp_namespace.md](../entrypoints/cgp_namespace.md).
