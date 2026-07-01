# `#[derive(CgpData)]`

`#[derive(CgpData)]` is the umbrella derive for extensible data: applied to a struct or an enum, it generates every CGP trait impl that lets the type be taken apart, rebuilt, and converted to and from a generic field representation.

## Purpose

`#[derive(CgpData)]` turns an ordinary struct or enum into *extensible data* — a type whose fields can be enumerated, accessed, built up incrementally, and matched generically at the type level, rather than only through hand-written field-by-field code. The motivation is the same one that drives the rest of CGP: a concrete `struct Person { first_name, last_name }` is opaque to generic code, because nothing about it can be referred to by a type parameter. `#[derive(CgpData)]` exposes the type's shape as type-level data so that generic providers can operate over any data type uniformly.

The derive does this by generating two complementary views of the type. The first is a *representation* view: the type's fields become a type-level product (for structs) or sum (for enums) of named [`Field`](../types/field.md) entries, reachable through [`HasFields`](../traits/has_fields.md) and convertible with `FromFields`/`ToFields`. The second is an *incremental* view: a generated partial type that holds each field in a present-or-absent state, so that values can be assembled one field at a time (for structs) or peeled apart one variant at a time (for enums). Together these are what make data "extensible" — generic code can add a field to a partial struct, or remove a variant from a partial enum, without naming the concrete type.

The practical payoff is that data becomes the subject of CGP wiring, not just behavior. Builders that merge several source structs into a target struct, dispatchers that match an enum variant and route it to a handler, and casts that widen or narrow between related enums all work generically because `#[derive(CgpData)]` supplied the field-level machinery they consume. Behavior-only components never need this derive; it is for the data types that flow through them.

`#[derive(CgpData)]` is the high-level entry point. It dispatches on whether the annotated item is a struct or an enum and emits exactly what [`#[derive(CgpRecord)]`](derive_cgp_record.md) or [`#[derive(CgpVariant)]`](derive_cgp_variant.md) would emit for that shape — so the two shape-specific derives are simply `CgpData` restricted to one kind of type.

## Syntax

The derive is applied like any other derive macro, on a `struct` or an `enum`, and takes no arguments:

```rust
use cgp::prelude::*;

#[derive(CgpData)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}
```

What the derive generates depends entirely on the kind of item. For a struct it produces the record machinery (field getters, the field representation, and the incremental builder); for an enum it produces the variant machinery (the field representation, variant constructors, and the incremental extractor). The two paths share the `HasFields`/`FromFields`/`ToFields` representation traits but otherwise emit different impls, so the sections below treat them separately.

Field tags drive everything. A named struct field or an enum variant becomes a type-level string [`Symbol!`](../macros/symbol.md), and an unnamed field of a tuple struct becomes a positional [`Index<N>`](../types/index.md); that tag is what addresses the field in the generated impls. Struct field types and single-field variant payload types become the field values. Generic parameters on the type are carried through onto the generated impls.

## Expansion

`#[derive(CgpData)]` expands into a fixed set of impls determined by whether the input is a struct or an enum. The exact generated code is verbose because every field name is spelled out as its full `Symbol<Len, Chars<...>>` form; the blocks below abbreviate those symbols as `Symbol!("name")` for readability, matching how the expansion snapshots read once collapsed.

For a struct, the derive emits the record expansion. Given:

```rust
#[derive(CgpData)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}
```

it generates per-field [`HasField`](../traits/has_field.md)/`HasFieldMut` getters, the representation traits, and the builder machinery. The representation traits expose the struct as a product of named fields:

```rust
impl HasFields for Person {
    type Fields = Cons<
        Field<Symbol!("first_name"), String>,
        Cons<Field<Symbol!("last_name"), String>, Nil>,
    >;
}

impl FromFields for Person {
    fn from_fields(Cons(first_name, Cons(last_name, Nil)): Self::Fields) -> Self {
        Self { first_name: first_name.value, last_name: last_name.value }
    }
}

impl ToFields for Person { /* self -> Cons(...) */ }
// plus HasFieldsRef and ToFieldsRef for borrowed access
```

The builder half generates a partial companion struct `__PartialPerson`, whose each field is wrapped in a `MapType` marker so the field can be present (`IsPresent`), absent (`IsNothing`), or void (`IsVoid`):

```rust
pub struct __PartialPerson<__F0__: MapType, __F1__: MapType> {
    pub first_name: <__F0__ as MapType>::Map<String>,
    pub last_name: <__F1__ as MapType>::Map<String>,
}

impl HasBuilder for Person {
    type Builder = __PartialPerson<IsNothing, IsNothing>; // all absent
    fn builder() -> Self::Builder { __PartialPerson { first_name: (), last_name: () } }
}

impl FinalizeBuild for __PartialPerson<IsPresent, IsPresent> { /* -> Person */ }
```

This builder expansion — `__Partial{Name}`, `HasBuilder`, `IntoBuilder`, `PartialData`, `FinalizeBuild`, the per-field `UpdateField` impls (which `BuildField` is built on), and the per-field `HasField` impls on the partial type — is exactly what [`#[derive(BuildField)]`](derive_build_field.md) produces on its own. See that document for the full builder breakdown.

For an enum, the derive emits the variant expansion instead. Given:

```rust
#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}
```

it generates the representation traits (now over a sum), the [`FromVariant`](../traits/from_variant.md) constructors, and the extractor machinery. The representation is a sum of named fields terminated by [`Void`](../types/either.md):

```rust
impl HasFields for Shape {
    type Fields = Either<
        Field<Symbol!("Circle"), Circle>,
        Either<Field<Symbol!("Rectangle"), Rectangle>, Void>,
    >;
}

impl FromVariant<Symbol!("Circle")> for Shape {
    type Value = Circle;
    fn from_variant(_tag: PhantomData<Symbol!("Circle")>, value: Circle) -> Self {
        Self::Circle(value)
    }
}
// plus FromVariant for Rectangle, and FromFields/ToFields/ToFieldsRef
```

The extractor half generates partial enums `__PartialShape` and `__PartialRefShape`, plus `HasExtractor`/`HasExtractorRef`/`HasExtractorMut`, `FinalizeExtract`, and per-variant [`ExtractField`](../traits/extract_field.md) impls that peel one variant off and return the remaining variants:

```rust
pub enum __PartialShape<__F0__: MapType, __F1__: MapType> {
    Circle(<__F0__ as MapType>::Map<Circle>),
    Rectangle(<__F1__ as MapType>::Map<Rectangle>),
}

impl HasExtractor for Shape {
    type Extractor = __PartialShape<IsPresent, IsPresent>;
    /* to_extractor / from_extractor */
}
```

This extractor expansion is exactly what [`#[derive(ExtractField)]`](derive_extract_field.md) produces, and the `FromVariant` impls are what [`#[derive(FromVariant)]`](derive_from_variant.md) produces. `#[derive(CgpData)]` on an enum is the union of those two building-block derives plus the shared representation traits.

Two details of the expansion are worth holding onto. The partial type is named `__Partial{Name}` (and `__PartialRef{Name}` for the borrowed extractor), with reserved double-underscore names so they do not collide with user types. And the `Tag` that keys each field follows the same rule as [`#[derive(HasField)]`](derive_has_field.md): a named field or enum variant is keyed by the [`Symbol!`](../macros/symbol.md) of its identifier, while an unnamed field of a tuple struct is keyed by its positional [`Index<N>`](../types/index.md). A tuple-struct record therefore exposes `Field<Index<0>, _>` entries in its `Fields` product and `UpdateField<Index<0>, _>` impls in its builder, not symbol tags.

## Examples

A struct example shows the builder view end to end. Because `Person` derives `CgpData`, it gains a builder that can be filled field by field and from other structs that share field names:

```rust
use cgp::prelude::*;
use cgp::core::field::impls::CanBuildFrom;

#[derive(CgpData)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

#[derive(CgpData)]
pub struct Employee {
    pub employee_id: u64,
    pub first_name: String,
    pub last_name: String,
}

fn make_employee(person: Person) -> Employee {
    Employee::builder()                                   // all fields absent
        .build_from(person)                               // fills first_name + last_name
        .build_field(PhantomData::<Symbol!("employee_id")>, 1)
        .finalize_build()                                 // all present -> Employee
}
```

An enum example shows the extractor view. Because `Shape` derives `CgpData`, a value can be converted to its extractor and have one variant pulled out, with the remainder carrying the still-unmatched variants:

```rust
use cgp::prelude::*;
use cgp::core::field::traits::FinalizeExtractResult;

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

fn area(shape: Shape) -> f64 {
    match shape.to_extractor().extract_field(PhantomData::<Symbol!("Circle")>) {
        Ok(circle) => core::f64::consts::PI * circle.radius * circle.radius,
        Err(remainder) => {
            let rect = remainder
                .extract_field(PhantomData::<Symbol!("Rectangle")>)
                .finalize_extract_result();
            rect.width * rect.height
        }
    }
}
```

These two views are what higher-level constructs build on: builders and field-dispatch handlers consume the struct machinery, while variant dispatch and the `upcast`/`downcast` casts consume the enum machinery.

## Related constructs

`#[derive(CgpData)]` is the umbrella over a family of shape- and capability-specific derives. [`#[derive(CgpRecord)]`](derive_cgp_record.md) and [`#[derive(CgpVariant)]`](derive_cgp_variant.md) are `CgpData` restricted to structs and enums respectively, useful when you want to document intent or when only one shape is valid. The building-block derives generate slices of the same output: [`#[derive(BuildField)]`](derive_build_field.md) emits just the struct builder, [`#[derive(ExtractField)]`](derive_extract_field.md) just the enum extractor, and [`#[derive(FromVariant)]`](derive_from_variant.md) just the variant constructors. [`#[derive(HasFields)]`](derive_has_fields.md) emits just the representation traits, and [`#[derive(HasField)]`](derive_has_field.md) just the plain field getters. The generated types reference [`Field`](../types/field.md), the [`product`](../macros/product.md) (`Cons`/`Nil`) and [`sum`](../macros/sum.md) (`Either`/`Void`) type-level lists, and the `MapType` markers `IsPresent`/`IsNothing`/`IsVoid`.

## Source

The derive entry point is `derive_cgp_data` in [crates/macros/cgp-macro-lib/src/cgp_data.rs](../../../crates/macros/cgp-macro-lib/src/cgp_data.rs), which parses the input into `ItemCgpData` and calls `to_items()`. The dispatch on struct vs. enum is in [crates/macros/cgp-macro-core/src/types/cgp_data/item.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/item.rs); the record path is in `record.rs` and the variant path in `variant.rs` in the same directory, which delegate to the `derive_has_fields/`, `derive_builder/`, `derive_extractor/`, and `derive_from_variant.rs` submodules. The runtime traits are in [crates/core/cgp-field/src/traits/](../../../crates/core/cgp-field/src/traits/) and the `MapType` markers in `crates/core/cgp-field/src/impls/map_type.rs`. Expansion snapshots and behavioral tests live in [crates/tests/cgp-tests/tests/extensible_records/](../../../crates/tests/cgp-tests/tests/extensible_records/) and [crates/tests/cgp-tests/tests/extensible_variants/](../../../crates/tests/cgp-tests/tests/extensible_variants/).
