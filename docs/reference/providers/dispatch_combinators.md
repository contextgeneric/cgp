# Dispatch combinators

The dispatch combinators are the `cgp-dispatch` provider structs that route an extensible-data value — a record or a variant — to per-field or per-variant handlers, and assemble or finalize the result. They are all [`Computer`](../components/computer.md)-family providers (and several are also [`Handler`](../components/handler.md)/`TryComputer` providers) built on top of the extractor and builder trait families.

## Purpose

The dispatch combinators solve the problem of handling an arbitrary record or enum generically, where the set of fields or variants is not known at the call site and the handling logic for each one lives in a separate provider. A hand-written `match` on a concrete enum names every variant in one place and calls a fixed function for each; these combinators instead drive the extractor family from [`extract_field`](../traits/extract_field.md) and the builder family from [`has_builder`](../traits/has_builder.md) to do the same work over a value whose shape is only known at the type level, dispatching each field or variant to a handler chosen by type.

The combinators divide into two halves that mirror the two halves of the extensible-data machinery. The *matcher* half consumes a sum type: it tries each variant in turn, hands the matched payload to a handler, and proves the match exhaustive without a wildcard arm. The *builder* half produces a product type: it starts from an empty builder, runs a handler per field to compute that field's value, and finalizes the fully-populated record. Both halves are expressed as handler providers so they compose with the rest of the handler ecosystem — they can be wired into a context with [`delegate_components!`](../macros/delegate_components.md), nested inside [`UseInputDelegate`](../providers/use_delegate.md), and chained with the combinators in [`handler_combinators`](handler_combinators.md).

## Definition

The combinators are zero-sized provider structs, each generic over a list of handlers or a per-element provider, that implement one or more of the handler component traits. This section groups them by role: the matcher loop they share, the matchers that drive it directly, the field/variant adapters that prepare a value for a handler, the convenience matchers that build a handler list from a context's fields, and the builders that go the other way. Every struct verified here carries `PhantomData` over its type parameters and is a pure type-level entity with no runtime value, in keeping with how CGP providers work.

### `DispatchMatchers` — the matcher loop

`DispatchMatchers<Handlers>` is the engine every matcher delegates to. It is a type alias for a monadic pipeline that runs a list of handlers and stops at the first one that succeeds:

```rust
pub type DispatchMatchers<Providers> = PipeMonadic<OkMonadic, Providers>;
```

The list `Providers` is a `Product![...]` of handlers, each of which takes the partial value (the extractor) and returns `Result<Output, Remainder>`: `Ok(output)` means that handler matched and produced the output, while `Err(remainder)` means it did not match and hands back the extractor with one more variant ruled out. [`PipeMonadic`](monad_providers.md) under the `OkMonadic` monad threads the pipeline along the `Err` branch and short-circuits on `Ok`, so the list runs handler by handler until one returns `Ok`, carrying the shrinking remainder forward through each `Err`. When the last handler still returns `Err`, the remainder type has every variant ruled out and is therefore uninhabited, which is what lets the enclosing matcher discharge it without a fallback arm. `DispatchMatchers` is an implementation detail of the matchers below and is not normally named by users.

### `MatchWithHandlers` and its borrowed forms

`MatchWithHandlers<Handlers>` is the owned-input matcher: given a value, it converts the value to its extractor and runs the handler list over it. Its `Computer`/`AsyncComputer` impls require the input to implement [`HasExtractor`](../traits/extract_field.md), run `DispatchMatchers<Handlers>` over `Input::Extractor` to obtain `Result<Output, Remainder>`, and call `finalize_extract_result` on that result so the uninhabited remainder is discharged and the bare `Output` is returned:

```rust
pub struct MatchWithHandlers<Handlers>(pub PhantomData<Handlers>);

impl<Context, Code, Input, Output, Remainder, Handlers> Computer<Context, Code, Input>
    for MatchWithHandlers<Handlers>
where
    Input: HasExtractor,
    DispatchMatchers<Handlers>:
        Computer<Context, Code, Input::Extractor, Output = Result<Output, Remainder>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;
    // compute: DispatchMatchers::compute(context, code, input.to_extractor())
    //              .finalize_extract_result()
}
```

`MatchWithHandlersRef<Handlers>` and `MatchWithHandlersMut<Handlers>` are the same provider over a borrowed input. They implement the handler traits for `&'a Input` and `&'a mut Input` respectively, require `HasExtractorRef`/`HasExtractorMut`, and run the handler list over `Input::ExtractorRef<'a>`/`Input::ExtractorMut<'a>`, so a value can be matched without being moved. Each of the three structs implements both the synchronous `Computer` and the `AsyncComputer` form.

### `MatchFirstWithHandlers` and its borrowed forms

`MatchFirstWithHandlers<Handlers>` is the matcher for the multi-argument calling convention, where the input is a tuple `(Input, Args)` carrying the value being matched together with extra arguments to pass along to each handler. It threads `Args` through the loop unchanged: the handler list runs over `(Input::Extractor, Args)` and returns `Result<Output, (Remainder, Args)>`, so on a miss both the shrunken remainder and the still-owned arguments are carried to the next handler. When the loop finishes with an `Err`, the remainder is uninhabited and is discharged directly through `finalize_extract`:

```rust
pub struct MatchFirstWithHandlers<Handlers>(pub PhantomData<Handlers>);

impl<Context, Code, Input, Args, Output, Remainder, Handlers>
    Computer<Context, Code, (Input, Args)> for MatchFirstWithHandlers<Handlers>
where
    Input: HasExtractor,
    DispatchMatchers<Handlers>: Computer<
        Context, Code, (Input::Extractor, Args),
        Output = Result<Output, (Remainder, Args)>>,
    Remainder: FinalizeExtract,
{
    type Output = Output;
    // compute: match DispatchMatchers::compute(context, code, (input.to_extractor(), args)) {
    //     Ok(output) => output,
    //     Err((remainder, _)) => remainder.finalize_extract(),
    // }
}
```

The "first" in the name reflects that the value being matched is the *first* element of the input tuple, with the remaining arguments riding alongside. As with the plain matcher, there are borrowed variants `MatchFirstWithHandlersRef<Handlers>` and `MatchFirstWithHandlersMut<Handlers>` over `(&'a Input, Args)` and `(&'a mut Input, Args)`, and each struct implements both `Computer` and `AsyncComputer`.

### The field and variant adapters

The handler list a matcher runs is normally a list of *adapters*, each of which tries one field or variant and forwards the matched payload to an inner provider. Four adapters cover the matching side, and each pairs a value-extraction step with a delegation to a provider that defaults to [`UseContext`](use_context.md).

`ExtractFieldAndHandle<Tag, Provider>` is the variant adapter for the owned/value calling convention. Its `Output` is `Result<Output, Remainder>`: it calls `ExtractField<Tag>` on the input, and on success wraps the payload in a `Field<Tag, Value>` and hands it to `Provider`, returning `Ok` of the provider's output; on failure it returns `Err` of the remainder, the extractor with that variant ruled out. `ExtractFirstFieldAndHandle<Tag, Provider>` is the same adapter for the `(Input, Args)` convention, threading the arguments into the provider call as `(Field<Tag, Value>, Args)` and returning `Result<Output, (Remainder, Args)>`:

```rust
pub struct ExtractFieldAndHandle<Tag, Provider = UseContext>(pub PhantomData<(Tag, Provider)>);
pub struct ExtractFirstFieldAndHandle<Tag, Provider = UseContext>(pub PhantomData<(Tag, Provider)>);
```

`HandleFieldValue<Provider>` and `HandleFirstFieldValue<Provider>` are the unwrapping adapters that sit between an extract adapter and the actual work. An extract adapter delivers a `Field<Tag, Value>` so the variant name is still attached to the payload; `HandleFieldValue` strips the `Field` wrapper and passes the bare `Value` to `Provider`, and `HandleFirstFieldValue` does the same for the `(Field<Tag, Input>, Args)` tuple, forwarding `(Input, Args)`:

```rust
pub struct HandleFieldValue<Provider = UseContext>(pub PhantomData<Provider>);
pub struct HandleFirstFieldValue<Provider = UseContext>(pub PhantomData<Provider>);
```

`DowncastAndHandle<Inner, Provider>` is the adapter that matches a *group* of variants at once rather than a single one. Instead of `ExtractField`, it uses `CanDowncastFields<Inner>` (see [`cast`](../traits/cast.md)) to try to narrow the input to a smaller enum type `Inner`; on success it hands the whole `Inner` value to `Provider` and returns `Ok`, and on failure it returns `Err` of the remainder. This lets a matcher delegate several variants to one sub-matcher in a single step:

```rust
pub struct DowncastAndHandle<Input, Provider = UseContext>(pub PhantomData<(Input, Provider)>);
```

Each of these adapters implements both `Computer` and `AsyncComputer`. Because every one returns the `Result<Output, Remainder>` (or `Result<Output, (Remainder, Args)>`) shape that `DispatchMatchers` expects, a `Product!` of them is exactly the handler list a matcher consumes.

### `MatchWithValueHandlers` and `MatchWithFieldHandlers`

Writing out the full handler list for every variant is mechanical, so the convenience matchers build the list automatically from the input type's own field list. `MatchWithFieldHandlers<Provider>` and `MatchWithValueHandlers<Provider>` are type aliases over [`UseInputDelegate`](use_delegate.md) that dispatch on the input type and, for each input, synthesize the per-variant handler list from that input's [`HasFields`](../traits/has_fields.md):

```rust
pub type MatchWithFieldHandlers<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputs<Provider>>;

pub type MatchWithValueHandlers<Provider = UseContext> =
    UseInputDelegate<MatchWithFieldHandlersInputs<HandleFieldValue<Provider>>>;
```

The difference between the two is exactly one `HandleFieldValue` wrapper. `MatchWithFieldHandlers` builds a list of `ExtractFieldAndHandle<Tag, Provider>` adapters, so `Provider` receives each matched payload as a `Field<Tag, Value>` with the variant name still attached. `MatchWithValueHandlers` wraps `Provider` in `HandleFieldValue` first, so `Provider` receives the bare `Value` — this is the form to use when the per-variant handler is an ordinary computer over the payload type, such as one generated by [`#[cgp_computer]`](../macros/cgp_computer.md). The list itself is assembled by the `HasFieldHandlers`/`ToFieldHandlers` machinery described below.

The borrowed counterparts are `MatchWithFieldHandlersRef`/`MatchWithValueHandlersRef` (over `&Input`) and `MatchWithValueHandlersMut` (over `&mut Input`). These additionally wire the `…RefComponent` handler traits through [`PromoteRef`](handler_combinators.md), so a single struct serves both the by-value-of-reference and the by-reference handler interfaces. The first-argument convenience matchers — `MatchFirstWithFieldHandlers`, `MatchFirstWithValueHandlers`, and their `Ref`/`Mut` variants — are the same aliases built on `ExtractFirstFieldAndHandle`, `HandleFirstFieldValue`, and the `MatchFirstWith…` matchers, for the `(Input, Args)` calling convention.

### `ToFieldHandlers`, `HasFieldHandlers`, and `MapFieldHandler`

The convenience matchers turn a context's field list into a handler list through three cooperating traits. `MapFieldHandler` is a type-level function from a field's `Tag` to the adapter that should handle it; `ToFieldHandlers` walks the [`Either`](../types/either.md)-spine of a field list and applies that function to each field, producing a `Cons`-list of adapters; and `HasFieldHandlers` is the convenience entry point that reads a context's `HasFields::Fields` and runs `ToFieldHandlers` over it:

```rust
pub trait MapFieldHandler {
    type FieldHandler<Tag>;
}

pub trait ToFieldHandlers<M> {
    type Handlers;
}

pub trait HasFieldHandlers<M> {
    type Handlers;
}

impl<Context, Fields, M> HasFieldHandlers<M> for Context
where
    Context: HasFields<Fields = Fields>,
    Fields: ToFieldHandlers<M>,
{
    type Handlers = Fields::Handlers;
}
```

`ToFieldHandlers` is implemented inductively over the sum spine: for `Either<Field<Tag, Value>, RestFields>` it produces `Cons<M::FieldHandler<Tag>, RestFields::Handlers>`, and for the terminating [`Void`](../types/either.md) it produces `Nil`. The two `MapFieldHandler` markers supplied by the crate are `MapExtractFieldAndHandle<Provider>`, whose `FieldHandler<Tag>` is `ExtractFieldAndHandle<Tag, Provider>`, and `MapExtractFirstFieldAndHandle<Provider>`, whose `FieldHandler<Tag>` is `ExtractFirstFieldAndHandle<Tag, Provider>`. Composing these is how `MatchWithValueHandlers` ends up running an `ExtractFieldAndHandle<Tag, HandleFieldValue<Provider>>` for each variant of the input enum without the user spelling out the list.

### `BuildAndSetField` and `BuildAndMerge`

The builder side runs in the opposite direction: rather than taking a value apart, it assembles a record field by field, with one handler computing each field's value. `BuildAndSetField<Tag, Provider>` is the single-field builder adapter. It takes a builder (a partial record from [`has_builder`](../traits/has_builder.md)), runs `Provider` over a *reference* to that builder to compute the value for `Tag`, then calls `BuildField<Tag>` to set that field and returns the advanced builder. Because the provider sees `&Builder`, it can read fields already set on the partial record while computing the next one:

```rust
pub struct BuildAndSetField<Tag, Provider = UseContext>(pub PhantomData<(Tag, Provider)>);

impl<Context, Code, Tag, Value, Provider, Output, Builder> Computer<Context, Code, Builder>
    for BuildAndSetField<Tag, Provider>
where
    Provider: for<'a> Computer<Context, Code, &'a Builder, Output = Value>,
    Builder: BuildField<Tag, Value = Value, Output = Output>,
{
    type Output = Output;
    // compute: let value = Provider::compute(context, code, &builder);
    //          builder.build_field(PhantomData::<Tag>, value)
}
```

`BuildAndMerge<Provider>` is the bulk counterpart. Instead of setting one field, it runs `Provider` over a reference to the builder to produce another record's worth of fields, then uses `CanBuildFrom` (see [`has_builder`](../traits/has_builder.md)) to copy every shared field from that result into the builder in one step — the field-list analogue of `BuildAndSetField`:

```rust
pub struct BuildAndMerge<Provider = UseContext>(pub PhantomData<Provider>);

impl<Context, Code, Builder, Provider, Output, Res> Computer<Context, Code, Builder>
    for BuildAndMerge<Provider>
where
    Provider: for<'a> Computer<Context, Code, &'a Builder, Output = Res>,
    Builder: CanBuildFrom<Res, Output = Output>,
{
    type Output = Output;
    // compute: let output = Provider::compute(context, code, &builder);
    //          builder.build_from(output)
}
```

Both `BuildAndSetField` and `BuildAndMerge` implement `Computer`, `TryComputer`, and `Handler`; the `TryComputer` and `Handler` forms additionally require `Context: HasErrorType` and propagate the inner provider's error.

### `BuildWithHandlers` and `BuildAndMergeOutputs`

`BuildWithHandlers<Output, Handlers>` is the entry point that turns a list of builder adapters into a complete record. It starts from `Output::builder()` (an empty partial record, via `HasBuilder`), pipes that builder through the handler list with [`PipeHandlers`](handler_combinators.md) so each adapter sets its field, and calls `finalize_build` on the fully-populated result to recover the concrete `Output`:

```rust
pub struct BuildWithHandlers<Output, Handlers>(pub PhantomData<(Output, Handlers)>);

impl<Context, Code, Input, Output, Builder, Handlers, Res> Computer<Context, Code, Input>
    for BuildWithHandlers<Output, Handlers>
where
    Output: HasBuilder<Builder = Builder>,
    PipeHandlers<Handlers>: Computer<Context, Code, Builder, Output = Res>,
    Res: FinalizeBuild<Target = Output>,
{
    type Output = Output;
    // compute: PipeHandlers::compute(context, code, Output::builder()).finalize_build()
}
```

The original `Input` is discarded — `BuildWithHandlers` produces its output from the builder, not from the input. It implements `Computer`, `TryComputer`, and `Handler`, with the latter two requiring `Context: HasErrorType`. Because `finalize_build` is in scope only for the all-present builder configuration, omitting a handler for some field is a compile error rather than a runtime failure.

`BuildAndMergeOutputs<Output, Handlers>` is a higher-level wrapper used when the handler list is itself a list of plain field-producing providers rather than builder adapters. It is a `delegate_components!` table that maps the whole handler family (`ComputerComponent`, `TryComputerComponent`, `HandlerComponent`, and their `Ref` forms) to `BuildWithHandlers<Output, Handlers::Mapped>`, where each provider in `Handlers` has first been wrapped in `BuildAndMerge` by mapping the list through the `ToBuildAndMergeHandler` [`MapType`](../traits/map_type.md) marker. In effect it lets a caller supply a list of result-producing providers and have each one merged into the builder automatically.

## Examples

A matcher is normally driven by spelling out one extract adapter per variant and wrapping each payload handler in `HandleFieldValue`. The following dispatches a `Shape` enum to a per-variant area computer:

```rust
use cgp::extra::dispatch::{
    ExtractFieldAndHandle, HandleFieldValue, MatchWithHandlers,
};

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

// ComputeArea is a #[cgp_computer] over the payload types Circle and Rectangle.
let circle = Shape::Circle(Circle { radius: 5.0 });

let _area = MatchWithHandlers::<
    Product![
        ExtractFieldAndHandle<Symbol!("Circle"), HandleFieldValue<ComputeArea>>,
        ExtractFieldAndHandle<Symbol!("Rectangle"), HandleFieldValue<ComputeArea>>,
    ],
>::compute(&(), PhantomData::<()>, circle);
```

The list tries `Circle` first; if the runtime value is a circle, `ComputeArea` runs on the `Circle` payload and the loop stops. Otherwise the remainder carries `Circle` ruled out into the `Rectangle` adapter, which is the last arm, so its failure would leave an uninhabited remainder that `finalize_extract_result` discharges.

The same dispatch is far shorter through `MatchWithValueHandlers`, which builds that list from the enum's own fields. Wiring it into a context's `Computer` component with [`UseInputDelegate`](use_delegate.md) lets the matcher be selected when the input is a `Shape`:

```rust
delegate_components! {
    App {
        ComputerComponent: UseInputDelegate<new AreaComputers {
            [Circle, Rectangle, Triangle]: ComputeArea,
            [Shape, ShapePlus]: MatchWithValueHandlers,
        }>,
    }
}
```

Here a `Circle` input is handled directly by `ComputeArea`, while a `Shape` input is handled by `MatchWithValueHandlers`, which synthesizes `ExtractFieldAndHandle<Tag, HandleFieldValue<UseContext>>` for each variant and routes each payload back through the context's own `ComputerComponent` — so `Circle` and `Rectangle` payloads reach `ComputeArea` after all.

The builder side mirrors this. The following assembles a `FooBarBaz` by merging a built `FooBar` and computing the remaining `baz` field:

```rust
use cgp::extra::dispatch::{BuildAndMerge, BuildAndSetField, BuildWithHandlers};

type Handlers = Product![
    BuildAndMerge<BuildFooBar>,
    BuildAndSetField<Symbol!("baz"), BuildBaz>,
];

let foo_bar_baz = BuildWithHandlers::<FooBarBaz, Handlers>::compute(&context, code, ());
```

`BuildWithHandlers` starts from `FooBarBaz::builder()`, runs `BuildAndMerge<BuildFooBar>` to copy the `foo` and `bar` fields from a built `FooBar`, runs `BuildAndSetField<Symbol!("baz"), BuildBaz>` to compute and set `baz`, and finalizes. Dropping either handler leaves a field unset and fails to compile at `finalize_build`.

## Related constructs

The matchers stand on the enum-deconstruction traits in [`extract_field`](../traits/extract_field.md) (`HasExtractor`, `ExtractField`, `FinalizeExtract`) and obtain the variant list from [`has_fields`](../traits/has_fields.md); the builders stand on the record-assembly traits in [`has_builder`](../traits/has_builder.md) (`HasBuilder`, `BuildField`, `FinalizeBuild`, `CanBuildFrom`). The matcher loop is [`PipeMonadic`](monad_providers.md) under the `OkMonadic` monad, and the builder pipeline is [`PipeHandlers`](handler_combinators.md); both produce providers in the [`Computer`](../components/computer.md)/[`Handler`](../components/handler.md) families. The convenience matchers dispatch through [`UseInputDelegate`](use_delegate.md) and reuse the borrowed-input promotion of [`PromoteRef`](handler_combinators.md), and their per-element provider defaults to [`UseContext`](use_context.md). The grouped-variant adapter `DowncastAndHandle` relies on [`cast`](../traits/cast.md). The high-level overview that ties all of these together is [`dispatching`](../../concepts/dispatching.md), and the attribute macro that generates a matcher-backed trait impl automatically is [`#[cgp_auto_dispatch]`](../macros/cgp_auto_dispatch.md). The data-type patterns these combinators serve are [extensible records](../../concepts/extensible-records.md) (the builders) and [extensible variants](../../concepts/extensible-variants.md) (the matchers); `BuildAndMergeOutputs` drives the [application builder](../../examples/application-builder.md) example, while `MatchWithValueHandlers` and the explicit `MatchWithHandlers` form drive the [extensible shapes](../../examples/extensible-shapes.md) example over a non-recursive enum and the [expression interpreter](../../examples/expression-interpreter.md) example over a recursive one.

## Source

The provider structs live under [crates/extra/cgp-dispatch/src/providers/](../../../crates/extra/cgp-dispatch/src/providers/): the matchers in `with_handlers/` (`match_with_handlers.rs`, `match_with_handlers_ref.rs`, `match_with_handlers_mut.rs`, `match_first_with_handlers*.rs`, `build_with_handlers.rs`), the matcher loop alias in `dispatchers/dispatch_matchers.rs`, the field/variant adapters in `field_matchers/` (`extract_field.rs`, `extract_first_field.rs`, `extract_handle.rs`, `field_value.rs`, `first_field_value.rs`), the convenience matchers and the `ToFieldHandlers`/`HasFieldHandlers`/`MapFieldHandler` machinery in `matchers/` (`match_with_field_handlers.rs`, `match_first_with_field_handlers.rs`, `to_field_handlers.rs`), and the builders in `field_builders/` (`build_and_set_field.rs`, `build_and_merge.rs`) and `builders/build_and_merge_outputs.rs`. The prelude re-exports the value-handler matchers from [crates/main/cgp-extra/src/prelude.rs](../../../crates/main/cgp-extra/src/prelude.rs); the remaining structs are reached through `cgp::extra::dispatch`. Tests exercising them are in [crates/tests/cgp-tests/tests/extensible_variants/](../../../crates/tests/cgp-tests/tests/extensible_variants/) (matchers) and [extensible_records/](../../../crates/tests/cgp-tests/tests/extensible_records/) (builders).
