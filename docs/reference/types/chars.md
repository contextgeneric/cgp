# `Chars` and `Symbol`

`Chars<const CHAR: char, Tail>` is a type-level character list and `Symbol<const LEN: usize, Chars>` wraps such a list together with the string's byte length — together they are CGP's encoding of a string as a type.

## Purpose

`Chars` and `Symbol` exist so that a string can be a *type* rather than a value, which is what lets a field name participate in trait resolution. CGP keys field access by a `Tag` type: to read a field called `name`, something must stand in for the string `"name"` at the type level so the compiler can match one `HasField` impl against another purely from the tag. A `Symbol` is that something — a unique type whose entire identity is the character sequence it encodes, so two symbols built from the same string are the same type and two built from different strings are different types.

The reason the encoding is a *list of characters* rather than a single const value is a limitation in stable Rust: a `String` or `&str` cannot be used as a const-generic parameter, but a single `char` can. CGP works around this by spelling the string out one character at a time through a recursive `Chars` spine, the same way a heterogeneous list spells out its elements through a [`Cons`](cons.md)/[`Nil`](cons.md) spine. `Chars` is the specialized analogue of `Cons` in which the head is a `const char` rather than a type.

These two types are almost never written by hand. They are produced by the [`Symbol!`](../macros/symbol.md) macro, which takes a string literal and folds it into the corresponding `Symbol`/`Chars`/`Nil` chain. This document describes the runtime types and the traits they carry; the construction syntax and the macro's right-fold expansion live in the `Symbol!` macro document.

## Definition

`Chars` is a zero-sized struct carrying one character as a const parameter and the rest of the string as its `Tail`:

```rust
pub struct Chars<const CHAR: char, Tail>(pub PhantomData<Tail>);
```

The `Tail` is expected to be either the next `Chars` node or `Nil` to mark the end of the string, so a chain of `Chars` terminated by `Nil` is a type-level list of characters. `Chars` carries no runtime data — the character lives in the const parameter and the tail in a `PhantomData` — so the whole list is erased to a zero-sized value at runtime.

`Symbol` wraps a `Chars` chain and records the string's byte length as a separate const parameter:

```rust
pub struct Symbol<const LEN: usize, Chars>(pub PhantomData<Chars>);
```

The `Chars` type parameter is the head of the character list (despite the name, it is the whole chain, not a single node), and `LEN` is the string's byte length — the value of `str::len()`. The length is stored explicitly because stable Rust cannot compute the length of a `Chars` chain inside a const-generic context; baking it into the type lets length-dependent code read the size off the type directly instead of recursing through the list. Because `LEN` is the *byte* length, a four-character string of multi-byte scalars such as `Symbol!("世界你好")` records `12`, while its character list has one `Chars` node per Unicode scalar value.

## Behavior

Both types reconstruct their original string on demand through the [`StaticFormat`](../traits/static_format.md) trait, which formats a type-level string into a `Formatter` without needing a value. `Chars<CHAR, Tail>` implements `StaticFormat` by writing `CHAR` and then recursing into `Tail::fmt`, and `Nil` terminates the recursion by writing nothing; `Symbol<LEN, Chars>` forwards to its inner `Chars`. Each type also has a `Display` impl that defers to `StaticFormat`, so `<Symbol!("hello")>::default().to_string()` yields `"hello"`.

The other trait the `LEN` parameter enables is [`StaticString`](../traits/static_format.md), which exposes the string as a single `const VALUE: &'static str` rather than a formatting routine. Its blanket impl decodes the `Symbol`'s characters into a `[u8; LEN]` at compile time — this is the consumer for which `LEN` exists, since the byte buffer must be sized by a const. A reader needing the string as a value at runtime uses `Display`; a reader needing it as a const uses `StaticString::VALUE`.

`Symbol` also implements `Default` unconditionally (it is a zero-sized marker, so the default is just the empty `PhantomData`), which is what allows a `Symbol!("…")` type to be materialized as a value where one is needed, such as the tag passed to `get_field`.

## Examples

A type-level string most often appears as the `Tag` in a [`HasField`](../traits/has_field.md) bound, where it names the field a provider reads:

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

The same type can be constructed directly and inspected at runtime through its `Display` impl, which walks the `Chars` chain to rebuild the string:

```rust
use cgp::prelude::*;

let s = <Symbol!("hello")>::default();
assert_eq!(s.to_string(), "hello");
```

Because the encoding is a list, an empty string is `Symbol<0, Nil>` — a `Symbol` whose character list is just the terminator and whose recorded length is zero.

## Related constructs

`Chars` is the character-level specialization of the [`Cons`](cons.md)/`Nil` product spine: where `Cons` holds an arbitrary type as its head, `Chars` holds a `const char`, and both terminate in `Nil`. The pair is built by the [`Symbol!`](../macros/symbol.md) macro and consumed by [`StaticFormat`/`StaticString`](../traits/static_format.md) for display and const access. A `Symbol` is the field-name half of CGP's tagging scheme — its position counterpart for tuple fields is [`Index`](index.md) — and the tags it produces are matched by [`HasField`](../traits/has_field.md) and carried inside the [`Field`](field.md) entries of a struct's [`HasFields`](../traits/has_fields.md) representation. Symbols also appear as the lowercase segments of a type-level [`PathCons`](path_cons.md) path built by [`Path!`](../macros/path.md).

## Source

The runtime types are defined in [crates/core/cgp-base-types/src/types/chars.rs](../../../crates/core/cgp-base-types/src/types/chars.rs) (`Chars<const CHAR: char, Tail>`) and [crates/core/cgp-base-types/src/types/symbol.rs](../../../crates/core/cgp-base-types/src/types/symbol.rs) (`Symbol<const LEN: usize, Chars>`), with `Nil` in [crates/core/cgp-base-types/src/types/nil.rs](../../../crates/core/cgp-base-types/src/types/nil.rs). The `StaticFormat` impls that drive `Display` are in [crates/core/cgp-base-types/src/traits/static_format.rs](../../../crates/core/cgp-base-types/src/traits/static_format.rs), and the const-decoding `StaticString` impl that consumes `LEN` is in [crates/core/cgp-field/src/traits/static_string.rs](../../../crates/core/cgp-field/src/traits/static_string.rs). The constructing macro is [`Symbol!`](../macros/symbol.md). Tests covering display round-tripping and multi-byte strings are in [crates/tests/cgp-tests/src/tests/symbol.rs](../../../crates/tests/cgp-tests/src/tests/symbol.rs).
