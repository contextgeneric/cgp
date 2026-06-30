# `#[cgp_provider]`

`#[cgp_provider]` is applied to a provider-trait implementation written directly on a named provider struct, and it auto-generates the matching [`IsProviderFor`](../traits/is_provider_for.md) impl from that implementation's `where` clause.

## Purpose

`#[cgp_provider]` exists to remove the one piece of boilerplate that every hand-written provider impl would otherwise have to repeat: the `IsProviderFor` marker impl. CGP requires that, alongside a provider's implementation of a provider trait, the provider also implement `IsProviderFor<Component, Context, Params>` under exactly the same constraints. This marker is what lets the compiler produce a readable error — naming the missing dependency — when a context's wiring is incomplete, instead of a terse "trait not implemented" message. Writing it by hand means duplicating the impl's generic parameters and entire `where` clause, and keeping the two copies in sync forever.

`#[cgp_provider]` writes that second impl for you. You write only the real provider impl — `impl AreaCalculator<Context> for RectangleArea where ...` — and the macro emits a copy of it with the body stripped, the trait swapped to `IsProviderFor`, and the same `where` clause preserved. The dependencies are captured automatically and can never drift out of sync, because they are derived from the impl rather than restated.

This is the form to reach for when you are working in the provider trait's native vocabulary — implementing the provider trait directly, with an explicit `Context` type parameter and a static-method signature that takes `context: &Context` rather than `&self`. It is the lower-level counterpart to [`#[cgp_impl]`](cgp_impl.md), which presents the same implementation in consumer-trait clothing and then desugars down to `#[cgp_provider]`.

## Syntax

`#[cgp_provider]` is applied to an `impl` block that implements a provider trait for a provider struct, and its attribute argument is an optional component type:

```rust
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

The impl header is a normal provider-trait impl. The provider trait carries an explicit leading `Context` type parameter, the `Self` type is the provider struct (`RectangleArea`), and methods take the context as an ordinary parameter rather than as a `self` receiver. The provider struct must already exist; `#[cgp_provider]` does not define it. Use [`#[cgp_new_provider]`](cgp_new_provider.md) when you want the struct declared for you.

The attribute takes one optional argument, the **component type** used in the generated `IsProviderFor` impl. When omitted, the component defaults to the provider trait's name with a `Component` suffix, so implementing `AreaCalculator` targets `AreaCalculatorComponent`. Pass the component explicitly when the provider trait's name does not follow that convention or when a provider implements a trait under a differently named component:

```rust
#[cgp_provider(RunnerComponent)]
impl<Context, Code> Runner<Context, Code> for RunWithFooBar
where
    Context: CanFetchFoo + CanFetchBar + CanRunFooBar,
{
    fn run(context: &Context, _code: PhantomData<Code>) -> Result<(), Context::Error> {
        /* ... */
    }
}
```

## Syntax Grammar

The attribute argument of `#[cgp_provider]` is a single optional component type:

```ebnf
CgpProviderArgs -> ComponentType?

ComponentType   -> Type
```

When the argument is omitted the component defaults to the provider trait's name with a `Component` suffix; when present, that `Type` is substituted into the first position of the generated `IsProviderFor` impl. `Type` is the Rust type production.

## Expansion

`#[cgp_provider]` emits two items: the provider impl, passed through unchanged, and an `IsProviderFor` impl derived from it. Starting from:

```rust
#[cgp_provider]
impl<Context, Code, Input> ComputerRef<Context, Code, Input> for FirstNameToString
where
    Context: HasField<Symbol!("first_name"), Value: Display>,
{
    type Output = String;

    fn compute_ref(context: &Context, _code: PhantomData<Code>, _input: &Input) -> String {
        context.get_field(PhantomData).to_string()
    }
}
```

the macro produces:

```rust
impl<Context, Code, Input> ComputerRef<Context, Code, Input> for FirstNameToString
where
    Context: HasField<Symbol!("first_name"), Value: Display>,
{
    type Output = String;

    fn compute_ref(context: &Context, _code: PhantomData<Code>, _input: &Input) -> String {
        context.get_field(PhantomData).to_string()
    }
}

impl<Context, Code, Input> IsProviderFor<ComputerRefComponent, Context, (Code, Input)>
    for FirstNameToString
where
    Context: HasField<Symbol!("first_name"), Value: Display>,
{}
```

The derived impl is the original impl with its body and associated types removed and its trait replaced. It keeps the same generic parameters (`Context, Code, Input`) and the same `where` clause, so it holds under precisely the conditions that the provider impl holds. Its trait arguments are assembled from the provider trait's arguments: the first is the **component type** (`ComputerRefComponent`, the default derived from the `ComputerRef` trait name); the second is the **context type**, taken from the provider trait's leading type argument (`Context`); and the third is the **`Params` tuple** holding every remaining provider-trait type parameter — here `(Code, Input)`. For a provider trait with no extra parameters beyond the context, the `Params` tuple is the empty `()`.

The component-type argument is the only thing the attribute argument changes. Passing `#[cgp_provider(RunnerComponent)]` substitutes that type into the first position of the `IsProviderFor` impl in place of the default `{Trait}Component`; everything else about the expansion is unchanged.

## Examples

A self-contained provider for the `AreaCalculator` component, with its struct declared separately and its `IsProviderFor` impl generated, shows the construct in context:

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

The macro expands this into the provider impl above plus `impl<Context> IsProviderFor<AreaCalculatorComponent, Context> for RectangleArea where Context: HasDimensions {}`. A concrete context wires the component to `RectangleArea` exactly as it would for any provider, through [`delegate_components!`](delegate_components.md), and the `IsProviderFor` impl ensures that a context missing the `HasDimensions` dependency produces an error naming that dependency rather than an opaque one.

In most code, the same provider would be written more concisely with [`#[cgp_impl]`](cgp_impl.md), which lets the body use `self`/`Self` and omit the explicit `Context` parameter. `#[cgp_provider]` is the right choice when you prefer to work directly in the provider trait's own form, or when reading code that another tool or macro has already lowered to that form.

## Related constructs

`#[cgp_provider]` implements a provider trait generated by [`#[cgp_component]`](cgp_component.md). It is the lower-level form that [`#[cgp_impl]`](cgp_impl.md) desugars to; prefer `#[cgp_impl]` for new code and reach for `#[cgp_provider]` when working in the native provider-trait shape. [`#[cgp_new_provider]`](cgp_new_provider.md) behaves identically but also declares the provider struct. The generated [`IsProviderFor`](../traits/is_provider_for.md) impl is the same marker that [`check_components!`](check_components.md) relies on to verify wiring, and a provider is connected to a context through [`delegate_components!`](delegate_components.md).

## Source

The macro entry point is `cgp_provider` in [crates/macros/cgp-macro-lib/src/cgp_provider.rs](../../../crates/macros/cgp-macro-lib/src/cgp_provider.rs), which parses the optional component argument, lowers the impl, and emits the result. The logic lives in [crates/macros/cgp-macro-core/src/types/cgp_provider/](../../../crates/macros/cgp-macro-core/src/types/cgp_provider/): attribute argument parsing (the optional `new` keyword and component type) in `args.rs`; the lowering that derives the component default, the provider struct, and the `IsProviderFor` impl in `item.rs`; the emitted-token assembly in `lower.rs`; and the splitting of provider-trait arguments into context and `Params` tuple in `provider_impl_args.rs`. The `IsProviderFor` derivation itself is in [crates/macros/cgp-macro-core/src/types/provider_impl.rs](../../../crates/macros/cgp-macro-core/src/types/provider_impl.rs). Expansion snapshots are in [crates/tests/cgp-tests](../../../crates/tests/cgp-tests), notably `src/tests/compose.rs` and `src/tests/async/spawn.rs`.
