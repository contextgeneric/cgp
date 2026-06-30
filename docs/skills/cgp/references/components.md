# Components

A CGP **component** is the bundle that `#[cgp_component]` generates from one trait so that *using* a capability and *implementing* it become separate, swappable things — this is the central reference; read it first.

## What a component is

A component is a single trait definition compiled into a small machine: a **consumer trait** that callers invoke, a **provider trait** that implementations target, a `…Component` marker key, and the blanket impls that connect them. An ordinary Rust trait conflates using a capability with implementing it — the type you call `.area()` on is the same type that supplies the `area` body — and Rust's coherence rules then allow only one implementation per type. A component breaks that conflation, which is what lets many independent implementations of the same capability coexist and lets you implement a capability for a type you do not own.

The split is produced by `#[cgp_component]`, applied to an ordinary trait. Throughout this file the running examples are the greeting component (`CanGreet` / `Greeter` / `GreetHello`, on a `Person`) and the area component (`CanCalculateArea` / `AreaCalculator` / `RectangleArea`, on a `Rectangle`). Assume `use cgp::prelude::*;` in every snippet; the CGP version is v0.7.0.

## What `#[cgp_component(Greeter)]` generates

Applying the macro to a consumer trait produces five items: the consumer trait, the provider trait, two blanket impls, and the marker struct. Start from the trait, naming the provider trait in the attribute argument:

```rust
#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}
```

The first item is the **consumer trait**, emitted unchanged. This is the self-style trait callers use (`CanDoX`), so a caller writes `person.greet()` exactly as with any trait:

```rust
pub trait CanGreet {
    fn greet(&self);
}
```

The second item is the **provider trait**, the same interface with `Self` moved out into an explicit leading `Context` type parameter and every `self`/`Self` rewritten to `context`/`Context`. It is named in noun form (`SomethingDoer`, or `…Provider` when no noun fits), and it carries an `IsProviderFor` supertrait that captures the component, the context, and a `Params` tuple of any extra type parameters — `()` when there are none:

```rust
pub trait Greeter<Context>:
    IsProviderFor<GreeterComponent, Context, ()>
{
    fn greet(context: &Context);
}
```

A provider trait is implemented not for the context but for a dedicated zero-sized **provider** struct — a type-level marker that is never instantiated and carries no runtime value. Because a provider implements `Greeter<Context>` for *its own* struct over a generic `Context`, the orphan and overlap rules never bite, so `GreetHello`, `GreetGoodbye`, and any number of further providers for the same component can all exist at once. See [bypassing coherence](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/coherence.md) (online) for why moving `Self` to a parameter is what sidesteps the rules, and [modularity hierarchy](modularity-hierarchy.md) for the spectrum of how far to take the split.

## How the two traits connect

Two generated blanket impls bridge the consumer and provider sides, and together they make `person.greet()` resolve to a chosen provider without the caller naming it. Read them as the wiring machinery; you never write them.

The first is the **consumer blanket impl**, which says that any context implementing the provider trait *for itself* automatically gets the consumer trait. It forwards `context.greet()` to `Context::greet(self)`:

```rust
impl<Context> CanGreet for Context
where
    Context: Greeter<Context>,
{
    fn greet(&self) {
        Context::greet(self)
    }
}
```

The second is the **provider blanket impl**, which lets any provider that delegates this component inherit the provider trait from whatever it delegates to. The delegation is a type-level table lookup through `DelegateComponent`, keyed on the `GreeterComponent` marker:

```rust
impl<Context, Provider> Greeter<Context> for Provider
where
    Provider: DelegateComponent<GreeterComponent>
        + IsProviderFor<GreeterComponent, Context, ()>,
    Provider::Delegate: Greeter<Context>,
{
    fn greet(context: &Context) {
        Provider::Delegate::greet(context)
    }
}
```

The fifth item is the **component marker**, a zero-sized key into those delegation tables:

```rust
pub struct GreeterComponent;
```

These two examples use readable names for clarity; the emitted code uses reserved identifiers — `__Context__` for the context parameter (overridable) and `__Provider__` for the provider parameter — chosen so they never clash with a user's own type names.

## Wiring is the table lookup that ties it all together

**Wiring** is the step that supplies the delegation the provider blanket impl reads, by making a context into a type-level table whose entry for each component names the chosen provider. With the greeting component wired, the chain resolves end to end: `person.greet()` goes through the consumer impl to `Person` implementing the provider trait for itself, which goes through the provider impl to the table entry, landing on the selected provider's `greet`. Swapping the table entry is the only change needed to swap behavior; no caller is touched.

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

The full table grammar — including per-value dispatch and presets — lives in [wiring](wiring.md). This file shows only the single-entry form needed to make the resolution concrete.

## Why `IsProviderFor` exists

`IsProviderFor` is an empty marker trait that rides along on every provider trait as a supertrait, and its only job is to make a missing dependency produce a readable error. A provider lists what it needs from the context in a `where` clause; when that clause is unmet, the bare question "does this provider implement the provider trait?" yields only "trait not implemented", because the provider blanket impl is also a candidate and Rust suppresses its detailed reasoning whenever more than one impl could apply.

`IsProviderFor` is the second, independent path that un-hides the real reason. The macros implement it for a provider under *exactly the same* `where` bounds as the provider trait, and because that impl is the only candidate (no competing blanket), Rust commits to it and prints the precise unsatisfied constraint. The practical translation a reader needs: an `IsProviderFor` not implemented error means the provider trait is not implemented, and the named bound is the missing dependency. The trait is generated and consumed entirely by the macros — you observe it in errors, you never write it.

```rust
#[diagnostic::on_unimplemented(
    note = "You need to add `#[cgp_provider({Component})]` on the impl block for CGP provider traits"
)]
pub trait IsProviderFor<Component, Context, Params: ?Sized = ()> {}
```

## Writing providers

A provider can be written at three levels of sugar over the same machinery, and `#[cgp_impl]` is the one to reach for. The lower forms exist for when the native provider-trait shape is wanted explicitly, and they are what `#[cgp_impl]` desugars to.

The lowest form is `#[cgp_provider]`, applied to a provider-trait impl written directly on a provider struct. It passes the impl through unchanged and auto-generates the matching `IsProviderFor` impl from the same `where` clause, so the dependency set can never drift. The provider struct must already exist:

```rust
pub struct RectangleArea;

#[cgp_provider]
impl<Context> AreaCalculator<Context> for RectangleArea
where
    Context: HasDimensions,
{
    fn area(context: &Context) -> f64 {
        context.width() * context.height()
    }
}
```

That expands to the impl above plus the empty marker impl carrying the same bound, which is what surfaces a missing `HasDimensions` as a named error:

```rust
impl<Context> IsProviderFor<AreaCalculatorComponent, Context, ()> for RectangleArea
where
    Context: HasDimensions,
{}
```

The optional attribute argument overrides the component type, defaulting otherwise to the provider trait's name plus a `Component` suffix; pass it explicitly only when the trait name does not follow that convention.

The middle form is `#[cgp_new_provider]`, identical to `#[cgp_provider]` but it also declares the provider struct, folding `pub struct RectangleArea;` into the same block. Use it when introducing a fresh provider so you need not write the struct separately; a generic provider yields a struct with a `PhantomData` field over its parameters.

The preferred form is `#[cgp_impl]`, which lets you write the provider in **consumer-style syntax** — keeping `self`, `Self`, and the consumer trait's method signatures — and mechanically rewrites it into the provider-trait shape. The provider name moves into the attribute argument instead of the `Self` position; a leading `new` keyword declares the struct; and you may omit `for Context` entirely, letting the macro insert the context parameter for you:

```rust
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

The one rule this convenience must not let you forget: inside a `#[cgp_impl]` block, `self` and `Self` mean the **context**, never the provider struct. The provider has no runtime value, so the macro rewrites every `self` to the context value and every `Self` to the context type — those are the only things that exist when the method runs.

`#[cgp_impl]` desugars to `#[cgp_provider]` (or `#[cgp_new_provider]` when `new` is given). The block above is equivalent to writing the context out by hand and lowering it:

```rust
#[cgp_new_provider]
impl<__Context__> AreaCalculator<__Context__> for RectangleArea
where
    __Context__: HasDimensions,
{
    fn area(__context__: &__Context__) -> f64 {
        __context__.width() * __context__.height()
    }
}
```

The receiver `&self` became `__context__: &__Context__` — the receiver identifier is the snake-cased context name wrapped in double underscores — and `Self` in the `where` clause became the context type. When you write `for Context` explicitly, that name is used instead of `__Context__`, which is worth doing when you need to bound or refer to the context readably.

## Impl-side dependencies

A provider states what it needs from the context as bounds in its `where` clause, and those bounds — **impl-side dependencies** — are constraints the consumer trait never exposes. `CanCalculateArea` declares only `area`, while `RectangleArea` requires `Context: HasDimensions`; a caller bounding on `CanCalculateArea` never sees `HasDimensions`, so the requirement stays one level down and does not cascade through transitive callers. This is dependency injection through the `where` clause: any context satisfying the bound gains the capability automatically, and the wiring satisfies each bound by resolving it through the same context. See [impl-side dependencies](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/impl-side-dependencies.md) (online) for the full treatment, and [functions and getters](functions-and-getters.md) for the value-injection forms (`#[implicit]` arguments and getters) that read fields off the context through the same mechanism.

## A consumer trait is still an ordinary trait

A consumer trait can also be implemented directly on a context, exactly like a vanilla Rust trait, when code reuse across providers is not the goal. The consumer/provider split is a superset of ordinary traits, not a replacement: the provider machinery is what you opt into when a capability needs more than one implementation, and skipping it costs nothing for the simple case. You write `impl CanGreet for Person { ... }` as usual, and `person.greet()` resolves to that direct impl with no wiring involved.

## Further reference

For the full expansion and accepted syntax of each macro, see the online docs: [`#[cgp_component]`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/cgp_component.md), [`#[cgp_impl]`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/cgp_impl.md), [`#[cgp_provider]`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/cgp_provider.md), [`#[cgp_new_provider]`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/macros/cgp_new_provider.md), and [`IsProviderFor`](https://github.com/contextgeneric/cgp/blob/main/docs/reference/traits/is_provider_for.md). For the concepts behind the split, see [consumer and provider traits](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/consumer-and-provider-traits.md), [bypassing coherence](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/coherence.md), and [impl-side dependencies](https://github.com/contextgeneric/cgp/blob/main/docs/concepts/impl-side-dependencies.md). For sibling sub-skills, see [wiring](wiring.md), [checking](checking.md), [functions and getters](functions-and-getters.md), and [higher-order providers](higher-order-providers.md).
