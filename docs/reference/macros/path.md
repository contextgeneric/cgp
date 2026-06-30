# `Path!`

`Path!(@a.B.c)` is the type macro that builds a type-level path — a `PathCons` list of segments naming a route through nested delegation tables — from a dotted, `@`-prefixed sequence of names.

## Purpose

`Path!` exists to give a readable surface syntax for the `PathCons` spine that CGP uses to address an entry behind layers of delegation. A path names a route, read left to right, where each segment narrows a lookup one step: through a namespace, through a prefix, down to a component key. Writing that route as a nested `PathCons<…, PathCons<…, Nil>>` by hand is unwieldy and obscures the intent, so `Path!` lets it be written the way it reads — a dotted name like `@app.error.ErrorRaiserComponent` — and folds it into the corresponding spine.

The macro is the path-shaped sibling of the other type-level construction macros. Where [`Symbol!`](symbol.md) turns a string literal into a single type-level string and [`Product!`](product.md)/`Sum!` build product and sum lists, `Path!` builds the routing list, sharing their right-nested, `Nil`-terminated shape. It is the construction half of the [`PathCons`](../types/path_cons.md) type, and the same `@`-path syntax it accepts is embedded directly inside [`#[cgp_namespace]`](cgp_namespace.md) entries and `#[prefix(...)]` attributes, which is where paths are most often written.

## Syntax

`Path!` takes a single `@`-prefixed path made of one or more segments separated by dots. The leading `@` is required — it is the sigil that marks the body as a path rather than a plain type — and at least one segment must follow it:

```rust
Path!(@app)
Path!(@app.error)
Path!(@app.error.ErrorRaiserComponent)
```

Each segment is parsed as a type, and its first character decides how it is encoded. A segment that is a single identifier beginning with a lowercase letter — and is not a primitive type name — becomes a [`Symbol`](../types/chars.md) type-level string of that identifier; every other segment is kept as the named type it spells. So lowercase segments such as `app` and `error` become symbols, while capitalized segments such as `ErrorRaiserComponent` become references to those types (typically component keys or namespace markers). The exception for primitives means a lowercase name like `u32`, `bool`, `usize`, or `str` is treated as the named primitive type, not as a symbol.

This is the same convention namespaces describe for their `@`-paths: dotted lowercase segments are field-name-style symbols and capitalized segments are named types. Mixing the two is normal — a path like `@my_app.ShowImplComponent` interleaves a symbol segment and a component segment.

## Syntax Grammar

The input to `Path!` is a leading `@` followed by one or more dot-separated segments:

```ebnf
PathInput   -> `@` PathSegment ( `.` PathSegment )*

PathSegment -> Type
```

The leading `` `@` `` is required and at least one segment must follow. Each `PathSegment` is parsed as a Rust `Type`, but its encoding is decided semantically (see Expansion): a single lowercase identifier that is not a primitive type name becomes a `Symbol` type-level string, while every other segment — a capitalized name or a primitive — is kept as the named type. This same `@`-path grammar is what [`#[cgp_namespace]`](cgp_namespace.md) entries and `#[prefix(...)]` attributes embed, where it appears as the `Path` production.

## Expansion

`Path!` expands to a right-nested chain of [`PathCons`](../types/path_cons.md) terminated by `Nil`, with each segment encoded by the lowercase/capitalized rule. A three-segment path with one lowercase symbol and two named types desugars as follows:

```rust
// before
Path!(@app.error.ErrorRaiserComponent)
```

```rust
// after — readable form
PathCons<
    Symbol!("app"),
    PathCons<
        Symbol!("error"),
        PathCons<ErrorRaiserComponent, Nil>,
    >,
>
```

The macro builds this by parsing the segments after the `@` into a list and folding them from right to left onto `Nil`, wrapping each segment in a `PathCons` whose tail is the accumulated rest. A single-segment path therefore becomes `PathCons<Segment, Nil>`, and because a lowercase symbol segment is itself a `Symbol`/`Chars`/`Nil` chain, the fully desugared form of `@app` is `PathCons<Symbol<3, Chars<'a', Chars<'p', Chars<'p', Nil>>>>, Nil>`.

The same expansion appears verbatim inside the wirings that embed `@`-paths. A [`#[cgp_namespace]`](cgp_namespace.md) redirect entry `FooProviderComponent => @MyFooComponent` produces a `RedirectLookup<__Table__, PathCons<MyFooComponent, Nil>>`, and a `#[prefix(@MyBarComponent in MyNamespace)]` attribute produces a `PathCons<MyBarComponent, PathCons<BarProviderComponent, Nil>>` — the macro's fold is the same one driving those constructs.

## Examples

A path is typically written to express a redirect target. Used directly as a type, `Path!` names a route that a [`RedirectLookup`](../providers/redirect_lookup.md) can resolve against a table:

```rust
use cgp::prelude::*;

type ErrorRoute = Path!(@app.error.ErrorRaiserComponent);
// ErrorRoute = PathCons<Symbol!("app"),
//                  PathCons<Symbol!("error"),
//                      PathCons<ErrorRaiserComponent, Nil>>>
```

In practice the same syntax is more often embedded in a namespace table than written through the bare macro, since [`#[cgp_namespace]`](cgp_namespace.md) accepts `@`-paths directly in its entries:

```rust
use cgp::prelude::*;

cgp_namespace! {
    new MyNamespace {
        FooProviderComponent =>
            @MyFooComponent,
    }
}
// the entry's Delegate is RedirectLookup<__Table__, PathCons<MyFooComponent, Nil>>
```

Either way the path is the same `PathCons` list; the macro and the namespace simply offer two places to write it.

## Related constructs

`Path!` constructs the [`PathCons`](../types/path_cons.md) spine, the type it desugars to, and its lowercase segments are [`Symbol!`](symbol.md) type-level strings. It is the routing-list counterpart to the product and sum construction macros [`Product!`](product.md) and [`Sum!`](sum.md), sharing their right-fold-onto-`Nil` shape. The paths it builds are consumed by [`RedirectLookup`](../providers/redirect_lookup.md) when resolving a delegation, and its `@`-path syntax is embedded throughout [`#[cgp_namespace]`](cgp_namespace.md), where namespace entries and `#[prefix(...)]` attributes use the same dotted form.

## Source

The macro entry point is `Path` in [crates/macros/cgp-macro-lib/src/path.rs](../../../crates/macros/cgp-macro-lib/src/path.rs), which parses the body into a `UniPath` and emits its tokens. The parsing and codegen live in [crates/macros/cgp-macro-core/src/types/path/](../../../crates/macros/cgp-macro-core/src/types/path/): `unipath.rs` requires the leading `@`, parses the dot-separated segments, and right-folds them with `PathCons` onto `Nil`; `path_element.rs` decides per segment whether a lowercase, non-primitive identifier becomes a `Symbol` or the segment stays a named type. The runtime spine `PathCons` is defined in [crates/core/cgp-base-types/src/types/path.rs](../../../crates/core/cgp-base-types/src/types/path.rs), and the `RedirectLookup` provider that walks a path is in [crates/core/cgp-component/src/providers/redirect_lookup.rs](../../../crates/core/cgp-component/src/providers/redirect_lookup.rs). The `@`-path forms are exercised by the namespace snapshot tests in [crates/tests/cgp-tests/tests/namespace_tests/namespace_macro/](../../../crates/tests/cgp-tests/tests/namespace_tests/namespace_macro/).
