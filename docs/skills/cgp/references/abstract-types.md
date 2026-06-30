# Abstract types

An abstract type is a CGP trait carrying a single associated type that generic code names as `Self::Foo` without committing to a concrete type, leaving the concrete choice to wiring so it can differ from one context to another.

## The idea

An abstract type lets generic code name a type it does not fix. Instead of hard-coding `f64` or `String`, a trait declares one associated type — `trait HasNameType { type Name; }` — and code written against it refers to `Self::Name`, leaving the actual type open. The trait is the abstraction; the associated type is the slot a context fills in. This is the type-level analogue of an impl-side dependency: just as a getter lets a context supply a *value* a provider needs, an abstract-type trait lets a context supply a *type* a provider builds on.

The payoff is the same one CGP gives for behavior. A provider written in terms of `Self::Name` works unchanged whether a context chooses `String`, `&'static str`, or a custom name type, and two contexts can make different choices from the same generic code. Under the hood an abstract type is nothing more than an ordinary Rust trait with one associated type — generic functions constrain `Context: HasNameType` and use `Context::Name` exactly as they would any associated type. The CGP machinery only makes declaring and wiring these traits cheap.

## Direct implementation: it's just a trait

Because an abstract type is a vanilla associated-type trait, the most transparent way to bind it is to implement it directly on a concrete context. Given the trait, a context names the concrete type in a plain `impl`:

```rust
#[cgp_type]
pub trait HasNameType {
    type Name;
}

pub struct Person;

impl HasNameType for Person {
    type Name = String;
}
```

This makes `Person::Name` resolve to `String`, and any generic code constrained on `HasNameType` sees that choice. The direct form is the right mental model for newcomers — there is no hidden machinery, just an associated type pinned to a concrete one — and it is barely longer than the wired form shown below. The rest of this reference explains the `#[cgp_type]` macro and the wiring conveniences that make the choice swappable through a delegation table instead.

## Making a type swappable with `#[cgp_type]`

The `#[cgp_type]` macro turns an abstract-type trait into a full component, so the concrete type can be chosen through wiring rather than a hand-written impl. It is the abstract-type specialization of `#[cgp_component]`: applied to a trait with exactly one associated type and no methods, it produces everything `#[cgp_component]` would — the consumer trait, the provider trait, the blanket impls, the `…Component` marker — but each impl forwards the *associated type* rather than a method.

```rust
#[cgp_type]
pub trait HasNameType {
    type Name;
}
```

The default provider name is keyed off the *associated type* name, not the trait name, with a `TypeProvider` suffix. So `type Name;` yields the provider `NameTypeProvider` and the component marker `NameTypeProviderComponent`. A bound on the associated type — `type Name: Clone;` — is carried everywhere the type appears in the expansion and enforced on whatever concrete type a context chooses. You can override the derived provider name by passing one, exactly as with `#[cgp_component]`: `#[cgp_type(ProvideName)]`.

The construct that distinguishes `#[cgp_type]` from a plain component is an extra blanket impl it generates for the `UseType` provider, described next, which is what lets a context pick a concrete type without writing a provider of its own.

## Wiring a concrete type with `UseType`

A context binds an abstract type to a concrete one by wiring its provider component to `UseType<T>`. Because every abstract-type provider has the same trivial shape — "the associated type *is* this concrete type" — `#[cgp_type]` generates that shape once as a blanket impl of the provider trait for `UseType<Name>`, setting the associated type to the generic parameter. The context then names the concrete type directly in its delegation table:

```rust
delegate_components! {
    Person {
        NameTypeProviderComponent: UseType<String>,
    }
}
```

Wiring `NameTypeProviderComponent` to `UseType<String>` makes `Person` implement `HasNameType` with `Name = String`, with no bespoke provider, and any bound on the associated type is checked against `String` at the wiring site. `UseType<T>` is a zero-sized marker struct carrying no runtime value — it exists only to be named in a delegation table, never constructed. This is the type-level mirror of how `UseField` supplies a value-level getter (see [functions and getters](functions-and-getters.md)).

The blanket impl that `#[cgp_type]` generates for the example above is:

```rust
impl<Name, __Context__> NameTypeProvider<__Context__> for UseType<Name> {
    type Name = Name;
}
```

This says `UseType<T>` is a provider that supplies `T` as the abstract type, for any context. If the associated type carried a bound, that bound would be copied into the impl's `where` clause so the concrete type must satisfy it.

A word of caution on the name: the `UseType<T>` *provider struct* shown here is a different construct from the `#[use_type]` *attribute* covered below. The provider wires a concrete type *into* a context; the attribute imports an abstract type *into* a definition and rewrites bare mentions of it. They share a name because both center on abstract types, but they live in different places and do different jobs.

## The built-in `HasType` / `TypeProvider` component

Underneath every named abstract type sits CGP's single built-in abstract-type component, `HasType`. It is tag-indexed: `HasType<Tag>` is the consumer trait, `TypeProvider` is its provider trait, and a context can carry many distinct abstract types — one per `Tag` — resolving each through wiring.

```rust
#[cgp_component(TypeProvider)]
pub trait HasType<Tag> {
    type Type;
}
```

Every `#[cgp_type]` component you define is wired on top of this substrate: the macro generates an internal `WithProvider` impl that adapts a `TypeProvider` into the named component, so the same `UseType<T>` marker satisfies both the built-in `HasType` and any user-defined abstract type at once. In practice you rarely name `HasType<Tag>` directly — you define a readable `HasNameType` with `#[cgp_type]` and get `Self::Name` and its own provider, all resolving down to this `HasType` machinery. The takeaway is that `UseType<T>` is itself a `TypeProvider`, which is why one marker serves every abstract type.

## Abstract type as a getter return type

When an abstract type's only role is to be the return type of a getter, you can declare it inline with `#[cgp_auto_getter]` rather than defining a separate `#[cgp_type]` trait. The getter trait carries the associated type locally, and the field's type is inferred from it:

```rust
#[cgp_auto_getter]
pub trait HasName {
    type Name;

    fn name(&self) -> &Self::Name;
}
```

A context implementing this through its field wiring supplies both the concrete `Name` and the value. This keeps a one-off abstract type local to the getter that uses it instead of promoting it to a shared, wired component. See [functions and getters](functions-and-getters.md) for how `#[cgp_auto_getter]` derives the getter from a field.

## Importing an abstract type with `#[use_type]`

The strongly recommended way to *refer to* an abstract type from another definition is the `#[use_type]` attribute. A provider or component often needs a type that lives on a different trait — a `Scalar` from `HasScalarType`, an `Error` from `HasErrorType` — and Rust requires every mention to be written in fully-qualified form, `<Self as HasScalarType>::Scalar`, because a bare `Scalar` is not a type the compiler knows. Writing that prefix on every occurrence is verbose and easy to get wrong.

`#[use_type]` lets you write the bare identifier everywhere and have the macro expand it. You declare the import once alongside `#[cgp_fn]`, `#[cgp_impl]`, or `#[cgp_component]`, and the macro rewrites each standalone `Scalar` into `<Self as HasScalarType>::Scalar` while also adding `HasScalarType` as a supertrait (for `#[cgp_component]`) or a `where`-clause bound (for `#[cgp_impl]` and `#[cgp_fn]`). Consider a `rectangle_area` function that multiplies two implicit fields:

```rust
pub trait HasScalarType {
    type Scalar: Clone + Mul<Output = Self::Scalar>;
}

#[cgp_fn]
#[use_type(HasScalarType::Scalar)]
fn rectangle_area(
    &self,
    #[implicit] width: Scalar,
    #[implicit] height: Scalar,
) -> Scalar {
    width * height
}
```

The macro rewrites every bare `Scalar` to the qualified path and adds the supertrait/bound, so the effective trait and impl read:

```rust
pub trait RectangleArea: HasScalarType {
    fn rectangle_area(&self) -> <Self as HasScalarType>::Scalar;
}

impl<Context> RectangleArea for Context
where
    Self: HasField<Symbol!("width"), Value = <Self as HasScalarType>::Scalar>
        + HasField<Symbol!("height"), Value = <Self as HasScalarType>::Scalar>,
    Self: HasScalarType,
{
    fn rectangle_area(&self) -> <Self as HasScalarType>::Scalar {
        let width: <Self as HasScalarType>::Scalar =
            self.get_field(PhantomData::<Symbol!("width")>).clone();
        let height: <Self as HasScalarType>::Scalar =
            self.get_field(PhantomData::<Symbol!("height")>).clone();
        width * height
    }
}
```

The substitution is purely textual at the type level — it matches single-segment, argument-free type paths whose identifier equals the imported name — so a bare `Scalar` in the return type, an implicit-argument annotation, or a `let` binding inside the body is all rewritten the same way. Beyond saving keystrokes, the always-qualified rewrite removes the ambiguity the bare form cannot express, which is why this is the default way to import abstract types in all three macros.

The attribute has a few richer forms worth knowing. A leading `@` changes the rewrite target from `Self` to a named type, which imports a *foreign* abstract type from a generic parameter: `#[use_type(@Types::HasScalarType::Scalar)]` rewrites `Scalar` to `<Types as HasScalarType>::Scalar` and adds `Types: HasScalarType` as a `where` bound rather than a supertrait. A braced list imports several types from one trait, each optionally renamed with `as` or constrained with `=`: `#[use_type(HasScalarType::{Scalar = f64})]` both imports `Scalar` and emits `Self: HasScalarType<Scalar = f64>`, pinning it. The `= ...` equality form is rejected on `#[cgp_component]`, since a trait definition cannot carry the impl-side equality constraint it produces; it belongs on `#[cgp_fn]` and `#[cgp_impl]`.

## Sharing one type across contexts

The value of an abstract type compounds when several pieces of generic code share it. Because the type lives on a trait the context implements, every provider and trait that needs a `Scalar` refers to the *same* `Self::Scalar`, so a context fixes the choice once and all of them agree. This is sharpest when a trait's main subject is a generic parameter rather than the context itself — a `CanCalculateAreaOfShape<Shape>` implemented by one context for many shapes. The shapes carry no scalar type of their own; the shared context supplies a single `Scalar` through its `HasScalarType` wiring, and switching `UseType<f32>` to `UseType<f64>` changes the scalar for every shape at once. The same arrangement is how CGP shares one error type across an application: `HasErrorType` is itself defined with `#[cgp_type]`, and every fallible provider refers to the same `Self::Error` (see [error handling](error-handling.md)).

## Related references

Abstract types are wired into a context with [components](components.md) and consumed by [functions and getters](functions-and-getters.md). `UseType<T>` is one of the higher-order providers described in [higher-order providers](higher-order-providers.md), and the shared error type pattern is covered in [error handling](error-handling.md).

## Further reference

Online docs: [`#[cgp_type]`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/cgp_type.md), [`HasType`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/components/has_type.md), [`UseType` provider](https://github.com/contextgeneric/cgp/blob/main/docs/reference/providers/use_type.md), [`#[use_type]` attribute](https://github.com/contextgeneric/cgp/blob/main/docs/reference/attributes/use_type.md).
