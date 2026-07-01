# `MRef`

`MRef<'a, T>` is a "maybe-reference" — an enum that holds either a borrow of a `T` or an owned `T` — so a getter can return whichever it has without committing every implementor to one or the other.

## Purpose

`MRef` exists to let a single getter signature accommodate both the context that already stores a value and the context that must produce one on the fly. A getter that returns `&'a T` forces every context to keep a `T` it can lend out; a getter that returns `T` forces every context to hand over ownership, cloning even when it has a perfectly good reference to share. `MRef<'a, T>` removes that dilemma by being either case at runtime: a context with the value in a field returns `MRef::Ref` and lends it, while a context that computes or assembles the value returns `MRef::Owned` and gives it away. The caller treats both uniformly because `MRef` derefs to `T`.

The type earns its keep in CGP's getter machinery, where the return type chosen for a getter method decides what body the macro generates. When a getter is declared to return `MRef<'a, T>` and takes `&self`, the generated field accessor wraps the borrowed field as `MRef::Ref(...)`, so the common case — reading a stored field — costs nothing extra, while the same interface still permits a provider elsewhere to return an owned value. This is what lets a getter abstract over "do I have this value, or do I make it?" without splitting into two traits.

## Definition

`MRef` is a two-variant enum parameterized by a lifetime and an element type:

```rust
pub enum MRef<'a, T> {
    Ref(&'a T),
    Owned(T),
}
```

The `Ref` variant borrows a `T` for the lifetime `'a`; the `Owned` variant carries a `T` by value. The lifetime applies only to the borrowed case, so an `MRef` built from an owned value is effectively unbounded in `'a`. The enum is an ordinary owned value — there is nothing type-level about it — and it is the runtime payload a getter passes back to its caller.

## Behavior

`MRef` behaves like a smart pointer to `T`, which is what makes the two variants interchangeable at the call site. It implements `Deref<Target = T>` by matching on the variant and returning a `&T` either way, so `&*my_ref` and any method call that auto-derefs work regardless of which case is inside. It also implements `AsRef<T>` over the same logic, giving an explicit `as_ref()` for code that prefers it.

Constructing an `MRef` is frictionless because it implements `From` in both directions: `From<T>` builds the `Owned` variant and `From<&'a T>` builds the `Ref` variant, so a value or a reference converts with `.into()`. When a caller needs to take ownership unconditionally, `get_or_clone` resolves the enum to a plain `T` — returning the owned value as is, or cloning the borrowed one — and is available whenever `T: Clone`. These three pieces — transparent `Deref`/`AsRef`, the two `From` impls, and `get_or_clone` — are the whole surface; a borrowed `MRef` is read cheaply and promoted to ownership only when explicitly asked.

## Examples

`MRef` is used as the return type of a getter that should work whether the context stores the value or produces it. The following getter reads a borrowed field and hands it back as a borrowing `MRef`:

```rust
use cgp::prelude::*;

let stored = String::from("hello");

// a context lending a stored value:
let borrowed: MRef<'_, String> = MRef::from(&stored);
assert_eq!(&*borrowed, "hello");

// a provider returning a freshly built value through the same type:
let made: MRef<'_, String> = MRef::from(String::from("world"));
assert_eq!(made.as_ref(), "world");

// promote either to an owned value when ownership is required:
let owned: String = borrowed.get_or_clone();
assert_eq!(owned, "hello");
```

Both `borrowed` and `made` have the same type and are consumed the same way; only the construction differs, and `get_or_clone` clones the borrowed case while moving the owned one.

## Related constructs

`MRef` is one of the getter return modes recognized by the field-getter macros: a getter declared to return `MRef<'a, T>` over `&self` generates a borrowing accessor, parallel to how returning `&T`, `Option<&T>`, or `&str` selects other accessor shapes. It is therefore commonly seen with [`#[cgp_getter]`](../macros/cgp_getter.md) and the [`HasField`](../traits/has_field.md) access it builds on, and with the [`UseField`](../providers/use_field.md) provider that wires those getters. Its lifetime is an ordinary borrow lifetime and is unrelated to the type-level lifetime lift [`Life`](life.md), which serves a different purpose in provider wiring.

## Source

- The type is defined in [crates/core/cgp-field/src/types/mref.rs](../../../crates/core/cgp-field/src/types/mref.rs), including its `Deref`, `AsRef`, the two `From` impls, and `get_or_clone`.
- The macro logic that recognizes an `MRef<'a, T>` getter return type and emits an `MRef::Ref(...)` body is in [crates/macros/cgp-macro-core/src/functions/field/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/field/parse.rs) (the `MRef` field mode) and [crates/macros/cgp-macro-core/src/types/getter/get_field_with_mode_expr.rs](../../../crates/macros/cgp-macro-core/src/types/getter/get_field_with_mode_expr.rs).
