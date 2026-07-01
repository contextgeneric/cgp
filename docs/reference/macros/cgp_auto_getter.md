# `#[cgp_auto_getter]`

`#[cgp_auto_getter]` turns a getter trait into a single blanket implementation that satisfies each getter method by reading a context field through [`HasField`](../traits/has_field.md), keyed by the method's own name as a [`Symbol!`](symbol.md).

## Purpose

`#[cgp_auto_getter]` exists to publish a context field as a reusable getter *capability* — a named `self.name()` accessor that other providers depend on. Reading a value out of a context is the most basic form of dependency injection in CGP, and the underlying mechanism — `HasField<Symbol!("name"), Value = String>` — is precise but verbose and unfamiliar to most Rust developers. The macro lets you state the intent in ordinary trait-method syntax (`fn name(&self) -> &str;`) and generates the `HasField`-based plumbing for you.

Reach for `#[cgp_auto_getter]` when the field is a shared capability, not when a single provider needs a value for its own use. For the common case of reading a field inside one provider, an [`#[implicit]`](../attributes/implicit.md) argument is the preferred, lighter form: it injects the value as an ordinary-looking parameter with no separate trait to declare. A getter trait earns its keep when the accessor is genuinely shared — depended on by several providers through [`#[uses]`](../attributes/uses.md), or carrying an associated type inferred from the field so the type stays abstract. Both desugar to the same `HasField` bound and share the same access rules, so the choice is about whether the value is a published capability or a private input.

The defining trade-off of `#[cgp_auto_getter]` is that it produces exactly one implementation and no CGP wiring. Unlike a full component, there is no provider trait, no component name, and no delegation table; the macro emits a blanket impl over a generic context, and any context that derives `HasField` with a matching field automatically satisfies the trait. This makes it the lightest-weight getter construct — ideal when the field name in the context always matches the method name and you never need an alternative implementation.

The cost of that simplicity is rigidity. Because the field tag is derived directly from the method name, the context *must* expose a field of exactly that name. When you need the field name to differ from the method name, or you want the getter to be swappable through wiring, [`#[cgp_getter]`](cgp_getter.md) builds the same convenience on top of a real CGP component — but that is an advanced tool reserved for when a context needs full control over which field a getter reads from, and most getters do not. Prefer `#[cgp_auto_getter]` for a published accessor and an implicit argument for a private read; reach for `#[cgp_getter]` only when field-name decoupling is specifically wanted.

## Syntax

The macro is applied as a bare attribute on a trait definition and takes no arguments. The trait body consists of getter methods, each taking `&self` (or `&mut self`) and returning a reference:

```rust
#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}
```

A getter trait may declare several methods, and each maps independently to its own field. Each method name becomes the field tag, so the following injects two separate fields:

```rust
#[cgp_auto_getter]
pub trait HasDimensions {
    fn width(&self) -> &f64;
    fn height(&self) -> &f64;
}
```

The return type controls how the field value is read, and several shorthand forms are recognized so the method signature stays ergonomic. A plain reference `&T` reads a field of type `T` directly. The form `&str` is treated specially: it reads a `String` field and calls `.as_str()` on it, so you can return `&str` while the context stores a `String`. Other recognized forms include `Option<&T>` (an `Option<T>` field returned via `.as_ref()`), `&[T]` (a field whose value implements `AsRef<[T]>`), and an owned `T` (a `Copy` field returned by value). A `&mut self` receiver with a `&mut T` return reads the field mutably through `get_field_mut`.

A getter trait may also declare a single associated type and use it as the method's return type, which lets the abstract type be inferred from the field. This is covered under Expansion below. When an associated type is present, the trait must contain exactly one getter method, and that method's return type must be `&Self::AssocType`.

## Expansion

`#[cgp_auto_getter]` re-emits the trait unchanged and adds one blanket impl over a generic context. Starting from this input:

```rust
#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}
```

the macro produces the trait verbatim plus the following blanket impl. The context type parameter is the reserved name `__Context__`, the field tag is the method name rendered as a `Symbol!`, and because the return type is `&str`, the `Value` is `String` and the method appends `.as_str()`:

```rust
pub trait HasName {
    fn name(&self) -> &str;
}

impl<__Context__> HasName for __Context__
where
    __Context__: HasField<Symbol!("name"), Value = String>,
{
    fn name(&self) -> &str {
        self.get_field(PhantomData::<Symbol!("name")>).as_str()
    }
}
```

A trait with multiple getter methods produces one `where` predicate and one method body per field, all within the same blanket impl. The `&f64` returns below are plain references, so each `Value` matches the return type and no conversion is appended:

```rust
impl<__Context__> HasDimensions for __Context__
where
    __Context__: HasField<Symbol!("width"), Value = f64>,
    __Context__: HasField<Symbol!("height"), Value = f64>,
{
    fn width(&self) -> &f64 {
        self.get_field(PhantomData::<Symbol!("width")>)
    }

    fn height(&self) -> &f64 {
        self.get_field(PhantomData::<Symbol!("height")>)
    }
}
```

When the trait declares an associated type used as the return type, the macro lifts that type into an extra generic parameter on the impl and binds it through the `HasField` `Value`. This is what allows the abstract type to be inferred from the concrete field. Given:

```rust
#[cgp_auto_getter]
pub trait HasName {
    type Name: Display;

    fn name(&self) -> &Self::Name;
}
```

the blanket impl carries `Name` as a generic parameter, copies the associated type's bounds into the `where` clause, and sets the associated type to that parameter:

```rust
impl<__Context__, Name> HasName for __Context__
where
    Name: Display,
    __Context__: HasField<Symbol!("name"), Value = Name>,
{
    type Name = Name;

    fn name(&self) -> &Self::Name {
        self.get_field(PhantomData::<Symbol!("name")>)
    }
}
```

These desugarings are the exact shape the macro emits today; the only cosmetic difference from the snapshots is that `Symbol!("name")` is shown here in its sugared form rather than the expanded `Symbol<4, Chars<'n', ...>>`.

## Examples

A complete use pairs the getter trait with a context that derives `HasField`, after which the method is available with no further wiring:

```rust
use cgp::prelude::*;

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

fn greet(person: &Person) {
    println!("Hello, {}!", person.name()); // HasName, via the blanket impl
}
```

`Person` derives `HasField<Symbol!("name"), Value = String>`, which is precisely the bound the blanket impl requires, so `Person` implements `HasName` automatically and `person.name()` returns the `name` field as a `&str`.

A getter trait can always be implemented explicitly instead, which is useful when the context does not derive `HasField` or does not store the value under the matching field name. Because `#[cgp_auto_getter]` only adds a blanket impl, you may write the impl by hand on a concrete type:

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

The explicit form is more verbose but requires no understanding of `HasField` or blanket impls, and it demonstrates that an auto-getter trait is an ordinary Rust trait — the macro's only job is to save you from writing the boilerplate body.

## Related constructs

`#[cgp_auto_getter]` is the blanket-impl counterpart to [`#[cgp_getter]`](cgp_getter.md): both read context fields through `HasField`, but `#[cgp_getter]` produces a full CGP component that can be wired to a [`UseField`](../providers/use_field.md) provider, allowing the field name to differ from the method name and the getter to be swapped per context. It builds directly on [`#[derive(HasField)]`](../derives/derive_has_field.md), which generates the per-field `HasField` impls keyed by [`Symbol!`](symbol.md). For field access inside a method body rather than through a dedicated trait, the [`#[implicit]`](../attributes/implicit.md) argument attribute follows the same field-reading semantics. When the only purpose of a getter's associated type is to serve as its return type, the associated-type form here overlaps with abstract-type components defined by [`#[cgp_type]`](cgp_type.md).

## Source

The macro entry point is `cgp_auto_getter` in [crates/macros/cgp-macro-lib/src/cgp_auto_getter.rs](../../../crates/macros/cgp-macro-lib/src/cgp_auto_getter.rs), which rejects any attribute argument and runs `ItemCgpAutoGetter::preprocess(...).to_items()`. The logic lives in [crates/macros/cgp-macro-core/src/types/cgp_auto_getter/](../../../crates/macros/cgp-macro-core/src/types/cgp_auto_getter/): `item.rs` sets the `__Context__` context identifier and drives field parsing, and `blanket.rs` builds the single blanket impl. Getter parsing — including the `&str`/`Option<&T>`/`&[T]`/owned shorthands and the associated-type rules — is shared with `#[cgp_getter]` in [crates/macros/cgp-macro-core/src/functions/getter/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/getter/parse.rs), [crates/macros/cgp-macro-core/src/functions/field/parse.rs](../../../crates/macros/cgp-macro-core/src/functions/field/parse.rs), and [crates/macros/cgp-macro-core/src/types/getter/](../../../crates/macros/cgp-macro-core/src/types/getter/). Behavioral and expansion-snapshot tests are in [crates/tests/cgp-tests/tests/getters/](../../../crates/tests/cgp-tests/tests/getters/) (notably `assoc_type_auto_getter.rs` for the associated-type form and `string_auto.rs` for the `&str` shorthand).
