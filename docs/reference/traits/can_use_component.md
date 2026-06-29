# `CanUseComponent`

`CanUseComponent<Component, Params>` is the context-side check trait that holds whenever a context both delegates a component and its chosen provider is a valid provider for it, giving `check_components!` a single bound to assert that forces readable wiring errors.

## Purpose

`CanUseComponent` exists to answer one question about a context: "can this context actually use this component, with every dependency satisfied?" That question is deliberately not the same as "does the context implement the consumer trait?". Asking the consumer-trait question directly makes the compiler report the outermost unmet bound — usually a bare "the provider does not implement the provider trait" — and hide the indirect reasoning behind it, because the provider blanket impl is a competing candidate that suppresses detailed diagnostics. The root cause, often a single missing getter or abstract type, never appears.

`CanUseComponent` reframes the question along a path Rust will explain. It is satisfied only when two things hold together: the context delegates the component (it has a [`DelegateComponent`](delegate_component.md) entry for it), and the delegated provider satisfies [`IsProviderFor`](is_provider_for.md) for that exact context and parameter set. Because `IsProviderFor` carries the provider's real `where` bounds, requiring it forces the compiler to evaluate and report those bounds. The detailed error that was hidden behind the consumer-trait question becomes visible once the same wiring is probed through `CanUseComponent` instead.

The trait is mechanism, not API. Application code never names it; it is the bound that [`check_components!`](../macros/check_components.md) emits so that a wiring mistake is reported at the wiring site, pinpointing the absent dependency, rather than erupting at some distant call to the consumer trait.

## Definition

`CanUseComponent` is an empty marker trait parameterized by a component and a parameter tuple, with a single blanket impl supplying all of its instances:

```rust
pub trait CanUseComponent<Component, Params: ?Sized = ()> {}

impl<Context, Component, Params: ?Sized> CanUseComponent<Component, Params> for Context
where
    Context: DelegateComponent<Component>,
    Context::Delegate: IsProviderFor<Component, Context, Params>,
{
}
```

`Self` is the context being checked. `Component` is the component-name type, the same key used in the delegation table, and `Params` collects the component's extra generic parameters — a single parameter directly, several as a tuple, and the unit tuple `()` (the default) when there are none. The trait has no methods and no associated items; its meaning lives entirely in the blanket impl's `where` clause.

## Behavior

The blanket impl is the whole behavior, and its two bounds are the two halves of "can use". The first, `Context: DelegateComponent<Component>`, requires the context to have a table entry for the component — without a delegation there is nothing to use, and the lookup fails with the `DelegateComponent` diagnostic about a missing entry. The second, `Context::Delegate: IsProviderFor<Component, Context, Params>`, requires the delegated provider to be a genuine provider for this component, for this context, at these parameters. This is where dependency checking happens: `IsProviderFor` carries the provider's `where` bounds, so the compiler must satisfy them to grant `CanUseComponent`, and an unmet bound is reported in full.

`CanUseComponent` is mirror to [`IsProviderFor`](is_provider_for.md): it asks the same readability-restoring question, but indexed on the context (`Self = Context`) rather than the provider (`Self = Provider`). One starts from "the context delegates and the delegate is valid"; the other from "this specific provider is valid". `check_components!` uses `CanUseComponent` for its default, context-oriented checks and switches to `IsProviderFor` for its `#[check_providers(...)]` form, when a particular provider must be verified independently.

Routing a check through `CanUseComponent` rather than the consumer trait is what makes errors legible. Because the blanket impl threads the dependency bounds through `IsProviderFor`, an unsatisfied transitive requirement is surfaced and attributed to the actual missing piece. The trait is empty and the impl is unconditional in form, so satisfying it costs nothing at runtime; the only effect is the compile-time obligation its `where` clause imposes.

The two bounds also distinguish the two ways wiring can be wrong, and the diagnostics differ accordingly. A context that never delegated the component fails the first bound, and the `DelegateComponent` `on_unimplemented` note reports a missing table entry — the fix is to add the wiring. A context that delegated the component to a provider whose dependencies are unmet passes the first bound but fails the second, and the error is the provider's own unsatisfied `where` clause, carried up through `IsProviderFor` — the fix is to supply the missing dependency. Reading which bound failed tells a reader whether the wiring is absent or merely incomplete.

`check_components!` consumes the trait by emitting a private check trait whose supertrait is `CanUseComponent` and then one empty impl of that trait per checked entry; each impl compiles only if its `CanUseComponent` supertrait holds, so the entire table is an assertion that every listed component is usable. Because the assertion is a compile-time construct that produces no values, a successful build is the passing test and the macro adds nothing to the binary.

## Examples

`CanUseComponent` is asserted, never called. A `check_components!` table reduces to one impl of an internal check trait whose supertrait is `CanUseComponent`, so the impl compiles only if the bound holds:

```rust
use cgp::prelude::*;

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasName,
{
    fn greet(&self) {
        println!("Hello, {}!", self.name());
    }
}

#[derive(HasField)]
pub struct Person {
    pub first_name: String, // not `name`
}

delegate_components! {
    Person { GreeterComponent: GreetHello }
}

check_components! {
    Person { GreeterComponent }
}
```

The `check_components!` table forces the assertion `Person: CanUseComponent<GreeterComponent, ()>`. Resolving it requires `Person: DelegateComponent<GreeterComponent>` (satisfied by the wiring) and `GreetHello: IsProviderFor<GreeterComponent, Person, ()>` (not satisfied, because `GreetHello` needs `HasName` and `Person` has no `name` field). The compiler reports the absent `name` field at the check, rather than at a future `person.greet()` call. Writing the same bound directly probes the wiring without the macro:

```rust
fn assert_wiring()
where
    Person: CanUseComponent<GreeterComponent, ()>,
{}
```

## Related constructs

`CanUseComponent` is the bound that [`check_components!`](../macros/check_components.md) asserts to verify a context's wiring, and the macro's whole purpose is to route checking through it for readable errors. Its blanket impl is built directly on [`DelegateComponent`](delegate_component.md) — the context must delegate the component — and [`IsProviderFor`](is_provider_for.md) — the delegate must be a valid provider — so it ties the two wiring traits together into a single context-side question. It is the context-indexed counterpart of `IsProviderFor`, which `check_components!`'s `#[check_providers(...)]` form asserts on providers directly when a higher-order provider stack needs each layer checked. The components it checks are defined by [`#[cgp_component]`](../macros/cgp_component.md) and wired by [`delegate_components!`](../macros/delegate_components.md).

## Source

The trait and its sole blanket impl are defined in [crates/core/cgp-component/src/traits/can_use_component.rs](../../../crates/core/cgp-component/src/traits/can_use_component.rs) and re-exported through [crates/core/cgp-component/src/macro_prelude.rs](../../../crates/core/cgp-component/src/macro_prelude.rs). The checks that assert it are generated by `check_components!`, whose codegen lives in [crates/macros/cgp-macro-core/src/types/check_components/](../../../crates/macros/cgp-macro-core/src/types/check_components/) — `table.rs` chooses `CanUseComponent` versus `IsProviderFor` as the check trait's supertrait. Expansion snapshots are in [crates/tests/cgp-tests/src/tests/check_components.rs](../../../crates/tests/cgp-tests/src/tests/check_components.rs) and [crates/tests/cgp-tests/tests/namespace_tests/namespace_macro/default_impls.rs](../../../crates/tests/cgp-tests/tests/namespace_tests/namespace_macro/default_impls.rs).
