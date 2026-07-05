# Dispatching a component per type

A component that is generic over a type parameter often wants a different provider per value of that parameter, and this guide is about doing that with the `open` statement or a namespace rather than the legacy `UseDelegate` nested table.

This guide connects directly to [organizing wiring with namespaces and prefixes](namespaces-and-prefixes.md), which develops the namespace side of the choice in depth.

## Dispatch per type with `open` and namespaces, not `UseDelegate`

Route a generic-parameter component to a different provider per type with the [`open` statement](../reference/macros/delegate_components.md) or a [namespace](namespaces-and-prefixes.md), rather than the legacy [`UseDelegate`](../reference/providers/use_delegate.md) nested-table pattern. Both the `open` statement and namespaces dispatch through the `RedirectLookup` impl that every [`#[cgp_component]`](../reference/macros/cgp_component.md) already generates, so they store the per-type entries directly on the context and need no wrapper type. The legacy form nests a `UseDelegate` table:

```rust
delegate_components! {
    MyApp {
        AreaCalculatorComponent:
            UseDelegate<new AreaCalculatorComponents {
                Rectangle: RectangleArea,
                Circle: CircleArea,
            }>,
    }
}
```

while dispatching inline with `open`:

```rust
delegate_components! {
    MyApp {
        open AreaCalculatorComponent;

        @AreaCalculatorComponent.Rectangle: RectangleArea,
        @AreaCalculatorComponent.Circle: CircleArea,
    }
}
```

Because `open` and namespaces ride `RedirectLookup`, **a new component you intend to dispatch this way does not need the [`#[derive_delegate(UseDelegate<Param>)]`](../reference/attributes/derive_delegate.md) attribute at all** — that attribute exists only to generate the `UseDelegate` provider the legacy nested-table form relies on. You will still see `#[derive_delegate]` on some CGP-shipped components, such as the error and handler families, which carry it so existing `UseDelegate`-based wiring keeps working; but code that dispatches only through `open` or a namespace can omit it.

Choose between `open` and a namespace by scope. Prefer `open` for a self-contained context wiring its own components directly — it folds the per-type entries into the context's own table with no separate type. Reach for a [namespace](namespaces-and-prefixes.md) when a reusable, inheritable dispatch table is worth sharing across contexts, or when a single generic component is served by several providers whose per-type entries you want to merge into one flat table.

## Related guides

- [Organizing wiring with namespaces and prefixes](namespaces-and-prefixes.md) — the full namespace treatment, including flattening multi-provider dispatch that `open` alone cannot.
- [Guides summary](README.md#summary) — the cheat-sheet across all the guides.
