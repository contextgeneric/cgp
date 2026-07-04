# `#[implicit]`

`#[implicit]` marks a function argument as an implicit dependency: instead of being passed by the caller, the value is read from a same-named field on the context, and the argument disappears from the public signature.

## Purpose

`#[implicit]` exists to make field-based dependency injection look like an ordinary function parameter. In plain CGP, a provider that needs a `width` value from its context declares a `HasField<Symbol!("width"), Value = f64>` bound in its `where` clause and calls `self.get_field(PhantomData)` inside the body. That works, but it forces the author to understand `HasField`, type-level symbols, and `PhantomData` tags before writing even the simplest provider. `#[implicit]` hides all of that behind a normal-looking parameter.

The argument named `width: f64` with `#[implicit]` reads as "this function needs a `width` of type `f64`," which is exactly the intuition a Rust programmer already has. The macro then does the mechanical work: it removes the argument from the signature, adds the matching `HasField` bound, and binds a local variable to the field value at the top of the body. The result is code that looks like a function taking arguments but behaves like a provider injecting dependencies from its context.

This is why `#[implicit]` is the recommended starting point for basic CGP. It lets a newcomer write providers in [`#[cgp_fn]`](../macros/cgp_fn.md) and [`#[cgp_impl]`](../macros/cgp_impl.md) using only familiar function syntax, deferring the `HasField` machinery until they actually need to understand it.

## Syntax

`#[implicit]` is written as a bare marker attribute on a typed function argument, and the argument must have a plain identifier name. It takes no arguments in any form — a list or name-value spelling such as `#[implicit(foo)]` or `#[implicit = "foo"]` is rejected with a spanned error rather than silently ignored:

```rust
fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}
```

The argument name doubles as the field name. Here `width` and `height` name both the local variables used in the body and the context fields the values are read from, via `Symbol!("width")` and `Symbol!("height")`. The argument type is the type the body sees, and it determines how the field is accessed (described under Expansion).

Three rules constrain where `#[implicit]` may appear. The function must take `self` as its first argument, because the field is read from `self`; a function with implicit arguments but no receiver is rejected. The argument pattern must be a bare identifier, not a destructuring or `mut` pattern — to get a mutable local, clone the injected value explicitly inside the body. And a *mutable* implicit argument — one whose type carries a `&mut`, whether the outer reference of a `&mut T`/`&mut [T]` or the inner reference of an `Option<&mut T>` — must be the *only* implicit argument on its function, and requires a `&mut self` receiver: it is read through `get_field_mut`, which borrows the whole context exclusively, so it cannot coexist with any other field read. Immutable implicit arguments carry no such restriction — they are shared borrows and combine freely, in any number, on either a `&self` or a `&mut self` receiver.

`#[implicit]` is usable wherever CGP rewrites function bodies into providers: inside [`#[cgp_fn]`](../macros/cgp_fn.md) and inside the methods of a [`#[cgp_impl]`](../macros/cgp_impl.md) block. It is not a standalone macro — it is only meaningful as an argument attribute consumed by those macros.

## Expansion

`#[implicit]` rewrites each marked argument into a `HasField` bound plus a `let` binding, leaving the rest of the function untouched. Starting from a `#[cgp_fn]` definition:

```rust
#[cgp_fn]
fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}
```

the macro produces a trait whose method takes no extra arguments, and an impl whose `where` clause carries one `HasField` bound per implicit argument:

```rust
pub trait RectangleArea {
    fn rectangle_area(&self) -> f64;
}

impl<Context> RectangleArea for Context
where
    Self: HasField<Symbol!("width"), Value = f64>
        + HasField<Symbol!("height"), Value = f64>,
{
    fn rectangle_area(&self) -> f64 {
        let width: f64 = self.get_field(PhantomData::<Symbol!("width")>).clone();
        let height: f64 = self.get_field(PhantomData::<Symbol!("height")>).clone();

        width * height
    }
}
```

The two `let` bindings are inserted at the top of the body in argument order, before any of the original statements, so the names are in scope for the rest of the function. The generated context type parameter is literally named `__Context__` in the emitted code; the examples here use `Context` for readability.

The access expression depends on the argument type, following the same rules as [`#[cgp_auto_getter]`](../macros/cgp_auto_getter.md). An owned type — a path type such as `f64` or `String`, or a tuple or array — is read by reference and `.clone()`d, so the body receives an owned value; a plain `&T` is taken by reference with no conversion. Four forms are special: `&str` is backed by a `String` field and read with `.as_str()`; `&[T]` reads any field whose value implements `AsRef<[T]>` and calls `.as_ref()`; `Option<&T>` reads an `Option<T>` field via `.as_ref()`; and `Option<&str>` reads an `Option<String>` field via `.as_deref()`. The mutability of the access follows the *argument's* own type, not the receiver's: an argument carrying a `&mut` reads through `HasFieldMut`/`get_field_mut`, while every immutable argument — a `&[T]` slice included — reads through `HasField`/`get_field` even on a `&mut self` receiver. Each reference form has a mutable mirror: a `&mut [T]` reads a field implementing `AsMut<[T]>` via `.as_mut()`, an `Option<&mut T>` reads an `Option<T>` field via `.as_mut()`, and an `Option<&mut str>` reads an `Option<String>` field via `.as_deref_mut()`. Every mutable form requires a `&mut self` receiver, as described under Syntax. Concretely:

```rust
#[cgp_fn]
fn greet(&self, #[implicit] name: &str) {
    println!("Hello, {}!", name);
}
```

expands so that the bound is `HasField<Symbol!("name"), Value = String>` and the binding is `let name: &str = self.get_field(PhantomData::<Symbol!("name")>).as_str();`. The field is a `String`, but the argument the body works with is a borrowed `&str`.

Inside a [`#[cgp_impl]`](../macros/cgp_impl.md) block the rewrite is identical — the same `HasField` bounds are added to the impl's `where` clause and the same `let` bindings are prepended to the method body. For example:

```rust
#[cgp_impl(new RectangleArea)]
impl AreaCalculator {
    fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
        width * height
    }
}
```

gains `Self: HasField<Symbol!("width"), Value = f64> + HasField<Symbol!("height"), Value = f64>` on the impl, with `width` and `height` bound from the context at the top of `area`.

## Examples

A complete `#[cgp_fn]` capability with implicit arguments needs only a context that derives [`HasField`](../derives/derive_has_field.md) and contains the named fields:

```rust
use cgp::prelude::*;

#[cgp_fn]
pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

fn print_area(rect: &Rectangle) {
    println!("area = {}", rect.rectangle_area());
}
```

`Rectangle` derives `HasField` for `width` and `height`, which satisfies the two bounds the macro added, so `RectangleArea` is implemented for `Rectangle` through the generated blanket impl. The call `rect.rectangle_area()` reads both fields from `rect` and multiplies them — no arguments are passed, because both were declared implicit and are sourced from the context.

## Related constructs

`#[implicit]` is most often used inside [`#[cgp_fn]`](../macros/cgp_fn.md), which turns a function into a single-implementation capability, and inside [`#[cgp_impl]`](../macros/cgp_impl.md), which writes a provider for an existing component. It relies on [`#[derive(HasField)]`](../derives/derive_has_field.md) on the context to supply the field accessors that the generated bounds require. Its access rules — `.clone()` for owned values, `.as_str()` for `&str`, and a plain `&T` read by reference with no clone — are shared with [`#[cgp_auto_getter]`](../macros/cgp_auto_getter.md), which defines a reusable getter *capability* trait. An implicit argument is the preferred, default way to read any field from a provider's own context; reserve `#[cgp_auto_getter]` for the cases an implicit argument cannot cover — a field that lives on a type other than the provider's context (a getter required as a `where` bound on that type), an accessor that must exist as a named capability other code depends on, or a getter carrying an associated type inferred from the field. To bring in other CGP capabilities alongside implicit arguments, combine `#[implicit]` with [`#[uses]`](uses.md).

## Source

- Parsing: implicit-argument parsing lives in [crates/macros/cgp-macro-core/src/functions/implicits/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/implicits/parse.rs), which extracts `#[implicit]`-marked arguments, validates the `self`/`mut` rules, and rejects a malformed (non-bare) `#[implicit]` attribute.
- Per-argument model: [crates/macros/cgp-macro-core/src/types/implicits/](../../../crates/macros/cgp-macro-core/src/types/implicits/) — `arg_field.rs` builds the `HasField` bound and the `let` binding, and `arg_fields.rs` adds the bounds to the impl generics and prepends the bindings to the body.
- Field-type-to-access-mode mapping (`.clone()`, `.as_str()`, `.as_deref()`, and the reference/option/slice cases): [crates/macros/cgp-macro-core/src/functions/field/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/field/parse.rs) and [crates/macros/cgp-macro-core/src/types/getter/get_field_with_mode_expr.rs](../../../crates/macros/cgp-macro-core/src/types/getter/get_field_with_mode_expr.rs).
- Implementation document (how `#[implicit]` arguments are parsed and lowered into `HasField` bounds and `let` bindings, and the index of tests): [implementation/entrypoints/cgp_fn.md](../../implementation/entrypoints/cgp_fn.md).
