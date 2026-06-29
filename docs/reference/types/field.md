# `Field`

`Field<Tag, Value>` is the named-entry building block of CGP's structural data, pairing a `Value` with a type-level `Tag` that records the field's name without carrying any runtime cost.

## Purpose

`Field` exists so that a field's *name* and its *value* travel together as a single type, letting generic code know not just what a struct holds but what each piece is called. A bare [`Product!`](../macros/product.md) list such as `Product![String, u8]` records only the types and their order; it cannot tell `name: String` from any other `String`. Wrapping each element in a `Field` — `Field<Symbol!("name"), String>` — attaches the name as a phantom type, so the structural representation of a struct is self-describing: a provider walking the list can match on the tag to find the field it wants.

The tag is carried as a phantom type precisely because the name is needed only at compile time, for trait resolution and dispatch, never at run time. A `Field<Tag, Value>` is therefore exactly as large as its `Value` — the `PhantomData<Tag>` occupies no space — so encoding names this way costs nothing at run time while making field-by-field generic code possible. This is the same string-as-type trick that [`Symbol!`](../macros/symbol.md) provides for names and [`Index`](index.md) provides for tuple positions; `Field` is where that tag meets the value it labels.

`Field` is the element type that fills both spines of structural data. In a product (a record), the [`HasFields`](../traits/has_fields.md) representation of a struct is a `Product!` of `Field` entries, one per field. In a sum (an enum), it is a [`Sum!`](../macros/sum.md) of `Field` entries, one per variant, where the `Value` is the variant's payload. The same `Field<Tag, Value>` shape names a field in a record and a variant in an enum.

## Definition

`Field` is a two-parameter struct holding the value alongside a phantom tag:

```rust
pub struct Field<Tag, Value> {
    pub value: Value,
    pub phantom: PhantomData<Tag>,
}
```

The `Tag` parameter is the type-level name of the field. It is a phantom — it appears only inside `PhantomData<Tag>` and never in a runtime field — and is typically a type-level string such as `Symbol!("name")` for a named field or a type-level number such as `Index<0>` for a tuple-struct position. The `Value` parameter is the field's actual type, and `value` is the only data the struct stores. Aside from the tag, a `Field` is a thin wrapper around its `Value`.

## Behavior

A `Field` is built from a value with no tag argument, because the tag is fixed by the target type rather than passed in. The `From<Value>` impl constructs the wrapper directly, so `let f: Field<Symbol!("name"), String> = "Alice".to_string().into();` fills in `value` and sets `phantom` to `PhantomData`. The tag is inferred from the expected type, which is why generated `HasFields` code can build each entry with a plain `.into()`.

The remaining trait impls all defer to the value and ignore the tag, so a `Field` behaves like its `Value` for comparison and printing. `Debug` forwards to the value's `Debug` (the tag is not shown), and `PartialEq`/`Eq` compare only the `value`, each gated on the corresponding bound on `Value`. Two `Field` values are equal when their values are equal; the tag is a compile-time matter and plays no part in these runtime operations.

Because the tag lives only in `PhantomData`, a `Field` carries the name purely at the type level. Code that needs the name reads it from the `Tag` parameter through trait resolution — for example matching a `Field<Symbol!("name"), _>` against a `HasField<Symbol!("name")>` bound — rather than from any stored data.

## Examples

`Field` appears most often inside the `HasFields` representation that a derive generates, where each struct field becomes one entry tagged by its `Symbol!` name:

```rust
use cgp::prelude::*;

#[derive(HasFields)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

// generated:
// impl HasFields for Person {
//     type Fields = Product![
//         Field<Symbol!("name"), String>,
//         Field<Symbol!("age"), u8>,
//     ];
// }
```

A single `Field` can also be constructed directly from its value, with the tag supplied by the type annotation:

```rust
use cgp::prelude::*;

let name: Field<Symbol!("name"), String> = "Alice".to_string().into();
assert_eq!(name.value, "Alice");
```

For a tuple-struct field, the tag is an [`Index`](index.md) rather than a `Symbol!`, so the same wrapper names a positional field: `Field<Index<0>, u32>`.

## Related constructs

`Field` is the element type of the two structural spines: it fills a [`Product!`](../macros/product.md) (built from `Cons`/`Nil`, see [`cons.md`](./cons.md)) for a record and a [`Sum!`](../macros/sum.md) (built from `Either`/`Void`, see [`either.md`](./either.md)) for an enum. Its `Tag` is produced by [`Symbol!`](../macros/symbol.md) for a named field and by [`Index`](index.md) for a tuple-struct position. The list of `Field` entries for a whole type is assigned by [`#[derive(HasFields)]`](../derives/derive_has_fields.md) and surfaced through the [`HasFields`](../traits/has_fields.md) trait, while single-field access against a matching tag is the job of [`HasField`](../traits/has_field.md), built per field by [`#[derive(HasField)]`](../derives/derive_has_field.md).

## Source

`Field<Tag, Value>` and its `From`, `Debug`, `PartialEq`, and `Eq` impls are defined in [crates/core/cgp-field/src/types/field.rs](../../../crates/core/cgp-field/src/types/field.rs). It is consumed by the field machinery: [`HasFields`](../traits/has_fields.md) in [crates/core/cgp-field/src/traits/has_fields.rs](../../../crates/core/cgp-field/src/traits/has_fields.rs), with the record/variant rebuilding traits in [crates/core/cgp-field/src/traits/from_fields.rs](../../../crates/core/cgp-field/src/traits/from_fields.rs) and [crates/core/cgp-field/src/traits/to_fields.rs](../../../crates/core/cgp-field/src/traits/to_fields.rs). The derive that emits `Product!`/`Sum!` lists of `Field` entries lives under [crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_fields/](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_fields/).
