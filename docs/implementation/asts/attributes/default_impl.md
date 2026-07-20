# `#[default_impl]` — the AST stack

`#[default_impl(@test.ShowImplComponent.u32 in ExtendedNamespace)]` on a `#[cgp_impl]` provider registers that provider as a namespace's default for one path. It is a modifier attribute collected by the impl host; this page covers its AST types and the registration impl it builds, and the shared collection mechanism lives in the [attribute-modifier overview](README.md). For the user-facing syntax and the namespace machinery it plugs into, read the reference document [`DefaultNamespace`](../../../reference/traits/default_namespace.md).

## `DefaultImplAttribute`

The attribute parses into a `DefaultImplAttribute` — a `key_type` (a `UniPathOrType`, so the key may be a path or a type), the `in` keyword, and a `namespace` path (a `PathWithTypeArgs`). Parsing reads the key type, the `in` token, and the namespace in order.

## `DefaultImplAttributes`

`DefaultImplAttributes` is the collection wrapper — a `Vec<DefaultImplAttribute>` — that `CgpImplAttributes` fills, one entry per `#[default_impl]` attribute on the provider block. Its `to_item_impls(provider_generics, provider_type)` maps each entry through `to_item_impl`, and the host (`#[cgp_impl]`) emits the resulting impls after the provider impl, using the provider's own generics and provider type.

## `to_item_impl` — the registration impl

`DefaultImplAttribute::to_item_impl(provider_generics, provider_type)` emits one impl of the namespace's lookup trait, keyed on the given path type, whose `Delegate` associated type is the provider being defined:

```rust
// #[default_impl(@test.ShowImplComponent.u32 in ExtendedNamespace)] on provider ShowU32:
impl<__Components__> ExtendedNamespace<__Components__>
for PathCons<Symbol!("test"), PathCons<ShowImplComponent, PathCons<u32, Nil>>>
{
    type Delegate = ShowU32;
}
```

The namespace path gains a trailing `__Components__` type argument and the impl generics gain a matching `__Components__` parameter, so the default is generic over any table the namespace is queried through. The impl is built from quasi-quoted tokens and then re-spanned onto the user-written key with [`override_item_span`](../../entrypoints/delegate_components.md#error-spans), so a coherence conflict (`E0119`) between two default impls for the same key is reported on the key inside `#[default_impl(Key in …)]` rather than on the whole `#[cgp_impl]` attribute; only the boundary moves, so the interior tokens stay navigable in an IDE.

## The dropped `where` clause

**The provider's `where` clause is deliberately dropped from this impl**, and this is the subtle correctness point of `to_item_impl`. It receives the provider impl's generics *after* `#[implicit]`/`#[uses]`/`#[use_type]`/`#[use_provider]` have pushed their `Self`-keyed impl-side bounds into it — a provider with `#[use_type(HasErrorType.Error)]`, for instance, arrives carrying `where Self: HasErrorType`. Those bounds belong on the provider's own impl and its `IsProviderFor`, never on this registration impl, whose only job is `type Delegate = Provider`. The registration impl's `Self` is the path key (`PathCons<..>`), so a retained `Self: HasErrorType` would demand `PathCons<..>: HasErrorType` — a bound that never holds — and silently break every context that joins the namespace. `to_item_impl` therefore clears `generics.where_clause` before splitting, keeping only the parameters that name the key and provider plus the `__Components__` table.

The one consequence of dropping the `where` clause is a limitation on generic providers: a provider whose *type* is generic, and whose parameter appears only in the `Delegate` associated type, would leave that parameter unconstrained once the clause is gone, so a per-component default is written for a concrete provider rather than a generic one.

## Tests

The behavioral and snapshot tests exercise the emitted impl, the wiring it enables, the `where`-clause drop, and the cross-crate orphan restriction:

- [namespaces/default_impls.rs](../../../../crates/tests/cgp-tests/tests/namespaces/default_impls.rs) pins the emitted namespace-default impl (`snapshot_cgp_impl!`), and [namespaces/default_impls_wiring.rs](../../../../crates/tests/cgp-tests/tests/namespaces/default_impls_wiring.rs) checks a context picks up the default.
- [namespaces/default_impl_use_type.rs](../../../../crates/tests/cgp-tests/tests/namespaces/default_impl_use_type.rs) pins that the registration impl carries no `where` clause when the provider has a `#[use_type]` dependency, and resolves such a provider through a context that joins the namespace.
- The cross-crate orphan restriction on a default is pinned by the `cargo-cgp` UI fixtures [`usability/wiring/orphan/default_impl_foreign_prefix_path.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/usability/wiring/orphan/default_impl_foreign_prefix_path.rs) (a *prefixed* component's foreign path key) and [`usability/wiring/orphan/default_impl_foreign_component.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/usability/wiring/orphan/default_impl_foreign_component.rs) (a foreign *unprefixed* component marker key), both the [orphan-rule violation](../../../errors/wiring/orphan-rule.md) class.

The duplicate-key conflict `#[cgp_impl]` defers to the compiler is covered on the host's own page — see [Failure modes in entrypoints/cgp_impl.md](../../entrypoints/cgp_impl.md#failure-modes).

## Source

- The `default_impl/` submodule in [cgp-macro-core/src/types/attributes/default_impl/](../../../../crates/macros/cgp-macro-core/src/types/attributes/default_impl/): `attribute.rs` holds `DefaultImplAttribute`, its parser, and `to_item_impl`; `attributes.rs` holds the `DefaultImplAttributes` collection and `to_item_impls`.
- Boundary re-spanning is [`override_item_span`](../../../../crates/macros/cgp-macro-core/src/functions/override_span.rs).
- The host that drives it: [entrypoints/cgp_impl.md](../../entrypoints/cgp_impl.md).
