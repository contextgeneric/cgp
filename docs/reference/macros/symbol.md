# `Symbol!`

`Symbol!("...")` is the type macro that turns a string literal into a type-level string — a distinct type, carrying no runtime value, that CGP uses to name a field so the name can be matched and dispatched on at compile time.

## Purpose

`Symbol!` exists because CGP needs field *names* to be types, not values. The core getter mechanism, [`HasField`](../traits/has_field.md), is parameterized by a `Tag` type that identifies which field is being read; to look up a field called `name`, something must stand in for the string `"name"` at the type level. `Symbol!("name")` is that something. It produces a unique type whose entire identity is the character sequence it encodes, so two `Symbol!` invocations with the same string are the same type and two with different strings are different types.

Encoding strings as types is what lets field access participate in trait resolution. Because `Symbol!("width")` is a type, a context can carry one `HasField<Symbol!("width")>` impl and another `HasField<Symbol!("height")>` impl side by side, and the compiler picks the right one purely from the tag. The same string-as-type trick drives [`#[cgp_auto_getter]`](cgp_auto_getter.md), `UseField<Symbol!("...")>`, and the `Field<Symbol!("..."), Value>` entries that make up a struct's [`HasFields`](../traits/has_fields.md) representation. Wherever a name needs to be matched at compile time, it appears as a `Symbol!`.

The type-level string is distinct from the type-level *number* used for tuple fields. Unnamed (tuple) struct fields have no string name, so [`#[derive(HasField)]`](../derives/derive_has_field.md) tags them with the [`Index`](../types/index.md) type instead — `Index<0>`, `Index<1>`, and so on — which encodes a `usize` at the type level the way `Symbol!` encodes a string. A field is keyed by `Symbol!` when it has a name and by `Index` when it has only a position.

## Syntax

The macro takes a single string literal and is used wherever a type is expected. It appears in trait bounds, associated-type positions, and `PhantomData` tags:

```rust
Symbol!("name")
Symbol!("first_name")
Symbol!("")
```

Any valid string literal is accepted, including the empty string and multi-byte Unicode (`Symbol!("世界")`), and the macro is most commonly seen inside a `HasField` bound such as `HasField<Symbol!("name"), Value = String>`.

## Syntax Grammar

The input to `Symbol!` is a single string literal:

```ebnf
SymbolInput -> STRING_LITERAL
```

`STRING_LITERAL` is the Rust string-literal token, so any valid string literal is accepted — including the empty string and multi-byte Unicode. The macro is used in type position, so this single literal is the whole of its input.

## Expansion

`Symbol!("...")` expands to the `Symbol` type wrapping a `Chars` chain that spells out the string one character at a time. The string `"abc"` desugars as follows:

```rust
// before
Symbol!("abc")
```

```rust
// after
Symbol<3, Chars<'a', Chars<'b', Chars<'c', Nil>>>>
```

Two type constructors do the work, and both are defined in `cgp-base-types`. `Chars<const CHAR: char, Tail>` is a single character paired with the rest of the string; chained through its `Tail` and terminated by `Nil`, it forms a type-level list of characters — the direct analogue of [`Cons`](../types/cons.md)/`Nil` but specialized so the head is a `const char` rather than a type. `Symbol<const LEN: usize, Chars>` then wraps that character list together with the string's length.

The `LEN` parameter is the part of the expansion most likely to surprise a reader, and it exists to work around a limitation in stable Rust. Stable Rust cannot evaluate the length of a `Chars` chain inside a const-generic context, so the macro precomputes the length and bakes it into the type as a separate const parameter rather than deriving it from the character list. The value is the string's byte length — `str::len()` — so `Symbol!("abc")` records `3` and a four-character Chinese string like `Symbol!("世界你好")` records `12`, not `4`. The character list, by contrast, has one `Chars` node per Unicode scalar value. `LEN` lets length-dependent code read the size off the type directly instead of recursing through the list.

The macro builds the expansion by folding the characters from right to left onto `Nil`, then wrapping the result in `Symbol` with the precomputed length, so an empty string `Symbol!("")` becomes `Symbol<0, Nil>`.

## Examples

`Symbol!` most often appears as the tag in a getter bound. The following provider reads a `name` field from any context that exposes one, without that context ever naming the provider:

```rust
use cgp::prelude::*;

#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasField<Symbol!("name"), Value = String>,
{
    fn greet(&self) {
        println!("Hello, {}!", self.get_field(PhantomData::<Symbol!("name")>));
    }
}
```

A type-level string can also be constructed directly and inspected at runtime through its `Display` impl, which reconstructs the original string from the `Chars` chain:

```rust
let s = <Symbol!("hello")>::default();
assert_eq!(s.to_string(), "hello"); // prints "hello"
```

When a struct derives [`HasField`](../derives/derive_has_field.md), each named field's tag is a `Symbol!` of the field name, and the whole struct's [`HasFields`](../traits/has_fields.md) representation is a [`Product!`](product.md) of `Field<Symbol!("..."), Value>` entries — so `Symbol!` is the bridge between a field's source name and its type-level identity.

## Related constructs

`Symbol!` is the field-name half of CGP's tagging scheme; [`Index`](../types/index.md) is the field-position half, used for tuple-struct fields. The tags it produces are consumed by [`HasField`](../traits/has_field.md) for single-field access and by the `Field<Tag, Value>` entries inside [`HasFields`](../traits/has_fields.md). The character list it builds is a specialized [`Product!`](product.md) list — `Chars`/`Nil` mirror `Cons`/`Nil`. Higher-level getters that take a `Symbol!` tag include [`#[cgp_auto_getter]`](cgp_auto_getter.md) and the `UseField` pattern. For enums, the analogous variant names appear as `Symbol!` tags inside a [`Sum!`](sum.md) representation.

## Source

- Entry point: `Symbol` in [crates/macros/cgp-macro-lib/src/symbol.rs](../../../crates/macros/cgp-macro-lib/src/symbol.rs), which forwards to the `Symbol` construct in [crates/macros/cgp-macro-core/src/types/field/symbol.rs](../../../crates/macros/cgp-macro-core/src/types/field/symbol.rs) — its `ToTokens` impl performs the right-fold over the characters, computes `LEN` from `str::len()`, and wraps the result in `Symbol`.
- Runtime types: [crates/core/cgp-base-types/src/types/symbol.rs](../../../crates/core/cgp-base-types/src/types/symbol.rs) (`Symbol<const LEN: usize, Chars>`) and [crates/core/cgp-base-types/src/types/chars.rs](../../../crates/core/cgp-base-types/src/types/chars.rs) (`Chars<const CHAR: char, Tail>`), with `Nil` in [crates/core/cgp-base-types/src/types/nil.rs](../../../crates/core/cgp-base-types/src/types/nil.rs).
- Internal walkthrough (the parse-and-emit pipeline, the character fold and the `LEN` const workaround, and the index of tests): [implementation/entrypoints/symbol.md](../../implementation/entrypoints/symbol.md).
