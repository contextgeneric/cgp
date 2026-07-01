# `Index`

`Index<const I: usize>` encodes a `usize` at the type level, giving a tuple-struct field a type-level name based on its position the way [`Symbol!`](../macros/symbol.md) names a field by its string.

## Purpose

`Index` exists because CGP's getter mechanism keys every field by a *type* tag, and a tuple-struct field has no string name to turn into a [`Symbol!`](../macros/symbol.md) — it has only a position. To make positional fields participate in the same trait-resolution machinery as named fields, the position itself must become a type. `Index<I>` is that type: it carries a `usize` as a const-generic parameter and nothing else, so `Index<0>`, `Index<1>`, and `Index<2>` are three distinct types standing in for the first, second, and third fields of a tuple struct.

Encoding the position as a type is what lets positional field access resolve through traits. Because `Index<0>` is a type, a context can carry a `HasField<Index<0>>` impl for its first field and a `HasField<Index<1>>` impl for its second side by side, and the compiler selects the right one purely from the tag — exactly as it would for two differently-named `Symbol!` tags. `Index` is therefore the numeric counterpart to `Symbol!`: a field is keyed by a `Symbol!` when it has a name and by an `Index` when it has only a position.

`Index` carries no runtime data of its own; it is a zero-sized marker. Its sole job is to make a number available at the type level, so it can appear as a [`HasField`](../traits/has_field.md) tag, as the `Tag` of a [`Field`](field.md) entry inside a tuple struct's [`HasFields`](../traits/has_fields.md) list, and inside `PhantomData` wherever a positional name is needed at compile time.

## Definition

`Index` is a zero-sized struct parameterized only by a const-generic `usize`:

```rust
#[derive(Eq, PartialEq, Clone, Copy, Default)]
pub struct Index<const I: usize>;
```

The single const parameter `I` is the position the type represents — `Index<0>` for the field at offset zero, and so on. The struct has no fields, so a value of `Index<I>` carries no data; the number lives entirely in the type. The derived `Default`, `Clone`, and `Copy` make a value trivially available when one is needed (for instance as a `PhantomData`-free tag value), and `Eq`/`PartialEq` compare two values of the same `Index<I>` as always equal, since there is nothing to differ.

`Index<I>` implements both `Display` and `Debug`, and both print the underlying number `I` — `Index<0>` displays as `0`. The number a tag stands for is therefore visible directly in formatted output and in compiler diagnostics, with no Greek-letter or other alias substituted for it.

## Behavior

A tuple struct keys each of its fields by `Index<N>`, counting from zero, so the field at position `N` is read through the tag `Index<N>`. When a tuple struct derives [`HasField`](../derives/derive_has_field.md), the generated impl uses `Index<0>` for the `.0` field, `Index<1>` for `.1`, and so on, mapping each `get_field(PhantomData::<Index<N>>)` call to the corresponding positional access. The same `Index<N>` tags then appear as the `Tag` of each [`Field`](field.md) entry in the tuple struct's [`HasFields`](../traits/has_fields.md) representation, so generic code walking the field list reads positions where it would read `Symbol!` names for a named struct.

Because `Index<I>` is zero-sized and the position lives in the type, accessing a field by index resolves entirely at compile time: there is no array bound check and no runtime indexing. Selecting the wrong index is a type error, not a runtime panic, because `Index<5>` on a three-field struct simply has no matching `HasField` impl.

## Examples

When a tuple struct derives `HasField`, each positional field is tagged by an `Index`:

```rust
use cgp::prelude::*;

pub struct Pair(pub u32, pub String);

// generated for the first field:
// impl HasField<Index<0>> for Pair {
//     type Value = u32;
//     fn get_field(&self, _tag: PhantomData<Index<0>>) -> &u32 {
//         &self.0
//     }
// }
```

A field can then be read by supplying the `Index` tag, and the chosen position is fixed at compile time:

```rust
use cgp::prelude::*;

let pair = Pair(7, "hi".to_string());
assert_eq!(*pair.get_field(PhantomData::<Index<0>>), 7);
```

The number an `Index` carries is also visible through its `Display` impl, which prints the position:

```rust
assert_eq!(Index::<2>.to_string(), "2");
```

## Related constructs

`Index` is the field-position half of CGP's tagging scheme; [`Symbol!`](../macros/symbol.md) is the field-name half, used for named struct fields and enum variants. The tags it produces are consumed by [`HasField`](../traits/has_field.md) for single-field access — built per field by [`#[derive(HasField)]`](../derives/derive_has_field.md) — and appear as the `Tag` of [`Field`](field.md) entries inside the [`HasFields`](../traits/has_fields.md) list of a tuple struct. Both `Index` and `Symbol!` encode a primitive at the type level: `Index` a `usize`, `Symbol!` a string.

## Source

- `Index<const I: usize>` and its `Display` and `Debug` impls are defined in [crates/core/cgp-field/src/types/index.rs](../../../crates/core/cgp-field/src/types/index.rs).
- The `#[derive(HasField)]` codegen that tags tuple-struct fields with `Index<N>` lives under [crates/macros/cgp-macro-core/src/types/cgp_data/](../../../crates/macros/cgp-macro-core/src/types/cgp_data/), and the `HasField` trait it targets is in [crates/core/cgp-field/src/traits/has_field.rs](../../../crates/core/cgp-field/src/traits/has_field.rs).
