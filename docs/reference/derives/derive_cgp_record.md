# `#[derive(CgpRecord)]`

`#[derive(CgpRecord)]` derives the extensible-data machinery for a struct: field getters, the type-level field representation, and an incremental builder, so the struct can be enumerated, accessed, and assembled field by field.

## Purpose

`#[derive(CgpRecord)]` makes a struct into an *extensible record* — a product of named fields that generic CGP code can address, convert, and build up one field at a time. A plain Rust struct is opaque to generic code: there is no way to refer to "the `first_name` field" or "a struct with these fields" through a type parameter. This derive exposes the struct's fields as type-level data so that generic providers — builders, field mergers, field-dispatch handlers — can operate over any record uniformly.

The derive is the struct-specific face of [`#[derive(CgpData)]`](derive_cgp_data.md). When `CgpData` is applied to a struct it emits exactly what `CgpRecord` emits; the two are the same code path. Use `CgpRecord` when the type is always a struct and you want the name to say so, or when you prefer a derive whose meaning is unambiguous at the use site. Using `CgpRecord` on an enum is a type error, since it parses its input as a struct.

The defining capability a record gains is incremental construction. Beyond plain field access, the derive generates a *partial* companion type that tracks, in its type parameters, which fields are present and which are still absent. Generic code can start from an empty builder, set fields individually or copy them in bulk from other records that share field names, and finalize only once every field is present. This present/absent tracking happens entirely at the type level, so a missing field is a compile error, not a runtime panic.

## Syntax

The derive is applied to a struct and takes no arguments:

```rust
use cgp::prelude::*;

#[derive(CgpRecord)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}
```

Each named field becomes a type-level string [`Symbol!`](../macros/symbol.md) used as the field's `Tag`, while an unnamed field of a tuple struct becomes a positional [`Index<N>`](../types/index.md) — the same tagging rule as [`#[derive(HasField)]`](derive_has_field.md). The field's declared type becomes its value type, and generic parameters on the struct are carried onto the generated impls. The derive accepts the same structs that [`#[derive(CgpData)]`](derive_cgp_data.md) accepts for the record path; the only difference is that `CgpRecord` refuses non-struct inputs outright.

## Expansion

`#[derive(CgpRecord)]` expands into three groups of impls: field getters, the representation traits, and the builder machinery. The symbols below are abbreviated as `Symbol!("name")` in place of the full `Symbol<Len, Chars<...>>` spelling the macro actually emits. Starting from:

```rust
#[derive(CgpRecord)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}
```

the derive first emits a [`HasField`](../traits/has_field.md) and a `HasFieldMut` impl per field, keyed by the field-name symbol — the same getters [`#[derive(HasField)]`](derive_has_field.md) would produce:

```rust
impl HasField<Symbol!("first_name")> for Person {
    type Value = String;
    fn get_field(&self, _: PhantomData<Symbol!("first_name")>) -> &String { &self.first_name }
}
// HasFieldMut<Symbol!("first_name")>, and the same pair for "last_name"
```

Second, it emits the representation traits, exposing the struct as a type-level product of named [`Field`](../types/field.md) entries and providing whole-record conversions — the [`#[derive(HasFields)]`](derive_has_fields.md) output:

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
// plus ToFields, HasFieldsRef, ToFieldsRef
```

Third, it emits the builder machinery — the [`#[derive(BuildField)]`](derive_build_field.md) output. This centers on a partial companion struct `__PartialPerson` whose each field is wrapped in a `MapType` marker, so a field can be present (`IsPresent`, the value itself), absent (`IsNothing`, a unit `()`), or void (`IsVoid`):

```rust
pub struct __PartialPerson<__F0__: MapType, __F1__: MapType> {
    pub first_name: <__F0__ as MapType>::Map<String>,
    pub last_name: <__F1__ as MapType>::Map<String>,
}

impl HasBuilder for Person {
    type Builder = __PartialPerson<IsNothing, IsNothing>;     // empty builder
    fn builder() -> Self::Builder { __PartialPerson { first_name: (), last_name: () } }
}

impl IntoBuilder for Person {
    type Builder = __PartialPerson<IsPresent, IsPresent>;     // fully populated
    fn into_builder(self) -> Self::Builder { /* move each field in */ }
}

impl<__F0__: MapType, __F1__: MapType> PartialData for __PartialPerson<__F0__, __F1__> {
    type Target = Person;
}

impl FinalizeBuild for __PartialPerson<IsPresent, IsPresent> {
    fn finalize_build(self) -> Person { Person { first_name: self.first_name, last_name: self.last_name } }
}
```

It then emits, per field, an `UpdateField` impl that flips that one field's marker from one state to another (this is what `BuildField` is implemented in terms of), and a `HasField` impl on the partial type that is available only when that field's marker is `IsPresent`. The full per-field detail is documented in [`#[derive(BuildField)]`](derive_build_field.md).

The single most important fact about the expansion is that *presence is encoded in the type*. `builder()` returns `__PartialPerson<IsNothing, IsNothing>`, `finalize_build` is implemented only for `__PartialPerson<IsPresent, IsPresent>`, and each `build_field`/`build_from` call advances the markers. A premature `finalize_build` therefore fails to compile because the all-present impl does not apply. The partial type name is the reserved `__Partial{Name}`.

## Examples

A record is most useful when assembling one struct from field-compatible parts. Because both structs derive the record machinery, a builder can copy shared fields in bulk with `build_from` and fill the rest individually:

```rust
use cgp::prelude::*;
use cgp::core::field::impls::CanBuildFrom;

#[derive(CgpRecord)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
}

#[derive(CgpRecord)]
pub struct Employee {
    pub employee_id: u64,
    pub first_name: String,
    pub last_name: String,
}

fn promote(person: Person, id: u64) -> Employee {
    Employee::builder()
        .build_from(person)                                   // first_name + last_name
        .build_field(PhantomData::<Symbol!("employee_id")>, id)
        .finalize_build()
}
```

If a field were left unset before `finalize_build`, the call would not compile — the all-present `FinalizeBuild` impl simply would not be in scope for the partial type.

## Related constructs

`#[derive(CgpRecord)]` is the struct restriction of [`#[derive(CgpData)]`](derive_cgp_data.md), which dispatches to this same path; [`#[derive(CgpVariant)]`](derive_cgp_variant.md) is the enum counterpart. Its output decomposes into [`#[derive(HasField)]`](derive_has_field.md) (per-field getters), [`#[derive(HasFields)]`](derive_has_fields.md) (the representation traits), and [`#[derive(BuildField)]`](derive_build_field.md) (the incremental builder) — derive those individually when you need only one slice. The generated types reference [`Field`](../types/field.md), the [`product`](../macros/product.md) type-level list (`Cons`/`Nil`), and the `MapType` markers `IsPresent`/`IsNothing`/`IsVoid`.

## Source

The derive entry point is `derive_cgp_record` in [crates/macros/cgp-macro-lib/src/cgp_record.rs](../../../crates/macros/cgp-macro-lib/src/cgp_record.rs), which parses an `ItemCgpRecord` and calls `to_items()`. The record codegen is in [crates/macros/cgp-macro-core/src/types/cgp_data/record.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/record.rs), which composes `derive_has_field_impls_from_struct`, `derive_has_fields_impls_from_struct`, and the builder helpers in the `derive_builder/` submodule. The runtime traits (`HasBuilder`, `IntoBuilder`, `PartialData`, `FinalizeBuild`, `UpdateField`, `BuildField`, `HasFields`, `FromFields`, `ToFields`) live in [crates/core/cgp-field/src/traits/](../../../crates/core/cgp-field/src/traits/). Expansion snapshots and behavioral tests are in [crates/tests/cgp-tests/tests/extensible_data_tests/records/](../../../crates/tests/cgp-tests/tests/extensible_data_tests/records/).
