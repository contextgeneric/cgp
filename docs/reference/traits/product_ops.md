# Product operations: `AppendProduct`, `ConcatProduct`, `MapFields`

`AppendProduct`, `ConcatProduct`, and `MapFields` are type-level operations that transform [`Cons`](../types/cons.md)/`Nil` products (and, for `MapFields`, [`Either`](../types/either.md)/`Void` sums), letting generic code grow and reshape a context's field list without naming the concrete types.

## Purpose

These three traits are the list algebra that CGP's structural machinery is built from. A struct's whole shape is a type-level product of fields — a [`Cons`](../types/cons.md) spine terminated by `Nil`, produced by [`#[derive(HasFields)]`](../derives/derive_has_fields.md) — and an enum's shape is the analogous [`Either`](../types/either.md) sum terminated by `Void`. To process such a shape generically, code needs to compute new shapes from old ones at the type level: add one field to the end of a product, splice two products together, or rewrite every entry uniformly. `AppendProduct`, `ConcatProduct`, and `MapFields` are exactly those three computations, each expressed as a trait whose recursion over the spine the compiler evaluates during type checking.

Because they operate purely on types, these operations carry no runtime cost and impose no ordering on the program — they are pure functions from type lists to type lists. They are the plumbing beneath higher-level constructs: building a record one field at a time appends to a product, merging two records concatenates them, and producing a partial-record representation maps a marker over every field.

## Definition

`AppendProduct<Item>` adds a single entry to the end of a product, exposing the extended product as its `Output`. It recurses down the [`Cons`](../types/cons.md) spine, rebuilding each node, until it reaches `Nil`, where it inserts the new `Cons<Item, Nil>`:

```rust
pub trait AppendProduct<Item: ?Sized> {
    type Output;
}

impl<Item> AppendProduct<Item> for Nil {
    type Output = Cons<Item, Nil>;
}

impl<Head, Tail, Item> AppendProduct<Item> for Cons<Head, Tail>
where
    Tail: AppendProduct<Item>,
{
    type Output = Cons<Head, Tail::Output>;
}
```

`ConcatProduct<Items>` splices a second product onto the end of the first. It has the same spine recursion, but at `Nil` it substitutes the entire `Items` list rather than a single element, so the result is the first product's entries followed by all of the second's:

```rust
pub trait ConcatProduct<Items> {
    type Output;
}

impl<Items> ConcatProduct<Items> for Nil {
    type Output = Items;
}

impl<Head, Tail, Items> ConcatProduct<Items> for Cons<Head, Tail>
where
    Tail: ConcatProduct<Items>,
{
    type Output = Cons<Head, Tail::Output>;
}
```

`MapFields<Mapper>` rewrites every entry of a list through a [`MapType`](map_type.md) marker, exposing the rewritten list as `Mapped`. Unlike the other two, it is defined over both spines: for the product spine it walks `Cons`/`Nil`, and for the sum spine it walks `Either`/`Void`, in each case applying `Mapper::Map` to the head and recursing on the tail:

```rust
pub trait MapFields<Mapper> {
    type Mapped;
}

impl<Mapper, Current, Rest> MapFields<Mapper> for Cons<Current, Rest>
where
    Mapper: MapType,
    Rest: MapFields<Mapper>,
{
    type Mapped = Cons<Mapper::Map<Current>, Rest::Mapped>;
}

impl<Mapper> MapFields<Mapper> for Nil {
    type Mapped = Nil;
}
```

The `Either`/`Void` impls mirror these exactly, replacing `Cons` with `Either` and `Nil` with `Void`, so the same `Mapper` applies uniformly whether the shape is a record or a tagged union.

## Behavior

The defining behavior of all three is that they are evaluated at compile time by the trait resolver walking the spine. `AppendProduct` and `ConcatProduct` preserve every existing entry and differ only in what they graft on at the `Nil` terminator — one element versus a whole list — which makes append a special case of concat with a single-element tail. Neither touches the values' types beyond reordering them into a longer list.

`MapFields` is the transforming operation: it leaves the spine's length and shape unchanged but replaces each entry type `T` with `Mapper::Map<T>`. With `IsPresent` it is the identity; with `IsNothing` it collapses every entry to the unit type; with `IsOptional` it wraps every entry in `Option`. This is how a concrete product of field values becomes the field list of a partial-record representation, where each field is independently marked. Because `MapFields` covers both spines, the same marker turns a struct's product into a partial struct and an enum's sum into a partial enum.

## Examples

These operations underlie the extensible-record machinery, so they are most visible through its higher-level interface, but they can be exercised directly on type-level lists. Appending and concatenating compute new product types:

```rust
use cgp::prelude::*;

type Base = Product![Field<Symbol!("host"), String>];

// AppendProduct adds one field to the end
type WithPort = <Base as AppendProduct<Field<Symbol!("port"), u16>>>::Output;
// = Product![Field<Symbol!("host"), String>, Field<Symbol!("port"), u16>]

// ConcatProduct splices a whole product onto the end
type Extra = Product![Field<Symbol!("tls"), bool>];
type Full = <WithPort as ConcatProduct<Extra>>::Output;
// = Product![host, port, tls]
```

`MapFields` rewrites every entry uniformly. Applying `IsOptional` turns a product of plain values into a product of optionals, the shape a partial builder uses to track which fields are not yet filled:

```rust
type Fields = Product![String, u16, bool];
type Optional = <Fields as MapFields<IsOptional>>::Mapped;
// = Product![Option<String>, Option<u16>, Option<bool>]
```

## Related constructs

These operations act on the [`Cons`](../types/cons.md)/`Nil` product spine and, for `MapFields`, the [`Either`](../types/either.md)/`Void` sum spine — the two type-level lists that [`#[derive(HasFields)]`](../derives/derive_has_fields.md) produces from structs and enums. The convenient surface syntax for those lists is the [`Product!`](../macros/product.md) macro. `MapFields` applies a [`MapType`](map_type.md) marker to every entry, which is the same per-field state vocabulary that the builder family in [`HasBuilder`](has_builder.md) and the extractor family in [`ExtractField`](extract_field.md) use to track presence; `AppendProduct` and `ConcatProduct` are the list-growing operations behind assembling and merging records.

## Source

`AppendProduct` is defined in [crates/core/cgp-field/src/traits/append_product.rs](../../../crates/core/cgp-field/src/traits/append_product.rs), `ConcatProduct` in [crates/core/cgp-field/src/traits/concat_product.rs](../../../crates/core/cgp-field/src/traits/concat_product.rs), and `MapFields` in [crates/core/cgp-field/src/traits/map_fields.rs](../../../crates/core/cgp-field/src/traits/map_fields.rs). The [`MapType`](map_type.md) markers `MapFields` applies are in [crates/core/cgp-field/src/impls/map_type.rs](../../../crates/core/cgp-field/src/impls/map_type.rs), and the [`Cons`](../types/cons.md)/`Nil`/[`Either`](../types/either.md)/`Void` building blocks are under [crates/core/cgp-field/src/types/](../../../crates/core/cgp-field/src/types/). These operations are exercised end-to-end by the extensible-data tests under [crates/tests/cgp-tests/tests/extensible_records/](../../../crates/tests/cgp-tests/tests/extensible_records/) and [crates/tests/cgp-tests/tests/extensible_variants/](../../../crates/tests/cgp-tests/tests/extensible_variants/).
