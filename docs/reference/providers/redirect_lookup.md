# `RedirectLookup<Key, Components>`

`RedirectLookup<Key, Components>` is a zero-sized provider that implements a component's provider trait by looking up a type-level path in a separate table, re-routing the lookup along that path instead of resolving the component against the context directly.

## Purpose

`RedirectLookup` decouples *which key* a component is looked up under from *which table* answers it. The ordinary provider blanket impl looks a component up in the context's own [delegation table](../traits/delegate_component.md), keyed by the component-name struct. `RedirectLookup<Components, Path>` does the lookup differently: it consults the table `Components` keyed by an arbitrary type-level `Path`, then delegates to whatever provider that entry holds. This indirection is what lets one component's resolution be redirected to a different key in a different table — the basis for organizing wiring into namespaces and presets.

The redirection makes namespaces possible. A namespace groups a context's components under a path prefix so that several related components can be wired in one place and addressed by a shared path. `RedirectLookup` is the provider that turns a prefixed path back into a concrete provider: the namespace machinery sets a component's delegate to a `RedirectLookup` carrying the path under which the real provider was registered, so a lookup of the component follows that path into the table and lands on the intended provider. Presets are built the same way — a preset is a reusable table whose entries are reached through redirected paths.

`RedirectLookup` is not written by hand; it is emitted by macros. Every `#[cgp_component]` generates a `RedirectLookup` impl for its provider trait, and the namespace attributes generate `DelegateComponent` entries whose delegate is a `RedirectLookup`. Reading those generated entries is where this provider appears.

Like every CGP provider, `RedirectLookup` carries no runtime value. Both type parameters are held in `PhantomData`, and the struct exists only as a type-level marker describing a lookup to perform.

## Definition

`RedirectLookup` is a struct parameterized by a key and a table, defined in `cgp-component`:

```rust
pub struct RedirectLookup<Key, Components>(pub PhantomData<(Key, Components)>);
```

The `Key` parameter is the type-level path to look up — typically a [`PathCons`](../types/path_cons.md) chain of [`Symbol!`](../macros/symbol.md) segments ending in a component-name struct. The `Components` parameter is the table to look it up in, a type implementing [`DelegateComponent`](../traits/delegate_component.md). In the generated impls the two appear in the order `RedirectLookup<Components, Path>`, with the table first and the path second. The `PhantomData` makes both parameters part of a valueless struct.

## Behavior

`#[cgp_component]` generates a `RedirectLookup` impl of the provider trait alongside the consumer blanket impl, the provider blanket impl, the component-name struct, and the [`UseContext`](use_context.md) impl. The generated impl looks the path up in the table and forwards to the resulting delegate. For a component such as

```rust
#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}
```

the macro generates this impl (shown with the macro's real placeholder identifiers):

```rust
impl<__Context__, __Components__, __Path__> Greeter<__Context__>
    for RedirectLookup<__Components__, __Path__>
where
    __Components__: DelegateComponent<__Path__>,
    <__Components__ as DelegateComponent<__Path__>>::Delegate: Greeter<__Context__>,
{
    fn greet(__context__: &__Context__) -> String {
        <__Components__ as DelegateComponent<__Path__>>::Delegate::greet(__context__)
    }
}
```

The mechanism is one `DelegateComponent` lookup keyed on `__Path__` rather than on the component name. `RedirectLookup<Components, Path>` implements `Greeter` whenever `Components` maps `Path` to a delegate that itself implements `Greeter`, and the method forwards to that delegate. When the consumer trait carries generic type parameters, the impl additionally constrains `Path` with [`ConcatPath`](../traits/static_format.md) so the parameters are appended to the path before the lookup, letting the redirected key encode the generic arguments. As always, the impl is paired with a matching `IsProviderFor` impl so dependencies reach the [check traits](../../concepts/check-traits.md).

The namespace attributes are what populate the path side. The `#[prefix(@path in Namespace)]` attribute on a component generates a namespace impl whose `Delegate` is `RedirectLookup<Components, Path>`, with the prefix path joined onto the component name — so resolving the component under that namespace follows the prefixed path into the table. The `DefaultNamespace` trait plays the same role for the default routing. Together these turn a path-addressed wiring entry into a concrete provider through `RedirectLookup`.

## Examples

`RedirectLookup` appears in the delegate that the namespace machinery generates, where a component is registered under a path and reached through that path. Wiring a component under a path prefix produces a `RedirectLookup` entry:

```rust
use cgp::prelude::*;

pub struct App;

delegate_components! {
    App {
        namespace DefaultNamespace;

        @bar.baz: TestProvider,
    }
}
```

This registers `TestProvider` under the path `bar`/`baz` in `App`'s default namespace. When a component is later resolved against `App` through that namespace, its delegate is a `RedirectLookup<App, Path>` whose `Path` is the `PathCons` chain `bar` then `baz` then the component name. The lookup follows that path into `App`'s table — matching the entry registered above — and dispatches to `TestProvider`. The component name never keys the context directly; it is the tail of a path that `RedirectLookup` walks. This is the indirection that lets namespaces and presets organize wiring by path while still resolving to ordinary providers.

## Related constructs

`RedirectLookup` is generated by [`#[cgp_component]`](../macros/cgp_component.md) for every component, and is central to the namespace and preset machinery driven by [`#[cgp_namespace]`](../macros/cgp_namespace.md) and explained in [namespaces](../../concepts/namespaces.md). Its lookup is a [`DelegateComponent`](../traits/delegate_component.md) read keyed on a type-level path built from [`PathCons`](../types/path_cons.md) and [`Symbol!`](../macros/symbol.md), with generic parameters folded in through [`ConcatPath`](../traits/static_format.md). It sits beside the other `#[cgp_component]`-generated provider [`UseContext`](use_context.md), which routes back to the context rather than through a separate table, and its dependency propagation flows through [`IsProviderFor`](../traits/is_provider_for.md) for the [check traits](../../concepts/check-traits.md).

## Source

The struct is defined in [crates/core/cgp-component/src/providers/redirect_lookup.rs](../../../crates/core/cgp-component/src/providers/redirect_lookup.rs), and the related `DefaultNamespace` trait in [crates/core/cgp-component/src/namespaces.rs](../../../crates/core/cgp-component/src/namespaces.rs). The `RedirectLookup` provider impl is generated by `to_redirect_lookup_impl` in [crates/macros/cgp-macro-core/src/types/cgp_component/evaluated/to_redirect_lookup_impl.rs](../../../crates/macros/cgp-macro-core/src/types/cgp_component/evaluated/to_redirect_lookup_impl.rs), which appends generic parameters through `ConcatPath`. The namespace delegates that target `RedirectLookup` are produced by the `#[prefix]` attribute in [crates/macros/cgp-macro-core/src/types/attributes/prefix.rs](../../../crates/macros/cgp-macro-core/src/types/attributes/prefix.rs) and the redirect mapping in [crates/macros/cgp-macro-core/src/types/delegate_component/mapping/redirect.rs](../../../crates/macros/cgp-macro-core/src/types/delegate_component/mapping/redirect.rs). Expansion snapshots showing the generated impl and the namespace wiring are in [crates/tests/cgp-tests/tests/namespaces/](../../../crates/tests/cgp-tests/tests/namespaces/).
