# `#[derive(HasField)]`

`#[derive(HasField)]` is the derive macro that gives a struct field-level getters: for every field it generates a `HasField<Tag>` (and `HasFieldMut<Tag>`) implementation keyed by a type-level name, so that providers can read fields out of a context by name without the field ever appearing in a trait interface.

## Purpose

`#[derive(HasField)]` exists to turn the concrete fields of a struct into type-level entries that CGP's dependency-injection machinery can look up. The recurring problem in CGP is that a provider needs a value from the context — a `name`, a `width`, a configuration handle — but the provider is generic over the context type and cannot name the concrete struct. `HasField` solves this by indexing each field with a *tag type* that stands in for the field's name, so a provider can demand `Context: HasField<Symbol!("name"), Value = String>` and receive the field without knowing what the context actually is.

The reason this matters is that it makes field access an impl-side dependency rather than part of any public interface. A provider expresses "I need a `String` field called `name`" purely in its `where` clause; any context that derives `HasField` and happens to have such a field satisfies it automatically. The derive is the bridge between an ordinary Rust struct and that constraint-based access — without it, the struct's fields are invisible to the trait system, and every getter would have to be hand-written.

The trait being implemented is small. It carries the field's type as an associated `Value` and returns a reference to the field, with a `PhantomData<Tag>` parameter that exists only to tell the compiler which field is meant when several `HasField` impls are in scope:

```rust
pub trait HasField<Tag> {
    type Value;

    fn get_field(&self, _tag: PhantomData<Tag>) -> &Self::Value;
}
```

Higher-level constructs are built directly on these generated impls. [`#[cgp_auto_getter]`](../macros/cgp_auto_getter.md) and [`#[cgp_getter]`](../macros/cgp_getter.md) (through the [`UseField`](../providers/use_field.md) provider) generate blanket impls whose `where` clauses are `HasField` bounds, and the [`#[implicit]`](../attributes/implicit.md) argument form desugars function parameters into `get_field` calls. All of them assume the context has derived `HasField`; this derive is what makes them work.

## Syntax

The macro is applied as a derive on a struct definition and takes no arguments:

```rust
#[derive(HasField)]
pub struct Person {
    pub name: String,
    pub age: u8,
}
```

It applies to structs with named fields and to structs with unnamed (tuple) fields. The two cases differ only in how each field's tag is computed: a named field is keyed by [`Symbol!("field_name")`](../macros/symbol.md), the type-level string of its identifier, while a tuple field is keyed by [`Index<N>`](../types/index.md), the type-level natural number of its position. Unit structs produce no impls because they have no fields.

The derive concerns itself only with the *field-level* view. To obtain the whole-struct view as a single type-level [`Product`](../macros/product.md), derive [`HasFields`](derive_has_fields.md) instead; the two are complementary and frequently derived together.

## Expansion

`#[derive(HasField)]` emits one `HasField` impl and one `HasFieldMut` impl per field, leaving the struct definition itself untouched. Starting from a named-field struct:

```rust
#[derive(HasField)]
pub struct Person {
    pub name: String,
    pub age: u8,
}
```

the macro generates a pair of impls for each field, with the field's identifier turned into a `Symbol!` tag and the field's type used as `Value`:

```rust
impl HasField<Symbol!("name")> for Person {
    type Value = String;

    fn get_field(&self, key: PhantomData<Symbol!("name")>) -> &Self::Value {
        &self.name
    }
}

impl HasFieldMut<Symbol!("name")> for Person {
    fn get_field_mut(&mut self, key: PhantomData<Symbol!("name")>) -> &mut Self::Value {
        &mut self.name
    }
}

impl HasField<Symbol!("age")> for Person {
    type Value = u8;

    fn get_field(&self, key: PhantomData<Symbol!("age")>) -> &Self::Value {
        &self.age
    }
}

impl HasFieldMut<Symbol!("age")> for Person {
    fn get_field_mut(&mut self, key: PhantomData<Symbol!("age")>) -> &mut Self::Value {
        &mut self.age
    }
}
```

The `HasFieldMut` impls come from the same derive and provide mutable access; `HasFieldMut<Tag>` is a supertrait extension of `HasField<Tag>` that adds a `get_field_mut` method returning `&mut Self::Value`. Most CGP code only reads through `HasField`, but the mutable counterpart is always generated alongside it.

A tuple struct expands the same way, except that each field's tag is its positional `Index<N>` rather than a `Symbol!`. Starting from:

```rust
#[derive(HasField)]
pub struct Rectangle(pub f64, pub f64);
```

the macro generates:

```rust
impl HasField<Index<0>> for Rectangle {
    type Value = f64;

    fn get_field(&self, key: PhantomData<Index<0>>) -> &Self::Value {
        &self.0
    }
}

impl HasFieldMut<Index<0>> for Rectangle {
    fn get_field_mut(&mut self, key: PhantomData<Index<0>>) -> &mut Self::Value {
        &mut self.0
    }
}

impl HasField<Index<1>> for Rectangle {
    type Value = f64;

    fn get_field(&self, key: PhantomData<Index<1>>) -> &Self::Value {
        &self.1
    }
}

impl HasFieldMut<Index<1>> for Rectangle {
    fn get_field_mut(&mut self, key: PhantomData<Index<1>>) -> &mut Self::Value {
        &mut self.1
    }
}
```

When the struct has generic parameters, the impls carry them through faithfully: the macro splits the struct's generics into impl-generics, type-generics, and `where` clause, so `struct Wrapper<T> { pub value: T }` yields `impl<T> HasField<Symbol!("value")> for Wrapper<T>` with `Value = T`.

Field access also threads through smart pointers without an explicit derive. `HasField<Tag>` and `HasFieldMut<Tag>` have blanket impls for any type whose `Deref`/`DerefMut` target implements them, so a `Box<Person>` or a newtype that dereferences to `Person` resolves `get_field` to the inner struct's field. These blanket impls carry a `#[diagnostic::do_not_recommend]` attribute so the compiler does not suggest them in error messages, keeping the missing-field diagnostic pointed at the underlying struct.

## Examples

A provider that needs a value from its context expresses the need as a `HasField` bound, and the derive is what lets a concrete context satisfy it. First a provider for a greeting component, asking for a `String` field named `name`:

```rust
use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasField<Symbol!("name"), Value = String>,
{
    fn greet(&self) {
        println!("Hello, {}!", self.get_field(PhantomData));
    }
}
```

Then a context that derives `HasField` and wires the component to that provider:

```rust
#[derive(HasField)]
pub struct Person {
    pub name: String,
}

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}
```

Because `Person` derives `HasField`, it implements `HasField<Symbol!("name"), Value = String>`, which is exactly the bound `GreetHello` requires; the wiring therefore type-checks and `person.greet()` prints the person's name.

In practice the explicit `HasField` bound is rarely written by hand. The same `name` access is more idiomatically expressed with [`#[cgp_auto_getter]`](../macros/cgp_auto_getter.md) (`fn name(&self) -> &str`) or with an [`#[implicit]`](../attributes/implicit.md) argument (`fn greet(&self, #[implicit] name: String)`), both of which generate the `HasField` bound for you. The derive is the foundation those forms stand on.

## Related constructs

`#[derive(HasField)]` underpins most value-level dependency injection in CGP. [`#[cgp_auto_getter]`](../macros/cgp_auto_getter.md) turns a getter-trait method into a blanket impl backed by a `HasField` bound, and [`#[cgp_getter]`](../macros/cgp_getter.md) does the same through the [`UseField`](../providers/use_field.md) provider, which reads an arbitrary field by tag. The [`#[implicit]`](../attributes/implicit.md) argument form desugars context parameters into `get_field` calls against these impls. The tags it generates are documented in [`Symbol!`](../macros/symbol.md) (named fields) and [`Index<N>`](../types/index.md) (tuple fields). For the aggregate view of all fields at once, see [`#[derive(HasFields)]`](derive_has_fields.md), which is commonly derived alongside this one and is itself the basis for [`#[derive(CgpData)]`](derive_cgp_data.md).

## Source

The derive entry point is `derive_has_field` in [crates/macros/cgp-macro-lib/src/derive_has_field.rs](../../../crates/macros/cgp-macro-lib/src/derive_has_field.rs), registered as the `HasField` proc-macro derive in [crates/macros/cgp-macro/src/lib.rs](../../../crates/macros/cgp-macro/src/lib.rs). It parses the input as a `syn::ItemStruct`, wraps it in an `ItemCgpRecord` ([crates/macros/cgp-macro-core/src/types/cgp_data/record.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/record.rs)), and calls `to_has_field_impls`, whose codegen lives in `derive_has_field_impls_from_struct` in [crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_field.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_data/derive_has_field.rs) — this is where named fields are mapped to `Symbol` tags and unnamed fields to `Index` tags, and where both the `HasField` and `HasFieldMut` impls are emitted. The traits themselves are defined in [crates/core/cgp-field/src/traits/has_field.rs](../../../crates/core/cgp-field/src/traits/has_field.rs) and [crates/core/cgp-field/src/traits/has_field_mut.rs](../../../crates/core/cgp-field/src/traits/has_field_mut.rs). Expansion snapshots are in [crates/tests/cgp-macro-tests](../../../crates/tests/cgp-macro-tests).
