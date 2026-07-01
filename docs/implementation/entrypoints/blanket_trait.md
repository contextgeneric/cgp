# `#[blanket_trait]` — implementation

`#[blanket_trait]` turns a trait whose items carry default definitions into an extension trait by emitting the trait unchanged and generating the blanket impl that forwards those defaults, hiding the trait's supertraits behind its `where` clause. This document covers how that works internally; for the accepted syntax and the full expansion, read the reference document [reference/macros/blanket_trait.md](../../reference/macros/blanket_trait.md).

## Entry point

The macro is driven by the `blanket_trait` function in [cgp-macro-lib/src/blanket_trait.rs](../../../crates/macros/cgp-macro-lib/src/blanket_trait.rs). It parses the attribute argument as an optional context identifier — defaulting to the reserved `__Context__` when the attribute is empty — and the item as a `syn::ItemTrait`, then builds an `ItemBlanketTrait` and renders it directly.

```rust
let context_ident = if attr.is_empty() {
    Ident::new("__Context__", Span::call_site())
} else {
    parse2(attr)?
};
let item_blanket_impl = ItemBlanketTrait { context_ident, item_trait };
let items = item_blanket_impl.to_items()?;
```

Two failures surface here: a non-identifier attribute argument fails at `parse2`, and a non-trait item fails at `syn::parse2::<ItemTrait>`. Unlike `#[cgp_component]`, this macro has no multi-stage pipeline — the single `to_items` call does all the work.

## Pipeline

There is only one transform. `ItemBlanketTrait::to_items` emits two items: the trait, cloned verbatim from the input, and the blanket impl produced by `to_item_impl`. Because the trait is emitted unchanged, its declarations keep their default bodies as the user wrote them; the derivation happens only on the impl side. The [`blanket_trait` AST stack](../asts/blanket_trait.md) documents `ItemBlanketTrait`.

## Generated items

The macro emits the trait unchanged followed by one blanket impl over the context type. The impl is where all the derivation happens: it forwards each trait item's default into an impl item, requires the trait's supertraits on the context, and lifts each associated type into a fresh impl generic. The impl generics are the trait's own generics plus the context parameter plus one parameter per associated type, and the trait's supertraits become a `#context: <supertraits>` predicate.

For a method-carrying trait the impl copies each default method body verbatim, so every qualifying context inherits the body:

```rust
// #[blanket_trait] pub trait FooBar: Foo + Bar { fn foo_bar(&self) { self.foo(); self.bar(); } }

impl<__Context__> FooBar for __Context__
where
    __Context__: Foo + Bar,
{
    fn foo_bar(&self) { self.foo(); self.bar(); }
}
```

An **associated type** is handled specially, because lifting one out of a supertrait is the pattern's main use. Each associated type in the trait becomes a new generic parameter on the impl; a `RemoveSelfPathVisitor` pass rewrites `Self::FooBar` references (in the supertrait's associated-type equality, for instance) to name that free parameter; and the impl assigns the parameter back to the associated type:

```rust
// #[blanket_trait] pub trait HasFooTypeAtBar: HasFooTypeAt<Bar, Foo = Self::FooBar> { type FooBar; }

impl<__Context__, FooBar> HasFooTypeAtBar for __Context__
where
    __Context__: HasFooTypeAt<Bar, Foo = FooBar>,
{
    type FooBar = FooBar;
}
```

An **associated constant** is forwarded like a method: its default expression becomes the impl's constant definition.

## Behavior and corner cases

A **method or constant without a default** is a hard error: `to_item_impl` calls `ok_or_else` on the missing `default`, so the macro rejects it with a spanned `syn::Error` ("function item require implementation block" / "const item require implementation expression") rather than emitting a broken impl. An **associated type** needs no default, because the macro supplies the assignment itself from the lifted parameter.

A **bound on an associated type** is moved onto the lifted parameter in the impl's `where` clause: `type FooBar: Clone` adds `FooBar: Clone` alongside the supertrait predicate, so the blanket impl only applies when the underlying type is `Clone`. The bound is collected from the trait item before its `default` is stripped for the impl copy.

The **supertraits** of the trait become the hidden dependency: they are cloned into a single `#context: <supertraits>` `where`-predicate on the impl, which is what shields callers from naming `Foo + Bar`. The trait keeps its supertraits in the declaration too, since the trait is emitted unchanged.

Any trait item other than a type, method, or constant — a macro invocation, say — is rejected with an "unsupported trait item" error.

## Snapshots

Every `snapshot_blanket_trait!` invocation across the suite is indexed here; all live in the `blanket_traits` target:

- [blanket_traits/basic.rs](../../../crates/tests/cgp-tests/tests/blanket_traits/basic.rs) — the canonical minimal expansion: supertrait bounds only, no body, yielding an empty impl (a trait alias in all but name).
- [blanket_traits/with_method.rs](../../../crates/tests/cgp-tests/tests/blanket_traits/with_method.rs) — a default method body copied verbatim into the blanket impl.
- [blanket_traits/associated_type.rs](../../../crates/tests/cgp-tests/tests/blanket_traits/associated_type.rs) — a local associated type lifted into an impl generic and tied to a supertrait's associated type via `Foo = Self::FooBar`.
- [blanket_traits/associated_type_bounded.rs](../../../crates/tests/cgp-tests/tests/blanket_traits/associated_type_bounded.rs) — the same, plus a bound (`type FooBar: Clone`) moved onto the lifted parameter in the `where` clause.

Two variants have no snapshot yet: an associated *constant* forwarded from its default expression, and the `#[blanket_trait(Ctx)]` form overriding the default context identifier.

## Tests

The snapshot files double as behavioral tests:

- [blanket_traits/basic.rs](../../../crates/tests/cgp-tests/tests/blanket_traits/basic.rs) and the others each wire a concrete `Context` and assert through a `CanUse…` check trait that the generated blanket impl applies.
- No `cgp-macro-tests` failure case pins the "missing default body" error path, which is a candidate to add.

## Source

- Entry point: `blanket_trait` in [cgp-macro-lib/src/blanket_trait.rs](../../../crates/macros/cgp-macro-lib/src/blanket_trait.rs).
- Logic: [cgp-macro-core/src/types/blanket_trait.rs](../../../crates/macros/cgp-macro-core/src/types/blanket_trait.rs), documented in [asts/blanket_trait.md](../asts/blanket_trait.md).
- `Self::AssocType`-to-parameter rewriting: `RemoveSelfPathVisitor` in [cgp-macro-core/src/visitors/remove_self_path.rs](../../../crates/macros/cgp-macro-core/src/visitors/remove_self_path.rs).
- Fragment construction: [parse_internal!](../macros/parse_internal.md).
