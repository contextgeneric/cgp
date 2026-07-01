# `#[implicit]`

`#[implicit]` marks a function argument as an implicit dependency: instead of being passed by the caller, the value is read from a same-named field on the context, and the argument disappears from the public signature.

## Purpose

`#[implicit]` exists to make field-based dependency injection look like an ordinary function parameter. In plain CGP, a provider that needs a `width` value from its context declares a `HasField<Symbol!("width"), Value = f64>` bound in its `where` clause and calls `self.get_field(PhantomData)` inside the body. That works, but it forces the author to understand `HasField`, type-level symbols, and `PhantomData` tags before writing even the simplest provider. `#[implicit]` hides all of that behind a normal-looking parameter.

The argument named `width: f64` with `#[implicit]` reads as "this function needs a `width` of type `f64`," which is exactly the intuition a Rust programmer already has. The macro then does the mechanical work: it removes the argument from the signature, adds the matching `HasField` bound, and binds a local variable to the field value at the top of the body. The result is code that looks like a function taking arguments but behaves like a provider injecting dependencies from its context.

This is why `#[implicit]` is the recommended starting point for basic CGP. It lets a newcomer write providers in [`#[cgp_fn]`](../macros/cgp_fn.md) and [`#[cgp_impl]`](../macros/cgp_impl.md) using only familiar function syntax, deferring the `HasField` machinery until they actually need to understand it.

## Syntax

`#[implicit]` is written as a bare attribute on a typed function argument, and the argument must have a plain identifier name:

```rust
fn area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}
```

The argument name doubles as the field name. Here `width` and `height` name both the local variables used in the body and the context fields the values are read from, via `Symbol!("width")` and `Symbol!("height")`. The argument type is the type the body sees, and it determines how the field is accessed (described under Expansion).

Three rules constrain where `#[implicit]` may appear. The function must take `self` as its first argument, because the field is read from `self`; a function with implicit arguments but no receiver is rejected. The argument pattern must be a bare identifier, not a destructuring or `mut` pattern — to get a mutable local, clone the injected value explicitly inside the body. And when the receiver is `&mut self`, at most one implicit argument is allowed, since each one borrows from the same context.

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

The access expression depends on the argument type, following the same rules as [`#[cgp_auto_getter]`](../macros/cgp_auto_getter.md). For an owned type such as `f64` or `String`, the macro reads the field by reference and appends `.clone()`, so the body receives an owned value. The one special case worth knowing is `&str`: an argument typed `&str` is backed by a `String` field, and the access uses `.as_str()` rather than `.clone()`. Concretely:

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

`#[implicit]` is most often used inside [`#[cgp_fn]`](../macros/cgp_fn.md), which turns a function into a single-implementation capability, and inside [`#[cgp_impl]`](../macros/cgp_impl.md), which writes a provider for an existing component. It relies on [`#[derive(HasField)]`](../derives/derive_has_field.md) on the context to supply the field accessors that the generated bounds require. Its access rules — `.clone()` for owned values, `.as_str()` for `&str` — are shared with [`#[cgp_auto_getter]`](../macros/cgp_auto_getter.md), which defines a reusable getter *capability* trait; prefer an implicit argument for reading a field as a provider's own input, and reserve `#[cgp_auto_getter]` for the case where the field should be published as a `self.name()` accessor other providers depend on. To bring in other CGP capabilities alongside implicit arguments, combine `#[implicit]` with [`#[uses]`](uses.md).

## Source

Implicit-argument parsing lives in [crates/macros/cgp-macro-core/src/functions/implicits/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/implicits/parse.rs), which extracts `#[implicit]`-marked arguments and validates the `self`/`mut` rules. The per-argument model is in [crates/macros/cgp-macro-core/src/types/implicits/](../../../crates/macros/cgp-macro-core/src/types/implicits/): `arg_field.rs` builds the `HasField` bound and the `let` binding, and `arg_fields.rs` adds the bounds to the impl generics and prepends the bindings to the body. The field-type-to-access-mode mapping (`.clone()`, `.as_str()`, and the reference/option/slice cases) is in [crates/macros/cgp-macro-core/src/functions/field/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/field/parse.rs) and [crates/macros/cgp-macro-core/src/types/getter/get_field_with_mode_expr.rs](../../../crates/macros/cgp-macro-core/src/types/getter/get_field_with_mode_expr.rs). Expansion snapshots are in [crates/tests/cgp-tests/tests/cgp_fn_tests/basic.rs](../../../crates/tests/cgp-tests/tests/cgp_fn_tests/basic.rs) and [crates/tests/cgp-tests/tests/component_tests/cgp_impl/implicit_args/](../../../crates/tests/cgp-tests/tests/component_tests/cgp_impl/implicit_args/).
