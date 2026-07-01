# `IsProviderFor`

`IsProviderFor<Component, Context, Params>` is the marker trait that every CGP provider trait carries as a supertrait, propagating a provider's `where`-bounds so that an unmet dependency surfaces as a readable compiler error instead of being silently hidden.

## Purpose

`IsProviderFor` exists to make missing dependencies diagnosable. A CGP provider implements its provider trait under a `where` clause that lists everything it needs from the context — a getter, an abstract type, another component. When that clause is unmet, the natural question "does this provider implement the provider trait?" produces a frustrating answer: Rust reports only that the trait is not implemented, with no explanation, because a competing candidate (the provider blanket impl from [`#[cgp_component]`](../macros/cgp_component.md)) is also in scope and Rust suppresses the detailed reasoning whenever more than one impl could apply. The real cause — a single absent field or type — stays buried.

`IsProviderFor` is the second, independent path that un-hides those errors. It is an empty marker trait, trivially satisfiable in principle, but the macros implement it for a provider under *exactly the same* `where` bounds as the provider trait itself. Because every generated provider trait names `IsProviderFor` as a supertrait, asking whether a provider satisfies `IsProviderFor` forces the compiler to evaluate those bounds — and since that question has only one candidate impl (the explicit one the macro wrote, not a blanket), Rust does not hide its reasoning and reports the precise unsatisfied constraint. The trait carries no behavior; its entire value is making the dependency set visible to the compiler along a path Rust will explain.

This is why the dedicated, full treatment lives here while other documents mention `IsProviderFor` only in passing. To a user writing components and providers, it is invisible plumbing generated and consumed by the macros. To anyone reading a wiring error or building higher-order providers, it is the mechanism that turns an opaque failure into a pointer at the actual gap.

The phenomenon it works around is specific to how Rust prunes diagnostics. When two impls could satisfy a bound — here, the provider blanket impl that routes through the delegation table, and the user's explicit provider impl — Rust reports only that no impl matched, withholding the per-impl reasons because it cannot know which candidate the programmer intended. `IsProviderFor` sidesteps the ambiguity by being implemented along a single, explicit path with no competing blanket, so the compiler commits to that path and prints why its `where` clause failed.

## Definition

`IsProviderFor` is an empty trait parameterized by a component, a context, and a parameter tuple:

```rust
#[diagnostic::on_unimplemented(
    note = "You need to add `#[cgp_provider({Component})]` on the impl block for CGP provider traits"
)]
pub trait IsProviderFor<Component, Context, Params: ?Sized = ()> {}
```

The three parameters identify which provider trait implementation is being asserted. `Component` is the component-name type that corresponds to the provider trait, the same key used in the delegation table. `Context` is the context type the provider trait is implemented for. `Params` collects any additional generic parameters of the provider trait, grouped into a tuple when there is more than one and defaulting to the unit tuple `()` when there are none — so a provider trait with parameters `<I, J>` becomes `Params = (I, J)`, and a parameterless one uses the default. `Self` is the provider type whose validity is being asserted. The `#[diagnostic::on_unimplemented]` note nudges a reader who hits the error toward the missing `#[cgp_provider]` attribute that would generate the impl.

## Behavior

`IsProviderFor` is generated, never hand-written, and it appears in three places that together form the diagnostic chain. First, [`#[cgp_component]`](../macros/cgp_component.md) emits every provider trait with `IsProviderFor` as a supertrait, so that for a component `Foo<I, J>` the provider trait reads `pub trait FooGetterAt<Context, I, J>: IsProviderFor<FooGetterAtComponent, Context, (I, J)>`. This supertrait link is what binds a provider's dependency set to the marker.

Second, [`#[cgp_provider]`](../macros/cgp_provider.md) and [`#[cgp_impl]`](../macros/cgp_impl.md) emit, beside the provider trait impl, an `IsProviderFor` impl for the same provider type under the same `where` clause. A provider implemented as `impl<I, J> FooGetterAt<Context, I, J> for GetFooValue where Context: HasField<Symbol!("foo"), Value = u64>` gains a matching empty impl `impl<Context, I, J> IsProviderFor<FooGetterAtComponent, Context, (I, J)> for GetFooValue where Context: HasField<Symbol!("foo"), Value = u64> {}`. The two impls carry identical bounds, so the marker is satisfiable exactly when the provider trait is.

Third, [`delegate_components!`](../macros/delegate_components.md) propagates the marker through the delegation table. For each entry it emits an `IsProviderFor` impl on the table type that forwards to the delegated provider's own `IsProviderFor`, generic over context and params:

```rust
impl<Context, Params> IsProviderFor<FooGetterAtComponent, Context, Params> for MyAppComponents
where
    GetFooValue: IsProviderFor<FooGetterAtComponent, Context, Params>,
{}
```

This forwarding is the crucial step. Because it is an explicit impl rather than a blanket, the compiler follows it and surfaces every unsatisfied constraint coming from `GetFooValue` — the table re-exposes the provider's real requirements at the point of lookup. The forwarding is also what carries dependencies across layers: when a provider bundle delegates to another bundle, each bundle's `IsProviderFor` impl forwards to the next, so a requirement unmet several tables deep still propagates back to where the component is checked. The generic parameters are literally named `Context` and `Params` in the emitted code. The `Params` slot is filled by whatever a check supplies: a single parameter goes in directly, multiple parameters as a tuple, and a parameterless component uses `()`.

The supertrait link on the provider trait is the piece that ties this together for the consumer side. A provider trait generated from `#[cgp_component(FooGetterAt)]` for a component `CanGetFooAt<I, J>` reads as `pub trait FooGetterAt<Context, I, J>: IsProviderFor<FooGetterAtComponent, Context, (I, J)>`, so any attempt to use the provider trait must first establish `IsProviderFor`. That is why probing `IsProviderFor` is equivalent to probing the provider trait's dependency set, and why the marker can stand in for the provider trait whenever a readable error is wanted.

## Examples

`IsProviderFor` is something you observe rather than invoke. A provider that depends on a field gets a matching marker impl automatically:

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

`#[cgp_impl]` emits both the `Greeter` provider trait impl and an `IsProviderFor<GreeterComponent, Context, ()>` impl for `GreetHello`, each guarded by the same `HasField` bound. When this provider is wired onto a context that lacks a `name` field, the table's forwarding `IsProviderFor` impl carries that unmet `HasField` bound back to the point of use — which is exactly what [`check_components!`](../macros/check_components.md) leverages to report the missing field at the wiring site:

```rust
#[derive(HasField)]
pub struct App {
    pub first_name: String, // not `name`
}

delegate_components! {
    App { GreeterComponent: GreetHello }
}

check_components! {
    App { GreeterComponent }
}
```

The check forces `App: CanUseComponent<GreeterComponent, ()>`, which routes through `GreetHello: IsProviderFor<GreeterComponent, App, ()>`, whose `where` clause names the `HasField` bound — so the compiler reports the absent `name` field rather than a bare "provider trait not implemented".

The marker can also be probed directly, which is what the `#[check_providers(...)]` form of `check_components!` does for a provider that is not the one a context delegates to. Asserting it by hand looks like a where-bound with no body:

```rust
fn assert_provider()
where
    GreetHello: IsProviderFor<GreeterComponent, App, ()>,
{}
```

This holds only if `GreetHello`'s dependencies are met for `App` — the same condition as the provider trait, surfaced along the explicit path. For a component with several type parameters the final argument is the grouping tuple, so a two-parameter component is asserted as `IsProviderFor<FooGetterAtComponent, App, (I, J)>`.

## Related constructs

`IsProviderFor` is the supertrait that [`#[cgp_component]`](../macros/cgp_component.md) attaches to every provider trait, and the empty impl beside it is generated by [`#[cgp_provider]`](../macros/cgp_provider.md) and [`#[cgp_impl]`](../macros/cgp_impl.md). [`delegate_components!`](../macros/delegate_components.md) forwards it through each [`DelegateComponent`](delegate_component.md) entry so a delegated provider's requirements stay visible. [`CanUseComponent`](can_use_component.md) is the context-side counterpart whose blanket impl requires `IsProviderFor` of the delegate, and [`check_components!`](../macros/check_components.md) is the macro that forces both — its `#[check_providers(...)]` form asserts `IsProviderFor` on named providers directly, which is the tool for localizing failures in higher-order provider stacks.

## Source

- The trait is defined in [crates/core/cgp-component/src/traits/is_provider.rs](../../../crates/core/cgp-component/src/traits/is_provider.rs) and re-exported through [crates/core/cgp-component/src/macro_prelude.rs](../../../crates/core/cgp-component/src/macro_prelude.rs).
- The provider-trait supertrait link and the per-impl marker impl are emitted by [crates/macros/cgp-macro-core/src/types/cgp_component/](../../../crates/macros/cgp-macro-core/src/types/cgp_component/) and the `#[cgp_provider]`/`#[cgp_impl]` codegen; the table forwarding impl is built in [crates/macros/cgp-macro-core/src/types/delegate_component/mapping/eval.rs](../../../crates/macros/cgp-macro-core/src/types/delegate_component/mapping/eval.rs).
- For how it is generated and the index of tests, see the implementation document [implementation/entrypoints/cgp_component.md](../../implementation/entrypoints/cgp_component.md).
