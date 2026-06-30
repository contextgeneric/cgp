# Functions and Getters

The ergonomic surface of basic CGP — `HasField` field access, `#[cgp_fn]` single-implementation capabilities, `#[implicit]` arguments, and getter traits — the constructs that let CGP code read like ordinary Rust functions and accessors.

## How field access works underneath: `HasField`

Every value a provider reads out of its context flows through one tiny consumer trait, `HasField<Tag>`, which keys a single field by a *type-level* name rather than by the concrete struct it lives in. A provider is generic over its context and cannot reach into a struct it does not know, so instead of naming the field directly it demands one by tag: a `where`-clause bound `Context: HasField<Symbol!("name"), Value = String>` says "any context wired to me must carry a `String` field called `name`," and the trait system supplies it. This makes field access an [impl-side dependency](components.md) — a requirement hidden from the trait interface and satisfied automatically by any matching context. Assume `use cgp::prelude::*;` throughout; the CGP version is v0.7.0.

The trait carries the field's type as an associated `Value` and returns a reference, taking a `PhantomData<Tag>` argument whose only job is to tell the compiler which field is meant when several `HasField` impls are in scope:

```rust
pub trait HasField<Tag> {
    type Value;
    fn get_field(&self, _tag: PhantomData<Tag>) -> &Self::Value;
}
```

The `Tag` is a type-level name, and CGP has two kinds. A named struct field is keyed by `Symbol!("field_name")`, the type-level string of its identifier; a tuple field is keyed by `Index<N>`, the type-level natural number of its position. Both are [type-level primitives](type-level-primitives.md) — types with no values — which is exactly why `get_field` needs the `PhantomData<Tag>` argument to carry one at the call site. A `HasFieldMut<Tag>: HasField<Tag>` companion adds `get_field_mut` returning `&mut Self::Value` for the rarer mutable case.

Reading a field is the PhantomData tag-inference trick in practice. Inside a provider body you write `self.get_field(PhantomData)` and let Rust infer the tag from the surrounding bound, or pin it explicitly with `self.get_field(PhantomData::<Symbol!("name")>)` when more than one field could match:

```rust
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

You almost never write `HasField` impls by hand. The companion derive does it for you.

## Generating field access: `#[derive(HasField)]`

`#[derive(HasField)]` turns a struct's concrete fields into the type-level entries the trait system looks up, emitting one `HasField` and one `HasFieldMut` impl per field and leaving the struct definition untouched. It is the bridge between an ordinary Rust struct and constraint-based field access: without it, a struct's fields are invisible to CGP, and every getter would be hand-written. For a named struct,

```rust
#[derive(HasField)]
pub struct Person {
    pub name: String,
    pub age: u8,
}
```

the derive emits a `Symbol!`-keyed pair per field — `impl HasField<Symbol!("name")> for Person { type Value = String; … }` reading `&self.name`, and the same for `age` — so `Person` satisfies `HasField<Symbol!("name"), Value = String>` exactly as the `GreetHello` bound above requires. A tuple struct expands identically except each field is keyed by its positional `Index<N>` instead of a `Symbol!`:

```rust
#[derive(HasField)]
pub struct Rectangle(pub f64, pub f64);
// → impl HasField<Index<0>> for Rectangle { type Value = f64; … &self.0 }
// → impl HasField<Index<1>> for Rectangle { type Value = f64; … &self.1 }
```

Generic parameters thread through faithfully, and field access also flows through smart pointers — `HasField` has a blanket impl over any `Deref` target that has the field, so a `Box<Person>` resolves `get_field` to the inner struct. Deriving `HasField` is the one thing a context must do for every higher-level construct on this page — `#[cgp_fn]`, `#[implicit]`, and both getter macros all desugar into `HasField` bounds.

## Single-implementation capabilities: `#[cgp_fn]`

`#[cgp_fn]` turns a plain function into a CGP capability that every context gains automatically, with no wiring step at all. A full [component](components.md) defines a consumer trait, a provider trait, and a delegation table so that many providers can be swapped per context; `#[cgp_fn]` is the lightweight counterpart for the common case where a capability has a single natural definition. You write the body as if `self` were concrete, and the macro emits a trait plus a *blanket* impl over a generic context — so the method becomes available on every type that satisfies the body's impl-side dependencies, with no `delegate_components!` block and no provider struct anywhere.

The function name in snake case becomes the method name, and the trait name defaults to that name in PascalCase. Given:

```rust
#[cgp_fn]
pub fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}
```

the macro emits a `RectangleArea` trait whose method takes no arguments, and a blanket impl over the reserved context type `__Context__` in which each `#[implicit]` parameter became a `HasField` bound and a `get_field` binding at the top of the body:

```rust
pub trait RectangleArea {
    fn rectangle_area(&self) -> f64;
}

impl<__Context__> RectangleArea for __Context__
where
    Self: HasField<Symbol!("width"), Value = f64>
        + HasField<Symbol!("height"), Value = f64>,
{
    fn rectangle_area(&self) -> f64 {
        let width: f64 = self.get_field(PhantomData::<Symbol!("width")>).clone();
        let height: f64 = self.get_field(PhantomData::<Symbol!("height")>).clone();
        width * height
    }
}
```

That generated blanket impl is the whole point of the macro, so it is worth seeing: the context type parameter is literally `__Context__` and references to it inside the impl read as `Self`. Pass an identifier to override the default trait name, which is useful when a verb-style name reads better — `#[cgp_fn(CanCalculateRectangleArea)]` generates `CanCalculateRectangleArea` instead of `RectangleArea`.

Generics and `where` clauses are handled with a deliberate split: every generic parameter in the function's `<...>` list goes onto *both* the trait and the impl, while the function's `where` bounds land only on the impl, hidden from the trait as impl-side dependencies. A generic area function makes this concrete:

```rust
#[cgp_fn]
pub fn rectangle_area<Scalar>(
    &self,
    #[implicit] width: Scalar,
    #[implicit] height: Scalar,
) -> Scalar
where
    Scalar: Mul<Output = Scalar> + Copy,
{
    width * height
}
```

Here `Scalar` appears on `RectangleArea<Scalar>` and its impl, while `Scalar: Mul<Output = Scalar> + Copy` stays on the impl only, ordered before the implicit `HasField` bounds (which are always appended last). One restriction is intentional: `#[cgp_fn]` does not support generics on the *method* itself — method-level generics belong to the trait and impl, and the rare genuine need for them is an advanced case better written as an explicit blanket impl or a full component.

## Field access dressed as a function argument: `#[implicit]`

The `#[implicit]` attribute marks a function argument as sourced from a context field instead of from the caller, so a provider reads like a function taking parameters while behaving like one injecting dependencies. This is the recommended on-ramp to CGP: a programmer who understands functions and arguments can write a complete provider without first meeting `HasField`, `Symbol!`, or `PhantomData`. Strongly prefer implicit arguments in basic code — they keep CGP looking like ordinary Rust. The argument name doubles as the field name, so `#[implicit] width: f64` reads as "this function needs a `width` of type `f64`," and the macro removes the argument from the signature, adds the matching `HasField` bound, and binds the value at the top of the body (as the `#[cgp_fn]` expansion above shows).

The argument type controls how the field is read, following a small set of rules so the body always receives exactly the declared type. An owned type such as `f64` or `String` reads the field by reference and appends `.clone()`, leaving the context's field intact; the one special case to memorize is `&str`, which is backed by a `String` field and read with `.as_str()` rather than `.clone()`, letting the body borrow without forcing the context to store a `&str`:

```rust
#[cgp_fn]
pub fn greet(&self, #[implicit] name: &str) {
    println!("Hello, {}!", name);
}
// bound: HasField<Symbol!("name"), Value = String>
// binding: let name: &str = self.get_field(PhantomData::<Symbol!("name")>).as_str();
```

Three rules constrain where `#[implicit]` may appear. The function must take `self` as its first argument, since the field is read from it; the argument pattern must be a bare identifier, not a destructuring or `mut` pattern (clone inside the body for a mutable local); and a `&mut self` receiver allows at most one implicit argument, since each borrows from the same context. `#[implicit]` is usable in both `#[cgp_fn]` and the methods of a `#[cgp_impl]` provider, with the same desugaring in each — inside `#[cgp_impl]` the `HasField` bounds simply join the provider impl's `where` clause.

## Importing capabilities: `#[uses]`

`#[uses(...)]` adds `Self: Trait<...>` bounds to a provider's `where` clause, written to read like a `use` import of the capabilities the body depends on. A provider that calls another `#[cgp_fn]` capability, or a [component](components.md) consumer trait, must require the context to implement it — a `where Self: SomeTrait` bound that reads as machinery. `#[uses(RectangleArea)]` instead reads as "this function uses the `RectangleArea` capability," and the macro turns each listed name into the corresponding `Self` bound on the impl so the body can call those methods directly on `self`. Building a scaled area on top of the base one:

```rust
#[cgp_fn]
#[uses(RectangleArea)]
pub fn scaled_rectangle_area(&self, #[implicit] scale_factor: f64) -> f64 {
    self.rectangle_area() * scale_factor * scale_factor
}
```

This adds `Self: RectangleArea` to the generated impl's `where` clause, alongside the `HasField` bound from the implicit `scale_factor` — the imported bound lands on the impl only, never on the trait, exactly like writing `where Self: RectangleArea` by hand. The syntax accepts only the simplified `TraitIdent<Params>` form, deliberately: because it is meant to read like an import, it does not accept associated-type-equality bounds such as `Iterator<Item = u8>` — write those as an explicit `where` clause in the body instead. `#[uses(...)]` works in both `#[cgp_fn]` and `#[cgp_impl]`, and the imported capability may itself be defined either way.

## Adding supertraits and trait bounds: `#[extend]` and `#[extend_where]`

In `#[cgp_fn]`, the function's own `where` clauses are impl-side dependencies kept off the trait, so there is no place to write a supertrait by hand — `#[extend(...)]` is the only way to add one. Where `#[uses]` adds a hidden impl-side bound (the `use` equivalent), `#[extend]` promotes its bound to a *supertrait* of the generated trait — a public requirement every implementor satisfies and every caller may rely on (the `pub use` equivalent). The bound appears in two places: as a supertrait on the trait, so an associated type like `Self::Scalar` resolves and callers know the bound holds, and in the impl's `where` clause so the body can use it. A `#[cgp_fn]` over an abstract scalar type:

```rust
pub trait HasScalarType {
    type Scalar: Clone + Mul<Output = Self::Scalar>;
}

#[cgp_fn]
#[extend(HasScalarType)]
pub fn rectangle_area(
    &self,
    #[implicit] width: Self::Scalar,
    #[implicit] height: Self::Scalar,
) -> Self::Scalar {
    width * height
}
// → pub trait RectangleArea: HasScalarType { fn rectangle_area(&self) -> Self::Scalar; }
```

`#[extend]` accepts the same simplified `TraitIdent<Params>` syntax as `#[uses]`, and is also usable on `#[cgp_component]` (where it simply duplicates the native `pub trait Foo: Bar` supertrait syntax for stylistic consistency). Its sibling `#[extend_where(...)]` adds *`where`-clause* predicates to the generated trait definition rather than supertraits, and is `#[cgp_fn]`-only. Unlike `#[uses]` and `#[extend]`, it accepts arbitrary predicates — including associated-type equality — so a generic parameter can carry a publicly visible bound:

```rust
#[cgp_fn]
#[extend_where(Scalar: Clone)]
pub fn rectangle_area<Scalar>(/* … */) -> Scalar
where
    Scalar: Mul<Output = Scalar>,
{ /* … */ }
// → pub trait RectangleArea<Scalar> where Scalar: Clone { fn rectangle_area(&self) -> Scalar; }
```

The `Scalar: Mul` bound from the body stays an impl-side dependency, while `Scalar: Clone` from `#[extend_where]` is promoted onto the trait so any code naming `RectangleArea<Scalar>` can rely on it without restating it.

## Getter traits: `#[cgp_auto_getter]`

A getter trait exposes a context field as a reusable `self.name()` accessor, and `#[cgp_auto_getter]` generates its single blanket impl by reading the field whose name matches the method name. Where an [implicit argument](#field-access-dressed-as-a-function-argument-implicit) injects a value once at the start of one method, a getter is the better tool when the same field is read across many methods or part-way through a body. The macro takes no arguments and re-emits the trait verbatim, adding a blanket impl over `__Context__` keyed by the method name as a `Symbol!`:

```rust
#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

// generated:
impl<__Context__> HasName for __Context__
where
    __Context__: HasField<Symbol!("name"), Value = String>,
{
    fn name(&self) -> &str {
        self.get_field(PhantomData::<Symbol!("name")>).as_str()
    }
}
```

That generated blanket impl is the point of the macro, and it follows the same access rules as `#[implicit]`: a plain `&T` return reads a `T` field directly, while the `&str` shorthand reads a `String` field and appends `.as_str()`. Other shorthands include `Option<&T>` (an `Option<T>` field via `.as_ref()`), `&[T]` (a field implementing `AsRef<[T]>`), an owned `T` (a `Copy` field by value), and `&mut T` with a `&mut self` receiver (mutable access via `get_field_mut`). A trait may declare several methods, each mapping independently to its own field — `fn width(&self) -> &f64; fn height(&self) -> &f64;` produces one `where` predicate and one body per field in the same impl.

A single getter may also declare a local associated type and use it as its return type, which lets the abstract type be inferred from the field. The trait must then contain exactly one method returning `&Self::AssocType`; the macro lifts the type into a generic parameter on the impl and binds it through the `HasField` `Value`:

```rust
#[cgp_auto_getter]
pub trait HasName {
    type Name: Display;
    fn name(&self) -> &Self::Name;
}
// → impl<__Context__, Name> HasName for __Context__
//   where Name: Display, __Context__: HasField<Symbol!("name"), Value = Name>
//   { type Name = Name; … }
```

A context gains the getter just by deriving `HasField` with a matching field — `person.name()` resolves through the blanket impl with no wiring. The cost of that simplicity is rigidity: the field name *must* equal the method name, and there is no way to swap the implementation. When you need either, reach for `#[cgp_getter]`.

## Wireable getters: `#[cgp_getter]` and `UseField`

`#[cgp_getter]` defines a getter as a full CGP [component](components.md) instead of a blanket impl, so the field name can differ from the method name and the getter can be swapped per context through [wiring](wiring.md). It accepts the same getter-method forms as `#[cgp_auto_getter]`, but because it is an extension of `#[cgp_component]` it needs a provider trait name. The default derives one from the trait name by stripping a leading `Has` and appending `Getter`, so `HasName` yields the provider `NameGetter` and the component marker `NameGetterComponent`; pass an argument like `#[cgp_getter(GetName)]` to override it.

The decoupling is delivered by an automatically generated `UseField<Tag>` provider impl. `UseField<Tag>` is a zero-sized provider (a `PhantomData`-only marker named in wiring, carrying no runtime value) that implements the getter by reading the field named `Tag` from the context — and crucially, `Tag` need not be the method name:

```rust
#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[derive(HasField)]
pub struct Person {
    pub first_name: String,
}

delegate_components! {
    Person {
        NameGetterComponent: UseField<Symbol!("first_name")>,
    }
}
// person.name() now reads the first_name field
```

The trait method is `name` but the context stores the value in `first_name`, and the wiring `NameGetterComponent: UseField<Symbol!("first_name")>` bridges the two — the field name lives in the wiring, not in the trait. Internally `#[cgp_getter]` generates a `UseField` impl whose tag is left as a free parameter, in contrast to the `#[cgp_auto_getter]` blanket impl that hard-codes the tag to the method name. The macro also emits a `UseFields` provider (the provider-side analogue of the auto-getter blanket impl, keyed by method name) and, for single-method getters, a `WithProvider` adapter.

For getters whose return type is reached *through* a field by `AsRef`/`AsMut` rather than being the field itself, the related `UseFieldRef<Tag, Value>` provider reads the field at `Tag` and calls `as_ref()` to expose `&Value` — for example `UseFieldRef<Symbol!("name"), str>` exposes `&str` from a `String` field. It decouples the exposed type from the stored type as well as the field name from the method name. Unlike `UseField`, it is not re-exported through the prelude; reach it through `cgp::core::field::impls`.

## Getters are just traits: explicit implementation

Every getter the macros produce is an ordinary trait, and explicit implementation is always available — the macros only save boilerplate. This matters when a context does not derive `HasField`, or stores the value under a name no tag matches. Because `#[cgp_auto_getter]` adds only a blanket impl and `#[cgp_getter]` a component whose consumer trait is plain Rust, you can write the impl by hand on a concrete type:

```rust
pub struct Person {
    pub full_name: String,
}

impl HasName for Person {
    fn name(&self) -> &str {
        &self.full_name
    }
}
```

The explicit form is more verbose but requires no understanding of `HasField` or blanket impls — a reminder that the whole apparatus on this page is convenience layered over vanilla Rust traits.

## Choosing between the constructs

The constructs here divide along two axes: how the value is read, and how flexible the implementation is. For a value consumed once at the start of a method, an `#[implicit]` argument keeps the access local and the code reading like a plain function. For a field read across several methods or mid-body, a getter trait exposes it as a reusable accessor; pick `#[cgp_auto_getter]` when the field name always matches the method name and no alternative is needed, and `#[cgp_getter]` (with `UseField`) when the field name must differ or the getter must be swappable per context. For a whole capability rather than a single field, `#[cgp_fn]` defines one with no wiring when a single implementation suffices, and a full [component](components.md) when many providers must coexist. All of them rest on the same `HasField` machinery and the same access rules, so mixing them carries no conceptual overhead.

## Further reference

Online docs (current v0.7.0): [`#[cgp_fn]`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/cgp_fn.md), [`HasField`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/traits/has_field.md), [`#[derive(HasField)]`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/derives/derive_has_field.md), [`#[implicit]`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/attributes/implicit.md), [`#[uses]`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/attributes/uses.md), [`#[extend]`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/attributes/extend.md), [`#[extend_where]`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/attributes/extend_where.md), [`#[cgp_auto_getter]`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/cgp_auto_getter.md), [`#[cgp_getter]`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/cgp_getter.md), [`UseField`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/providers/use_field.md), [`UseFieldRef`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/providers/use_field_ref.md), and the [implicit-arguments concept](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/implicit-arguments.md).
