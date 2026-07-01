# `#[cgp_impl]` — implementation

`#[cgp_impl]` lets a provider be written in consumer-trait clothing — an `impl Trait for Context` block that keeps `self`/`Self` — and lowers it into the provider-trait form that CGP requires, then hands that form to the same machinery [`#[cgp_provider]`](cgp_provider.md) drives. This document covers how the lowering works internally; for the accepted syntax and the full before/after expansion, read the reference document [reference/macros/cgp_impl.md](../../reference/macros/cgp_impl.md).

## Entry point

The macro is driven by the `cgp_impl` function in [cgp-macro-lib/src/cgp_impl.rs](../../../crates/macros/cgp-macro-lib/src/cgp_impl.rs). It parses the attribute into an `ImplArgs` (the optional `new` keyword, the provider type, and an optional `: ComponentType` override) and the item into a `syn::ItemImpl`, then runs the two-step lowering and emits the result:

```rust
let item_cgp_impl = ItemCgpImpl { args, item_impl };
let lowered = item_cgp_impl.lower()?;
let bare_impls = lowered.lower()?;
```

The first `lower` produces a `LoweredCgpImpl`; the second turns that into either a raw provider impl (fed through `#[cgp_provider]`) or an untouched consumer impl. The entry function emits `bare_impls` followed by any `default_impls` the block's `#[default_impl]` attributes generated. A malformed attribute is rejected while parsing `ImplArgs`, and applying the macro to a non-`impl` item fails at `syn::parse2::<ItemImpl>`.

## Pipeline

The macro moves through two lowering stages, each a method on the AST type the previous one produced; the [`cgp_impl` AST stack](../asts/cgp_impl.md) documents those types in full.

- **`ItemCgpImpl::lower`** processes the companion attributes and normalizes the impl header. It strips the CGP modifier attributes (`#[implicit]` arguments, `#[uses]`, `#[use_type]`, `#[use_provider]`, `#[default_impl]`) off the block, folds each into the impl generics or method bodies, and — when the `for Context` clause is omitted — inserts the reserved `__Context__` parameter so later stages always see an explicit context. It records the provider trait path and the context type on the resulting `LoweredCgpImpl`.
- **`LoweredCgpImpl::lower`** performs the consumer-to-provider rewrite: it swaps the provider type into the `Self` position, moves the context to the leading argument of the provider trait, rewrites `self`/`Self` with the [`replace_self` visitors](../asts/cgp_impl.md#the-replace_self-visitors), and hands the result to [`#[cgp_provider]`](cgp_provider.md)'s `ItemCgpProvider::lower`. The `#[cgp_impl(Self)]` special case skips the rewrite entirely and passes the block through unchanged.

## Generated items

For a normal provider, the macro emits exactly what `#[cgp_provider]` emits — the provider impl, its derived `IsProviderFor` impl, and (when `new` is set) the provider struct — plus one delegation impl per `#[default_impl]` attribute. The interesting work is upstream of that handoff: turning consumer-style syntax into the provider impl.

The lowering swaps the `Self` type to the provider, adds the context as the provider trait's leading type argument, and rewrites the receiver into a context parameter. A method that took `&self` becomes a static method taking the context by a snake-cased, double-underscored name derived from the context type:

```rust
// consumer-style input
#[cgp_impl(new ValueToString)]
impl<Context> FooProvider for Context {
    fn foo(&self, value: u32) -> String { value.to_string() }
}

// lowered provider impl handed to #[cgp_provider]
impl<Context> FooProvider<Context> for ValueToString {
    fn foo(__context__: &Context, value: u32) -> String { value.to_string() }
}
```

When the `for Context` clause is omitted, the inserted parameter is `__Context__`, so the same rewrite produces `__context__: &__Context__`. Both the explicit `Context` and the default `__Context__` snake-case to the same `__context__` receiver identifier.

## Behavior and corner cases

The **receiver identifier** is computed from the context type: if the context type is a bare identifier it is snake-cased and the result used as the parameter name, and any context type that is not a plain identifier falls back to the literal `__context__`. `Context` and `__Context__` both yield `__context__`. Every `self` in a body is rewritten to that identifier, every `Self` type to the context type, via the three `replace_self` visitors run in sequence.

A **`for Context` clause is optional**, and omitting it is the idiomatic form. When present, the `Self` type of the block *is* the context and the trait path is the provider trait; when absent, `ItemCgpImpl::lower` treats the block's `Self` type as the provider trait path and inserts `__Context__` at the front of the impl generics.

**Local associated types are preserved by name.** A `type Output = …` the block declares itself is collected before the rewrite so the `replace_self` type visitor leaves `Self::Output` alone rather than rewriting it to the context — only imported abstract types (`Self::Error` from `#[use_type]`) and receiver `self`/`Self` are rewritten.

The **companion attributes** are applied in `ItemCgpImpl::lower` before the provider rewrite: `#[implicit]` parameters are extracted from each signature and turned into `HasField` bounds on the context (and reads in the body), `#[uses(...)]` and `#[use_provider(...)]` add `Self` trait bounds to the impl generics, `#[use_type(...)]` imports an abstract type and rewrites its occurrences, and each `#[default_impl]` becomes a separate `DelegateComponent`-style impl emitted alongside the provider.

The **`#[cgp_impl(Self)]` passthrough** bypasses the whole rewrite: when the provider type is the literal `Self`, `LoweredCgpImpl::lower` returns the original `impl` block unchanged as an ordinary consumer-trait impl, so companion attributes still apply while the body keeps its `self` receiver. This form requires the `for Context` clause; omitting it is rejected with a spanned "Expected context type to be specified" error.

## Known issues

The `#[cgp_impl(Self)]` form requires an explicit `for Context` clause and errors cleanly when it is missing; there are no other known limitations specific to this macro beyond those inherited from [`#[cgp_provider]`](cgp_provider.md) (see its Known issues).

## Snapshots

Every `snapshot_cgp_impl!` invocation across the suite is indexed here, since these snapshots all belong to this entrypoint:

- [basic_delegation/provider_macro.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/provider_macro.rs) — the canonical plain expansion: `#[cgp_impl(new ValueToString)]` on an explicit `impl<Context> FooProvider for Context`, showing the `Self`-to-provider swap, the leading-context insertion, and `&self` becoming `__context__: &Context`.
- [implicit_arguments/cgp_impl_implicit.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_impl_implicit.rs) — `#[implicit]` arguments dropped from the signature and turned into `HasField` reads, with the implicit `__Context__` inserted (the `for` clause omitted).
- [higher_order_providers/use_provider_impl.rs](../../../crates/tests/cgp-tests/tests/higher_order_providers/use_provider_impl.rs) — a generic higher-order provider `ScaledArea<Inner>` with `#[use_provider]` completing the inner provider's bound.
- [namespaces/default_impls.rs](../../../crates/tests/cgp-tests/tests/namespaces/default_impls.rs) — several `#[cgp_impl]` blocks (`ShowString`, `ShowWithDisplay`, `ShowU32`) providing a generic component, exercising the `#[default_impl]` companion attribute alongside the provider rewrite.

The `#[cgp_impl(Self)]` bare-impl passthrough has no snapshot yet, and neither does a `#[cgp_impl]` carrying an explicit `: ComponentType` override.

## Tests

The behavioral tests confirm the lowered wiring works:

- [basic_delegation/provider_macro.rs](../../../crates/tests/cgp-tests/tests/basic_delegation/provider_macro.rs) wires `FooProviderComponent` to the generated `ValueToString` and checks the call resolves at run time.
- [implicit_arguments/cgp_impl_implicit.rs](../../../crates/tests/cgp-tests/tests/implicit_arguments/cgp_impl_implicit.rs) wires the implicit-argument provider through `delegate_and_check_components!` and confirms the field reads compute the area.
- [higher_order_providers/use_provider_impl.rs](../../../crates/tests/cgp-tests/tests/higher_order_providers/use_provider_impl.rs) wires the scaled higher-order provider onto a context and runs it.

## Source

- Entry point: `cgp_impl` in [cgp-macro-lib/src/cgp_impl.rs](../../../crates/macros/cgp-macro-lib/src/cgp_impl.rs).
- Lowering stages and their AST types: [cgp-macro-core/src/types/cgp_impl/](../../../crates/macros/cgp-macro-core/src/types/cgp_impl/), documented in [asts/cgp_impl.md](../asts/cgp_impl.md).
- `self`/`Self` rewriting: the `replace_self` visitors in [cgp-macro-core/src/visitors/replace_self/](../../../crates/macros/cgp-macro-core/src/visitors/replace_self/).
- Handoff target — the provider impl and its `IsProviderFor` derivation: documented in [entrypoints/cgp_provider.md](cgp_provider.md) and [asts/cgp_provider.md](../asts/cgp_provider.md).
