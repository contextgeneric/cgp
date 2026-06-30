# `delegate_and_check_components!`

`delegate_and_check_components!` wires a context's components and asserts the wiring is complete in one step, combining [`delegate_components!`](delegate_components.md) with [`check_components!`](check_components.md).

## Purpose

`delegate_and_check_components!` exists so that a context's wiring is checked the moment it is written, with no separate test block to remember. Because CGP wiring is lazy — a [`delegate_components!`](delegate_components.md) entry is accepted without verifying that the chosen provider can actually satisfy the component — it is easy to leave a context that compiles but fails at the first call to a consumer trait. A standalone [`check_components!`](check_components.md) block closes that gap, but keeping it in sync with the wiring is manual: add a delegation and you must remember to add its check. This macro removes that bookkeeping by deriving the checks directly from the delegations.

The recommendation is to use this macro for a main context's wiring, where catching a broken or incomplete wiring as early as possible is most valuable. Plain `delegate_components!` remains the right tool for intermediary provider tables — bundles that group providers together without being a context in their own right — and for complex cases where the checks need more control than the per-entry derivation gives, in which case a separate `check_components!` block is more flexible.

Functionally, the macro emits exactly what writing both macros by hand would, with the check entries inferred from the delegation keys. Every delegated component is checked unless explicitly opted out, so the default behavior is "wire it and prove it works."

## Syntax

The macro takes the same table shape as [`delegate_components!`](delegate_components.md) — an optional `new` keyword and generics, a target type, and brace-delimited `Key: Value` delegation entries — and additionally accepts a few attributes that control the checking half. The basic form simply wires and checks each entry:

```rust
delegate_and_check_components! {
    ScaledRectangle {
        AreaCalculatorComponent:
            ScaledAreaCalculator<RectangleAreaCalculator>,
    }
}
```

The check trait's name defaults to `__CanUse{Context}` (for example `__CanUseScaledRectangle`). This deliberately differs from the `__Check{Context}` name that [`check_components!`](check_components.md) derives, so that both macros can be used once each in the same module without a name clash. A table-level `#[check_trait(Name)]` attribute overrides the derived name:

```rust
delegate_and_check_components! {
    #[check_trait(TestScaledRectangle)]
    ScaledRectangle {
        AreaCalculatorComponent:
            ScaledAreaCalculator<RectangleAreaCalculator>,
    }
}
```

A component with generic parameters needs a `#[check_params(...)]` attribute on its entry, because the derived check would otherwise have no parameters to test. The delegation half does not need the parameters — the `DelegateComponent` impl is generic over them — but the check half does, so `#[check_params(...)]` supplies them, with the same single-versus-tuple convention as [`check_components!`](check_components.md):

```rust
delegate_and_check_components! {
    MyApp {
        #[check_params(
            Rectangle,
            Circle,
        )]
        AreaOfShapeCalculatorComponent:
            UseDelegate<new AreaOfShapeCalculatorComponents {
                Rectangle: RectangleArea,
                Circle: CircleArea,
            }>,
    }
}
```

A `#[skip_check]` attribute on an entry wires it without generating any check, for cases where that component is verified separately. This avoids having to split out a second plain `delegate_components!` block just to wire one component without a check:

```rust
delegate_and_check_components! {
    ScaledRectangle {
        #[skip_check]
        AreaCalculatorComponent:
            ScaledAreaCalculator<RectangleAreaCalculator>,
    }
}
```

The `#[check_params(...)]` and `#[skip_check]` attributes are mutually exclusive on a given key, and at most one may appear. The same array syntax that `delegate_components!` allows on the key side works here, with check params attaching per bracketed key as needed.

## Syntax Grammar

The body of `delegate_and_check_components!` is the same table shape as [`delegate_components!`](delegate_components.md), with an optional table-level check-trait attribute and a per-entry check attribute added:

```ebnf
DelegateAndCheck -> TableAttr* Generics? `new`? TargetType `{` TableBody `}`

TableAttr        -> `#` `[` `check_trait` `(` IDENTIFIER `)` `]`

TableBody        -> Statement* ( CheckedMapping ( `,` CheckedMapping )* `,`? )?

CheckedMapping   -> EntryAttr? Mapping       // Mapping, Key, ProviderValue — see delegate_components!

EntryAttr        -> `#` `[` `check_params` `(` Type ( `,` Type )* `,`? `)` `]`
                  | `#` `[` `skip_check` `]`
```

The `Mapping`, `Key`, `ProviderValue`, and `Statement` productions are exactly those of [`delegate_components!`](delegate_components.md); only the attributes differ. The table-level `#[check_trait(...)]` overrides the derived `__CanUse{Context}` trait name. Each mapping may carry at most one `EntryAttr`, and `#[check_params(...)]` and `#[skip_check]` are mutually exclusive: `#[check_params(...)]` supplies the generic parameters the derived check needs for a component with type parameters, and `#[skip_check]` wires the entry without generating a check at all.

## Expansion

The macro emits the delegation impls exactly as [`delegate_components!`](delegate_components.md) would, then appends a check trait and one impl per non-skipped entry, exactly as [`check_components!`](check_components.md) would. Starting from:

```rust
delegate_and_check_components! {
    #[check_trait(CheckMyContext)]
    MyContext {
        NameTypeProviderComponent: UseType<String>,
        NameGetterComponent: UseField<Symbol!("name")>,
    }
}
```

the macro first produces the wiring half — a [`DelegateComponent`](../traits/delegate_component.md) impl and an [`IsProviderFor`](../traits/is_provider_for.md) forwarding impl for each entry:

```rust
impl DelegateComponent<NameTypeProviderComponent> for MyContext {
    type Delegate = UseType<String>;
}
impl<__Context__, __Params__>
    IsProviderFor<NameTypeProviderComponent, __Context__, __Params__> for MyContext
where
    UseType<String>: IsProviderFor<NameTypeProviderComponent, __Context__, __Params__>,
{}

impl DelegateComponent<NameGetterComponent> for MyContext {
    type Delegate = UseField<Symbol!("name")>;
}
impl<__Context__, __Params__>
    IsProviderFor<NameGetterComponent, __Context__, __Params__> for MyContext
where
    UseField<Symbol!("name")>: IsProviderFor<NameGetterComponent, __Context__, __Params__>,
{}
```

then the checking half — a check trait aliasing [`CanUseComponent`](../traits/can_use_component.md), with one impl per delegated component:

```rust
trait CheckMyContext<__Component__, __Params__: ?Sized>:
    CanUseComponent<__Component__, __Params__>
{}

impl CheckMyContext<NameTypeProviderComponent, ()> for MyContext {}
impl CheckMyContext<NameGetterComponent, ()> for MyContext {}
```

Without the `#[check_trait(...)]` override, the trait would instead be named `__CanUseMyContext`. The whole output is identical to writing a `delegate_components!` block followed by a `check_components!` block whose check trait carries the `__CanUse{Context}` name.

A `#[check_params(...)]` entry expands its parameters into the `__Params__` slot of the generated check impls, one impl per listed parameter, while the delegation impl stays generic over the parameter. The earlier `MyApp` table therefore checks `MyApp` at `Rectangle` and at `Circle` (`impl __CanUseMyApp<AreaOfShapeCalculatorComponent, Rectangle> for MyApp {}` and likewise for `Circle`), even though its single `DelegateComponent` impl is parameter-generic. A `#[skip_check]` entry contributes its delegation impls but no check impl, so it appears in the wiring half and is absent from the checking half.

A generic table threads its generics through both halves. `<T> MyContext<T> { ... }` yields `impl<T> DelegateComponent<...> for MyContext<T>` delegations alongside `impl<T> __CanUseMyContext<..., ()> for MyContext<T> {}` checks.

## Examples

A main context wired and checked together is the intended use:

```rust
use cgp::prelude::*;

#[derive(HasField)]
pub struct MyContext {
    pub name: String,
}

delegate_and_check_components! {
    MyContext {
        NameTypeProviderComponent: UseType<String>,
        NameGetterComponent: UseField<Symbol!("name")>,
    }
}
```

If `MyContext` were missing the `name` field, the derived check on `NameGetterComponent` would fail to compile and report the missing `HasField` bound, rather than letting the gap slip through to a later use of `name()`.

Mixing checked and skipped entries lets a higher-order delegation be verified elsewhere while the rest is checked inline:

```rust
delegate_and_check_components! {
    ScaledRectangle {
        AreaCalculatorComponent:
            ScaledAreaCalculator<RectangleAreaCalculator>,

        #[skip_check]
        TransformCalculatorComponent:
            ComplexTransform<RectangleAreaCalculator>, // checked in a dedicated check_components! block
    }
}
```

## Related constructs

`delegate_and_check_components!` is the fusion of [`delegate_components!`](delegate_components.md) and [`check_components!`](check_components.md): the wiring half behaves exactly like the former and the checking half like the latter, so the semantics of [`DelegateComponent`](../traits/delegate_component.md), [`IsProviderFor`](../traits/is_provider_for.md), and [`CanUseComponent`](../traits/can_use_component.md) all carry over unchanged. It wires components defined with [`#[cgp_component]`](cgp_component.md) to providers written with [`#[cgp_impl]`](cgp_impl.md), [`#[cgp_provider]`](cgp_provider.md), or [`#[cgp_fn]`](cgp_fn.md), and supports nested-table values via [`use_delegate.md`](../providers/use_delegate.md) and field getters via [`use_field.md`](../providers/use_field.md). Reach for plain [`delegate_components!`](delegate_components.md) instead when building intermediary provider tables, and for a standalone [`check_components!`](check_components.md) block when a check needs `#[check_providers(...)]` or other control beyond per-entry `#[check_params]`/`#[skip_check]`.

## Source

The macro entry point is `delegate_and_check_components` in [crates/macros/cgp-macro-lib/src/delegate_and_check_components.rs](../../../crates/macros/cgp-macro-lib/src/delegate_and_check_components.rs), which parses the table, evaluates the delegation half via the shared `DelegateTable`, derives a `CheckComponentsTable` from the keys, and emits both. The logic lives in [crates/macros/cgp-macro-core/src/types/delegate_and_check_components/](../../../crates/macros/cgp-macro-core/src/types/delegate_and_check_components/): the `__CanUse{Context}` default name and `#[check_trait]` handling in `item.rs`, the `#[check_params]`/`#[skip_check]` parsing and their mutual exclusion in `check_params.rs`, the per-key conversion to check entries in `key_with_check_params.rs`, and the walk over delegation entries in `to_keys_with_check_params.rs`. It reuses the `DelegateTable` from [delegate_component/](../../../crates/macros/cgp-macro-core/src/types/delegate_component/) and the `CheckComponentsTable` from [check_components/](../../../crates/macros/cgp-macro-core/src/types/check_components/). Expansion snapshots are in [crates/tests/cgp-tests/src/tests/delegate_and_check_components.rs](../../../crates/tests/cgp-tests/src/tests/delegate_and_check_components.rs) and [crates/tests/cgp-tests/src/tests/check_components.rs](../../../crates/tests/cgp-tests/src/tests/check_components.rs).
