# `#[cgp_component]`

`#[cgp_component]` is the foundational CGP macro: applied to a trait, it turns that ordinary Rust trait into a full CGP component — a consumer trait, a matching provider trait, and the blanket implementations that let a context delegate the trait's behavior to a freely chosen provider.

## Purpose

`#[cgp_component]` exists to lift a single trait into a pair of traits that separate *using* a capability from *implementing* it. A normal Rust trait conflates the two: the type that implements `Area` is the same type that callers invoke `.area()` on, and Rust's coherence rules then allow only one implementation per type. CGP breaks this conflation by generating two traits from one definition. The **consumer trait** is what callers use (`context.area()`); the **provider trait** is what implementers write, with the original `Self` moved into an explicit `Context` type parameter so that any number of provider types can implement it without colliding.

The payoff is that providers become first-class, named, swappable units. Because a provider implements the provider trait for a generic `Context` rather than for itself, the usual orphan and overlap restrictions do not bite, and a crate can define many alternative providers for the same component. A concrete context then picks one provider per component through wiring (see [`delegate_components!`](delegate_components.md)), and the generated blanket impls route the consumer-trait call through that choice. `#[cgp_component]` is what makes a trait participate in this mechanism; without it, a trait is just a vanilla Rust trait.

## Syntax

The macro is applied as an attribute on a trait definition and takes the provider trait's name as its argument. The simplest form passes a bare identifier:

```rust
#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}
```

Here `CanCalculateArea` is the consumer trait (named in verb form, `CanDoSomething`) and `AreaCalculator` is the provider trait (named in noun form). When more control is needed, the macro accepts a key/value form instead of a bare identifier:

```rust
#[cgp_component {
    name: AreaCalculatorComponent,
    provider: AreaCalculator,
    context: Context,
}]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}
```

The three keys correspond to the three names the macro needs, and each has a default. The `provider` key sets the provider trait name and is the only required value; passing a bare identifier is shorthand for setting `provider` alone. The `name` key sets the component name type and defaults to the provider name with a `Component` suffix, so `AreaCalculator` yields `AreaCalculatorComponent`. The `context` key sets the identifier used for the generic context type parameter in the generated provider trait and defaults to `__Context__` — a deliberately unusual name chosen to avoid clashing with the user's own type parameters.

Two companion attributes extend the macro for special cases and are documented separately. Adding [`#[derive_delegate(...)]`](../attributes/derive_delegate.md) generates `UseDelegate` providers that dispatch on a generic parameter, and adding [`#[extend(...)]`](../attributes/extend.md) adds supertrait bounds to the generated consumer trait. The related macros [`#[cgp_type]`](cgp_type.md) and [`#[cgp_getter]`](cgp_getter.md) build on `#[cgp_component]` to derive additional constructs for abstract-type and getter components respectively.

When a component's supertrait exists only to supply an abstract type the trait's own signatures name — most commonly [`HasErrorType`](../components/has_error_type.md), whose `Error` a fallible method returns — prefer importing that type with [`#[use_type]`](../attributes/use_type.md) over writing the supertrait and the qualified `Self::` path by hand. Annotating the trait with `#[use_type(HasErrorType::Error)]` adds `HasErrorType` as a supertrait *and* rewrites a bare `Error` in the signatures to `<Self as HasErrorType>::Error`, so the definition reads `pub trait CanLoad { fn load(&self, path: &str) -> Result<String, Error>; }` rather than spelling `: HasErrorType` and `Self::Error`. For a supertrait `#[use_type]` cannot express — a capability supertrait with no associated type to import, or a bound the trait does not name in its signatures — declare it with `#[extend(...)]` in preference to native `: Supertrait` syntax, which reads as OOP-style inheritance rather than the capability import a CGP supertrait actually is. A local associated type the trait declares itself, such as a handler's `type Output`, always stays written as `Self::Output`; it is not imported, so `#[use_type]` neither lists nor rewrites it.

## Syntax Grammar

The attribute argument of `#[cgp_component]` is either a bare provider name or a comma-separated set of keyed values:

```ebnf
CgpComponentArgs -> ProviderName
                  | KeyValueArg ( `,` KeyValueArg )* `,`?

ProviderName     -> IDENTIFIER

KeyValueArg      -> `name` `:` ComponentName
                  | `provider` `:` IDENTIFIER
                  | `context` `:` IDENTIFIER

ComponentName    -> IDENTIFIER GenericArgs?
```

`ProviderName` is the bare-identifier form and is shorthand for setting `provider` alone. In the key/value form each of the three keys may appear at most once and in any order, and `provider` is required — the other two have defaults (`context` is `__Context__`, `name` is the provider name with a `Component` suffix). `IDENTIFIER` is a Rust identifier token, and `GenericArgs` is the Rust grammar's `< … >` argument list (so the component name may carry generic parameters while the provider name may not). The attribute delimiter shown in Syntax — `(...)` for the bare form and `{...}` for the key/value form — is ordinary Rust attribute syntax; the argument tokens inside follow this grammar regardless of which delimiter is used.

## Expansion

`#[cgp_component]` replaces the annotated trait with five top-level items plus a set of standard provider impls. Starting from this input:

```rust
#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}
```

the macro emits, first, the **consumer trait** unchanged from its definition:

```rust
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}
```

Second, it emits the **provider trait**, which is the consumer trait with `Self` replaced by an explicit leading `Context` type parameter and every `self`/`Self` reference rewritten to `context`/`Context`. The provider trait carries an [`IsProviderFor`](../traits/is_provider_for.md) supertrait that captures the component name and context so that unsatisfied dependencies surface as readable compiler errors. The supertrait's third argument is the `Params` tuple of the component's extra type parameters; for a component with no parameters beyond the context it is the empty `()`:

```rust
pub trait AreaCalculator<Context>:
    IsProviderFor<AreaCalculatorComponent, Context, ()>
{
    fn area(context: &Context) -> f64;
}
```

Third, it emits the **consumer blanket impl**, which says that any context implementing the provider trait *for itself* automatically gets the consumer trait. This is the bridge that lets callers write `context.area()`:

```rust
impl<Context> CanCalculateArea for Context
where
    Context: AreaCalculator<Context>,
{
    fn area(&self) -> f64 {
        Context::area(self)
    }
}
```

Fourth, it emits the **provider blanket impl**, which lets any provider that delegates this component (via [`DelegateComponent`](../traits/delegate_component.md)) inherit the provider trait from the provider it delegates to. This is the mechanism `delegate_components!` drives — it turns a context into a type-level table whose entry for `AreaCalculatorComponent` names the chosen provider:

```rust
impl<Context, Provider> AreaCalculator<Context> for Provider
where
    Provider: DelegateComponent<AreaCalculatorComponent>
        + IsProviderFor<AreaCalculatorComponent, Context, ()>,
    Provider::Delegate: AreaCalculator<Context>,
{
    fn area(context: &Context) -> f64 {
        Provider::Delegate::area(context)
    }
}
```

Note that the `IsProviderFor` bound sits on `Provider` itself, beside the `DelegateComponent` bound — not on `Provider::Delegate`. The blanket impl of `IsProviderFor` (generated by `delegate_components!`) forwards a component's dependencies through `Provider`, so requiring `Provider: IsProviderFor<…>` is what threads those dependencies down the delegation chain and surfaces them in error messages.

Fifth, it emits the **component name struct**, a zero-sized marker that serves as the key into delegation tables:

```rust
pub struct AreaCalculatorComponent;
```

Beyond these five items, the macro also generates standard provider impls that make the component usable in the patterns CGP relies on. It emits a [`UseContext`](../providers/use_context.md) impl, so that the provider trait can be satisfied by routing back through a context's own consumer-trait implementation; a `RedirectLookup` impl, used by the namespace and preset machinery; and, for each [`#[derive_delegate(...)]`](../attributes/derive_delegate.md) attribute present, a [`UseDelegate`](../providers/use_delegate.md) impl that dispatches on the named generic parameter. When the component is defined inside a namespace, prefix impls are emitted as well (see [`#[cgp_namespace]`](cgp_namespace.md)).

Two details of the expansion are worth holding onto because they are easy to get wrong. The generated type parameters carry reserved names, not the readable ones used above: the context parameter is literally `__Context__` unless overridden, and the provider parameter in the provider blanket impl is `__Provider__`. The examples here use `Context` and `Provider` for legibility, but the emitted code uses the reserved names. And the generic parameters of a component with type parameters (for example `CanCalculateArea<Shape>`) are appended *after* the context parameter in the provider trait and grouped into a parenthesized list in the `IsProviderFor` `Params` position — so `IsProviderFor<AreaCalculatorComponent, __Context__, (Shape)>` for one parameter and `(Shape, Scalar)` for several — see generic parameters in the `/cgp` skill for the multi-parameter rules.

## Examples

A complete component, from definition through wiring to use, ties the pieces together. First the component and a provider for it:

```rust
use cgp::prelude::*;

#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea {
    fn area(&self) -> f64;
}

#[cgp_auto_getter]
pub trait HasDimensions {
    fn width(&self) -> &f64;
    fn height(&self) -> &f64;
}

#[cgp_impl(new RectangleArea)]
impl AreaCalculator
where
    Self: HasDimensions,
{
    fn area(&self) -> f64 {
        self.width() * self.height()
    }
}
```

Then a concrete context that wires the component to that provider and uses it:

```rust
#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

delegate_components! {
    Rectangle {
        AreaCalculatorComponent: RectangleArea,
    }
}

fn print_area(rect: &Rectangle) {
    println!("area = {}", rect.area()); // CanCalculateArea, via RectangleArea
}
```

The call `rect.area()` resolves through the consumer blanket impl to `Rectangle::area(rect)`, which resolves through the provider blanket impl to `RectangleArea::area(rect)` because `Rectangle`'s table maps `AreaCalculatorComponent` to `RectangleArea`.

## Related constructs

`#[cgp_component]` is the root that most other constructs attach to. [`#[cgp_impl]`](cgp_impl.md) and [`#[cgp_provider]`](cgp_provider.md) are the idiomatic ways to write providers for a component; [`#[cgp_fn]`](cgp_fn.md) is the lighter-weight alternative when only one implementation is ever needed. [`delegate_components!`](delegate_components.md) wires a component to a provider on a concrete context, and [`check_components!`](check_components.md) verifies at compile time that the wiring is complete. The specialized forms [`#[cgp_type]`](cgp_type.md) and [`#[cgp_getter]`](cgp_getter.md) extend `#[cgp_component]` for abstract types and getters. The attributes [`#[derive_delegate]`](../attributes/derive_delegate.md), [`#[extend]`](../attributes/extend.md), and [`#[use_type]`](../attributes/use_type.md) modify what the macro generates.

## Source

The macro entry point is `cgp_component` in [crates/macros/cgp-macro-lib/src/cgp_component.rs](../../../crates/macros/cgp-macro-lib/src/cgp_component.rs), which drives the `preprocess → eval → to_items` pipeline. The logic lives in [crates/macros/cgp-macro-core/src/types/cgp_component/](../../../crates/macros/cgp-macro-core/src/types/cgp_component/): argument parsing in `args/`, the provider trait and blanket impls in `preprocessed/`, and the standard provider impls (`UseContext`, `RedirectLookup`, `UseDelegate`) in `evaluated/`. The default identifiers `__Context__` and `{Provider}Component` are set in `args/component_args.rs`. Behavioral and expansion-snapshot tests are in [crates/tests/cgp-tests](../../../crates/tests/cgp-tests) and [crates/tests/cgp-macro-tests](../../../crates/tests/cgp-macro-tests).
