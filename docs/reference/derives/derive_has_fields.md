# `#[derive(HasFields)]`

`#[derive(HasFields)]` is the derive macro that gives a type a whole-shape view: it implements `HasFields` (and `HasFieldsRef`, `ToFields`, `FromFields`, `ToFieldsRef`) so that the entire struct or enum is described by a single type-level [`Product`](../macros/product.md) of named fields, or for an enum a type-level [`Sum`](../macros/sum.md) of named variants.

## Purpose

`#[derive(HasFields)]` exists to expose a type's complete field structure as one type, rather than one entry per field. Where [`#[derive(HasField)]`](derive_has_field.md) answers "give me *this* field by name," `HasFields` answers "describe *all* the fields at once" — it produces a single associated `Fields` type that is the type-level list of every field, each tagged with its name. This aggregate view is what lets generic code fold over a context's fields uniformly: serialization, builders, conversions, and the extensible-record machinery all operate on the `Fields` type rather than reaching for fields one at a time.

The distinction from `HasField` is the whole point. `HasField<Tag>` is indexed access — many small impls, each keyed by a tag, used for dependency injection where a provider wants one specific value. `HasFields` is the structural mirror — a single impl whose `Fields` type enumerates the entire shape, used when an algorithm must process the type as a record (for a struct) or as a tagged union (for an enum). The two are complementary, and a context that wants both indexed access and structural processing derives both.

Alongside the structural type, the derive also generates the conversions that move values in and out of that representation. `ToFields` turns an owned value into its `Fields` product, `FromFields` rebuilds the value from a `Fields` product, and `ToFieldsRef` borrows the value as a product of references. Together these make the generic `Fields` view a two-way door: code can decompose a concrete type into its anonymous structural form, transform it generically, and reconstruct the concrete type.

## Syntax

The macro is applied as a derive and takes no arguments, but unlike `HasField` it accepts both structs and enums:

```rust
#[derive(HasFields)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

#[derive(HasFields)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}
```

For a struct the generated `Fields` type is a product; for an enum it is a sum. Named fields within either are tagged by [`Symbol!`](../macros/symbol.md), and unnamed (tuple) fields by [`Index<N>`](../types/index.md), exactly as in `HasField`. A single-field tuple struct (a newtype) is treated specially: its `Fields` is the inner type directly, not wrapped in a one-element product. Applying the derive to anything other than a struct or enum is a compile error.

## Expansion

`#[derive(HasFields)]` emits five impls — `HasFields`, `HasFieldsRef`, `ToFields`, `FromFields`, and `ToFieldsRef` — and leaves the type definition untouched. The shape of the `Fields` type is the load-bearing part. Starting from a named-field struct:

```rust
#[derive(HasFields)]
pub struct Person {
    pub name: String,
    pub age: u8,
}
```

each field becomes a [`Field<Tag, Value>`](../types/field.md) entry — the `Value` wrapped together with its type-level name tag — and the entries are chained into a [`Product!`](../macros/product.md). The `HasFields` impl names that product, and `HasFieldsRef` names the same product with each value borrowed:

```rust
impl HasFields for Person {
    type Fields = Product![
        Field<Symbol!("name"), String>,
        Field<Symbol!("age"), u8>,
    ];
}

impl HasFieldsRef for Person {
    type FieldsRef<'a> = Product![
        Field<Symbol!("name"), &'a String>,
        Field<Symbol!("age"), &'a u8>,
    ]
    where
        Self: 'a;
}
```

The accompanying conversions move values between `Person` and that product. `to_fields` builds the product from the struct's fields, `from_fields` destructures the product back into the struct, and `to_fields_ref` builds the borrowed product:

```rust
impl ToFields for Person {
    fn to_fields(self) -> Self::Fields {
        Cons(self.name.into(), Cons(self.age.into(), Nil))
    }
}

impl FromFields for Person {
    fn from_fields(Cons(name, Cons(age, Nil)): Self::Fields) -> Self {
        Self { name: name.value, age: age.value }
    }
}

impl ToFieldsRef for Person {
    fn to_fields_ref<'a>(&'a self) -> Self::FieldsRef<'a>
    where
        Self: 'a,
    {
        Cons((&self.name).into(), Cons((&self.age).into(), Nil))
    }
}
```

An enum expands to a [`Sum`](../macros/sum.md) instead of a product. Each variant becomes an [`Either`](../types/either.md) arm tagged by the variant name with [`Symbol!`](../macros/symbol.md), carrying that variant's own fields as a nested product, and the chain is terminated by `Void`. Starting from:

```rust
#[derive(HasFields)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}
```

the `HasFields` impl names the sum of variants:

```rust
impl HasFields for Shape {
    type Fields = Either<
        Field<Symbol!("Circle"), Product![Field<Symbol!("radius"), f64>]>,
        Either<
            Field<Symbol!("Rectangle"), Product![
                Field<Symbol!("width"), f64>,
                Field<Symbol!("height"), f64>,
            ]>,
            Void,
        >,
    >;
}
```

The enum derive likewise emits `HasFieldsRef` (the same sum with borrowed values), `ToFields`, `FromFields`, and `ToFieldsRef`, with each conversion matching on the concrete variant and mapping it to the corresponding `Either` arm.

The associated trait definitions these impls satisfy are minimal: `HasFields` carries `type Fields`, `HasFieldsRef` carries `type FieldsRef<'a>`, and the three conversion traits each supertrait one of those and add a single method (`to_fields`, `from_fields`, `to_fields_ref`).

## Examples

Deriving `HasFields` lets generic code treat any context as a record without naming its concrete type. A common pairing is to derive it alongside [`HasField`](derive_has_field.md) so the same struct supports both indexed and structural access:

```rust
use cgp::prelude::*;

#[derive(HasField, HasFields)]
pub struct Config {
    pub host: String,
    pub port: u16,
}
```

With `HasFields` in place, `Config::Fields` is `Product![Field<Symbol!("host"), String>, Field<Symbol!("port"), u16>]`, and a value can be round-tripped through that product:

```rust
let config = Config { host: "localhost".into(), port: 8080 };

let fields = config.to_fields();        // ToFields → the Product
let config_again = Config::from_fields(fields); // FromFields → back to Config
```

Generic algorithms key off the `Fields` type rather than the concrete struct. This is how higher-level constructs such as the extensible-builder and data-manipulation machinery in [`#[derive(CgpData)]`](derive_cgp_data.md) operate over arbitrary contexts: they bound `Context: HasFields` (or `FromFields`/`ToFields`) and process `Context::Fields` structurally.

## Related constructs

`#[derive(HasFields)]` is the structural counterpart to [`#[derive(HasField)]`](derive_has_field.md): `HasField` gives indexed, single-field access for dependency injection, while `HasFields` gives the aggregate view of the whole type. The `Fields` type it produces is built from [`Product`](../macros/product.md) for structs and [`Sum`](../macros/sum.md) for enums, with named entries tagged by [`Symbol!`](../macros/symbol.md). [`#[derive(CgpData)]`](derive_cgp_data.md) builds on this derive, generating `HasFields` together with the additional builder, partial-record, and field-update machinery needed for extensible data.

## Source

The derive entry point is `derive_has_fields` in [crates/macros/cgp-macro-lib/src/derive_has_fields.rs](../../../crates/macros/cgp-macro-lib/src/derive_has_fields.rs), registered as the `HasFields` proc-macro derive in [crates/macros/cgp-macro/src/lib.rs](../../../crates/macros/cgp-macro/src/lib.rs). It dispatches on the parsed item: structs go through `ItemCgpRecord::to_has_fields_impls` → `derive_has_fields_impls_from_struct`, and enums through `derive_has_fields_impls_from_enum`, both in [crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_fields/](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_fields/). Within that module, `product.rs` (`item_fields_to_product_type`) builds the struct `Fields` product, `sum.rs` (`variants_to_sum_type`) builds the enum `Fields` sum, and `from_fields_*`, `to_fields_*`, and `to_fields_ref_*` build the conversion impls. The traits are defined in [crates/core/cgp-field/src/traits/has_fields.rs](../../../crates/core/cgp-field/src/traits/has_fields.rs), [crates/core/cgp-field/src/traits/to_fields.rs](../../../crates/core/cgp-field/src/traits/to_fields.rs), and [crates/core/cgp-field/src/traits/from_fields.rs](../../../crates/core/cgp-field/src/traits/from_fields.rs); the `Field`, `Either`, and `Void` building blocks live under [crates/core/cgp-field/src/types/](../../../crates/core/cgp-field/src/types/). Expansion snapshots are in [crates/tests/cgp-macro-tests](../../../crates/tests/cgp-macro-tests).
