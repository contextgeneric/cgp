# `HasRuntime`

`HasRuntime` is the runtime abstraction in CGP: a pair of components that lets a context declare an abstract runtime *type* through `HasRuntimeType` and hand out a borrow of the runtime *value* through `HasRuntime`, with `RuntimeOf<Context>` as the alias for the resolved runtime type.

## Purpose

`HasRuntime` exists so that context-generic code can run asynchronous and effectful operations against a runtime without committing to a concrete one. A runtime in this sense is whatever object provides the capabilities an application needs at execution time — spawning tasks, sleeping, opening sockets, reading the clock — and different deployments want different runtimes (Tokio in production, a mock in tests, a single-threaded executor in a benchmark). Rather than thread a concrete runtime type through every signature, CGP lets the context name an abstract `Runtime` type and store one runtime value, and lets providers reach it generically through these two traits.

The abstraction is split deliberately into a type component and a getter component because the two questions are independent. `HasRuntimeType` answers *what* the runtime type is — an abstract associated type chosen per context — while `HasRuntime` answers *how to obtain the runtime value* of that type from a borrow of the context. Some code is generic only over the runtime type (it never touches a runtime value, only names types the runtime exposes); that code needs `HasRuntimeType` alone. Code that actually performs effects needs `HasRuntime`, which supertraits `HasRuntimeType` so the value's type is always in scope. Keeping them separate means a context can declare its runtime type in one place and supply the value in another, and a bound asks for exactly the capability it uses.

This pair is the substrate beneath CGP's task-running components. The [`Runner`](runner.md) family expresses "run this task," and the providers that implement it typically reach the runtime through `HasRuntime` to spawn or await the work. `HasRuntime` is the seam where context-generic logic meets the concrete async machinery, which is why it underpins asynchronous execution across a CGP application.

## Definition

The two components are declared in `cgp-runtime`. `HasRuntimeType` is an abstract-type component defined with [`#[cgp_type]`](../macros/cgp_type.md):

```rust
#[cgp_type]
pub trait HasRuntimeType {
    type Runtime;
}

pub type RuntimeOf<Context> = <Context as HasRuntimeType>::Runtime;
```

Because the trait carries no provider-name argument, `#[cgp_type]` derives the provider name from the associated type: `Runtime` yields the provider trait `RuntimeTypeProvider` and the component marker `RuntimeTypeProviderComponent`. The `Runtime` associated type carries no bound, so any concrete type may be plugged in. The `RuntimeOf<Context>` alias is the convenient spelling of the resolved runtime type, used wherever writing `<Context as HasRuntimeType>::Runtime` in full would be noise.

`HasRuntime` is a getter component defined with [`#[cgp_getter]`](../macros/cgp_getter.md), and it imports the runtime type with [`#[use_type(HasRuntimeType::Runtime)]`](../attributes/use_type.md) so the runtime type is available as the getter's return type under the bare name `Runtime`:

```rust
#[cgp_getter]
#[use_type(HasRuntimeType::Runtime)]
pub trait HasRuntime {
    fn runtime(&self) -> &Runtime;
}
```

`#[cgp_getter]` derives the provider name from the trait name by stripping the `Has` prefix and appending `Getter`, so `HasRuntime` yields the provider trait `RuntimeGetter` and the component marker `RuntimeGetterComponent`. `#[use_type(HasRuntimeType::Runtime)]` adds `HasRuntimeType` as a supertrait and rewrites the bare `Runtime` to `<Self as HasRuntimeType>::Runtime`, so the `runtime` method borrows the runtime value out of a borrow of the context, returning `&Runtime` — the abstract type supplied by `HasRuntimeType` — without writing `Self::Runtime` by hand.

## Behavior

`HasRuntimeType` behaves like any `#[cgp_type]` abstract-type component, so a context supplies its runtime *type* either by implementing `HasRuntimeType` directly or — far more commonly — by wiring `RuntimeTypeProviderComponent` to [`UseType<R>`](../providers/use_type.md) in `delegate_components!`. Wiring `RuntimeTypeProviderComponent: UseType<TokioRuntime>` makes the context resolve `HasRuntimeType` with `Runtime = TokioRuntime`, and from then on `RuntimeOf<Context>` is `TokioRuntime`. The full set of generated constructs — the consumer and provider blanket impls, the `UseContext` and `RedirectLookup` provider impls, and the `UseType`/`WithProvider` impls that make `UseType<R>` resolve the type — is exactly the `#[cgp_type]` expansion described in [`#[cgp_type]`](../macros/cgp_type.md).

`HasRuntime` behaves like any `#[cgp_getter]` getter component, so a context supplies its runtime *value* either by implementing `HasRuntime` directly or by wiring `RuntimeGetterComponent` to a [`UseField`](../providers/use_field.md) provider that names the field holding the runtime. Because `#[cgp_getter]` generates a `UseField` provider impl, a context that stores its runtime in a `runtime` field wires `RuntimeGetterComponent: UseField<Symbol!("runtime")>`, and the getter reads that field. The `HasRuntimeType` supertrait that `#[use_type]` adds means a context cannot satisfy `HasRuntime` without also having declared its runtime type, which keeps the returned `&Runtime` well-defined.

Together the two components let a context fully describe its runtime: one wiring entry fixes the abstract type, another fixes where the value lives. Context-generic providers then write `where Self: HasRuntime` and call `self.runtime()` to obtain a `&RuntimeOf<Self>`, never naming the concrete runtime. This is what makes the same task-running and effect-performing code reusable across a Tokio context, a mock context, and a test context with no changes beyond the wiring.

## Examples

A context declares its runtime type and the field holding the runtime value, and a provider reaches the runtime generically:

```rust
use cgp::prelude::*;
use cgp::extra::runtime::{HasRuntime, HasRuntimeType, RuntimeOf};

pub struct TokioRuntime { /* handle, clock, etc. */ }

#[derive(HasField)]
pub struct App {
    pub runtime: TokioRuntime,
}

delegate_components! {
    App {
        RuntimeTypeProviderComponent:
            UseType<TokioRuntime>,
        RuntimeGetterComponent:
            UseField<Symbol!("runtime")>,
    }
}
```

Here `App` resolves `HasRuntimeType` with `Runtime = TokioRuntime` through `UseType`, so `RuntimeOf<App>` is `TokioRuntime`, and it resolves `HasRuntime` by reading its `runtime` field through `UseField`. A context-generic function can now demand only the capability it needs:

```rust
fn runtime_of<Context>(context: &Context) -> &RuntimeOf<Context>
where
    Context: HasRuntime,
{
    context.runtime()
}
```

The function names neither `TokioRuntime` nor any field; it works for any context that wires a runtime type and a runtime getter. Swapping `UseType<TokioRuntime>` for `UseType<MockRuntime>` in a test context retargets every such function at the mock with no change to their bodies.

## Related constructs

`HasRuntimeType` is defined with [`#[cgp_type]`](../macros/cgp_type.md), which generates the abstract-type machinery and the [`UseType`](../providers/use_type.md) impl a context uses to fix its runtime type; it builds on the foundational [`HasType`/`TypeProvider`](has_type.md) substrate. `HasRuntime` is defined with [`#[cgp_getter]`](../macros/cgp_getter.md) and backed by a [`UseField`](../providers/use_field.md) provider that reads the runtime value from a named field. The [`Runner`](runner.md) family — `CanRun<Code>` and `CanSendRun<Code>` — is the primary consumer of this abstraction: task-running providers reach the runtime through `HasRuntime` to execute work asynchronously, which connects `HasRuntime` to the [handler](handler.md) family of effectful components. Both components are wired on a context with [`delegate_components!`](../macros/delegate_components.md).

## Source

- `HasRuntimeType`, the `RuntimeTypeProvider` provider trait, and the `RuntimeOf` alias are defined in [crates/extra/cgp-runtime/src/traits/has_runtime_type.rs](../../../crates/extra/cgp-runtime/src/traits/has_runtime_type.rs).
- `HasRuntime` and its `RuntimeGetter` provider trait are in [crates/extra/cgp-runtime/src/traits/has_runtime.rs](../../../crates/extra/cgp-runtime/src/traits/has_runtime.rs), re-exported through [crates/extra/cgp-runtime/src/lib.rs](../../../crates/extra/cgp-runtime/src/lib.rs) and reached from the facade as `cgp::extra::runtime`.
- The `#[cgp_type]` and `#[cgp_getter]` expansions these rely on live under [crates/macros/cgp-macro-core/src/types/](../../../crates/macros/cgp-macro-core/src/types/).
