# `PathCons`

`PathCons<Head, Tail>` is the type-level path spine — a recursive list of segments, where both the head and the tail may be unsized — that CGP uses to address an entry deep inside a delegation table.

## Purpose

`PathCons` exists to express a *route* through nested delegation tables as a single type. Where a bare component name picks one entry out of a context's table, CGP sometimes needs to point at an entry that lives behind one or more layers of indirection — inside a namespace, behind another namespace it inherits from, under a prefix. A path is the type that names such a route: a list of segments read left to right, each segment narrowing the lookup one step further. `PathCons` is the cons cell of that list, and [`Nil`](cons.md) terminates it, so `PathCons<A, PathCons<B, Nil>>` is the two-step path "first `A`, then `B`."

A path is a distinct spine from the [`Cons`](cons.md) product list even though both are right-nested and `Nil`-terminated, and the difference is the unsized bound. `PathCons` declares `Head: ?Sized` and `Tail: ?Sized`, which lets a path segment be a trait object or other unsized type and lets the whole path be manipulated without requiring its parts to have a known size. A product list keys a struct's fields and its elements are always sized values; a path keys a lookup and its segments are pure type-level markers that never need to be `Sized`.

The segments themselves are the same markers CGP uses elsewhere: a lowercase dotted name becomes a [`Symbol`](chars.md) type-level string, and a capitalized name becomes that named type (typically a component key such as `FooProviderComponent` or a namespace marker). A path is therefore an interleaving of symbols and component names — the form `@a.B.c` — assembled into a `PathCons` chain. Paths are written with the [`Path!`](../macros/path.md) macro rather than spelled by hand, so this document describes the runtime spine; the `@`-segment syntax and the expansion live in that macro's document.

## Definition

`PathCons` is a zero-sized struct holding two `PhantomData` markers, one for the head segment and one for the rest of the path:

```rust
pub struct PathCons<Head: ?Sized, Tail: ?Sized>(pub PhantomData<Head>, pub PhantomData<Tail>);
```

The `Head` is the first segment of the path and the `Tail` is the remainder, expected to be either another `PathCons` or `Nil` at the end. Both bounds are `?Sized` so that any type — sized or not — can occupy a segment. The struct carries no runtime data; like the other type-level building blocks it exists purely so that a route can be named and matched in trait resolution.

## Behavior

`PathCons` participates in path concatenation through the [`ConcatPath`](../traits/static_format.md) trait, which appends one path onto the end of another at the type level. The trait recurses down the spine: `PathCons<Head, Tail>` concatenates with `Other` by keeping `Head` and concatenating `Tail` with `Other`, while `Nil` concatenates with `Other` by simply becoming `Other`. The result is the expected behavior of list append, computed entirely as an associated-type projection:

```rust
pub trait ConcatPath<Other: ?Sized> {
    type Output: ?Sized;
}

impl<Head: ?Sized, Tail: ?Sized, Other: ?Sized> ConcatPath<Other> for PathCons<Head, Tail>
where
    Tail: ConcatPath<Other>,
{
    type Output = PathCons<Head, <Tail as ConcatPath<Other>>::Output>;
}

impl<Other: ?Sized> ConcatPath<Other> for Nil {
    type Output = Other;
}
```

Beyond concatenation, a `PathCons` path is consumed by [`RedirectLookup`](../providers/redirect_lookup.md), the provider that resolves a delegation by walking a context's table along a path. When a namespace or a prefixed component re-routes a lookup, it does so by producing a `RedirectLookup<Components, Path>` whose `Path` is a `PathCons` chain; `RedirectLookup` follows the chain segment by segment until it lands on a concrete provider. The path itself never names a provider — it only describes where to look — so the same path can resolve to different providers depending on the table it is walked against.

## Examples

Paths are produced by the [`Path!`](../macros/path.md) macro and most often appear inside the wirings emitted by [`#[cgp_namespace]`](../macros/cgp_namespace.md). A namespace entry that redirects one component key to a path desugars into a `RedirectLookup` over a `PathCons` chain:

```rust
use cgp::prelude::*;

cgp_namespace! {
    new MyNamespace {
        FooProviderComponent =>
            @MyFooComponent,
    }
}

// the emitted entry, in readable form:
// impl<__Table__> MyNamespace<__Table__> for FooProviderComponent {
//     type Delegate = RedirectLookup<__Table__, PathCons<MyFooComponent, Nil>>;
// }
```

A path with both a lowercase symbol segment and a capitalized component segment interleaves the two marker kinds. Registering a component into a namespace under a prefix produces a two-segment path:

```rust
// @MyBarComponent.BarProviderComponent  expands to
// PathCons<MyBarComponent, PathCons<BarProviderComponent, Nil>>
```

Here the lookup steps first through `MyBarComponent` and then through `BarProviderComponent` before resolving. A single-segment path is `PathCons<Segment, Nil>`, and the empty path is `Nil` alone.

## Related constructs

`PathCons` is the routing counterpart to the product spine [`Cons`](cons.md)/`Nil`; it shares the right-nested, `Nil`-terminated shape but its segments are `?Sized` markers rather than sized field values. Its segments are [`Symbol`](chars.md) type-level strings (for lowercase names) and named component or namespace types (for capitalized names). Paths are built by the [`Path!`](../macros/path.md) macro, appended through [`ConcatPath`](../traits/static_format.md), and walked by [`RedirectLookup`](../providers/redirect_lookup.md) when resolving a delegation. They are produced throughout [`#[cgp_namespace]`](../macros/cgp_namespace.md), which uses them to reroute namespace entries and to register prefixed components.

## Source

The runtime type is defined in [crates/core/cgp-base-types/src/types/path.rs](../../../crates/core/cgp-base-types/src/types/path.rs) (`PathCons<Head: ?Sized, Tail: ?Sized>`), with `Nil` in [crates/core/cgp-base-types/src/types/nil.rs](../../../crates/core/cgp-base-types/src/types/nil.rs). The `ConcatPath` trait and its impls are in [crates/core/cgp-base-types/src/traits/concat_path.rs](../../../crates/core/cgp-base-types/src/traits/concat_path.rs). The constructing macro is [`Path!`](../macros/path.md) ([crates/macros/cgp-macro-lib/src/path.rs](../../../crates/macros/cgp-macro-lib/src/path.rs)), whose fold over the segments lives in [crates/macros/cgp-macro-core/src/types/path/unipath.rs](../../../crates/macros/cgp-macro-core/src/types/path/unipath.rs). `RedirectLookup`, which consumes a path at resolution time, is in [crates/core/cgp-component/src/providers/redirect_lookup.rs](../../../crates/core/cgp-component/src/providers/redirect_lookup.rs).
