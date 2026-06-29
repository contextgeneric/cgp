# `Runner` (`CanRun` / `CanSendRun`)

The `Runner` family is CGP's pair of task-running components: `CanRun<Code>` runs a named task asynchronously, and `CanSendRun<Code>` is the variant whose returned future is `Send`, for when the work must cross a thread boundary.

## Purpose

The `Runner` family exists to give a context a uniform way to *execute a unit of work* selected at the type level, with the concrete behavior chosen through wiring. The unit of work is identified by a `Code` type parameter — a phantom tag, not a value — so a single context can host many distinct tasks, one per `Code`, and dispatch each to its own provider. "Running" a task here means invoking the provider wired for that `Code` and awaiting an asynchronous `Result<(), Error>`: the task either completes or produces the context's abstract error. The components carry no input or output beyond success-or-error; they model a fire-and-complete action rather than a transformation, which is what separates them from the [handler](handler.md) family that maps an `Input` to an `Output`.

`CanRun<Code>` is the base form, and `CanSendRun<Code>` exists to solve a specific Rust limitation around `Send` futures. When a runner needs to hand its future to a spawner like `tokio::spawn`, that future must be `Send`, but a generic `async fn` over abstract context types cannot promise `Send` without annotating `Send` bounds on every abstract type in scope — bounds that pollute every interface. `CanSendRun<Code>` sidesteps this by returning an explicit `impl Future<Output = ...> + Send`, so the `Send` requirement lives on this one trait. A context implements `CanSendRun<Code>` as a thin proxy over its `CanRun<Code>` implementation, and because the proxy is written against the *concrete* context, the compiler can confirm the concrete future is `Send` without abstract-type annotations. This is a workaround that remains necessary until Rust stabilizes Return Type Notation; once that lands, the `Send`-bound future could be expressed without the separate trait.

The family is the execution layer that ties together the rest of a CGP application: a runner provider typically reaches the context's runtime through [`HasRuntime`](has_runtime.md) to spawn or await work, and dispatches to other components — fetching data, performing effects — to do the actual job. `CanRun` is what application code calls to set everything in motion.

## Definition

Both components are declared in `cgp-run`, each with [`#[cgp_component]`](../macros/cgp_component.md), `#[async_trait]`, and a [`#[derive_delegate(UseDelegate<Code>)]`](../attributes/derive_delegate.md) attribute, and each supertraiting [`HasErrorType`](has_error_type.md) so the task can fail with the context's abstract error:

```rust
#[cgp_component(Runner)]
#[async_trait]
#[derive_delegate(UseDelegate<Code>)]
pub trait CanRun<Code>: HasErrorType {
    async fn run(&self, _code: PhantomData<Code>) -> Result<(), Self::Error>;
}

#[cgp_component(SendRunner)]
#[async_trait]
#[derive_delegate(UseDelegate<Code>)]
pub trait CanSendRun<Code>: HasErrorType {
    fn send_run(
        &self,
        _code: PhantomData<Code>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
```

The `Code` parameter is the type-level name of the task to run; it is passed only as `PhantomData<Code>`, so it carries no data and exists purely to select an implementation. `#[cgp_component(Runner)]` names the provider trait `Runner` and the component marker `RunnerComponent`; `#[cgp_component(SendRunner)]` names them `SendRunner` and `SendRunnerComponent`. The [`#[derive_delegate(UseDelegate<Code>)]`](../attributes/derive_delegate.md) attribute generates a `UseDelegate` provider that dispatches on `Code`, so a context can route different `Code` tags to different runner providers through an inner delegation table.

The two traits differ only in how they shape the asynchronous return. `CanRun::run` is an ordinary `async fn` returning `Result<(), Self::Error>`, with no `Send` requirement on the future. `CanSendRun::send_run` instead returns an explicit `impl Future<Output = Result<(), Self::Error>> + Send`, so callers may move the future across threads. Both produce `()` on success — the task's effect is observable elsewhere, not returned.

## Behavior

Because both are `#[cgp_component]` traits, each generates the standard machinery: a consumer blanket impl forwarding to the provider wired for the component marker, the `Runner` / `SendRunner` provider trait, and the usual `UseContext`, `RedirectLookup`, and — from `#[derive_delegate]` — `UseDelegate<Code>` provider impls. A context calls `context.run(PhantomData::<MyTask>)` and the consumer impl routes to whatever provider it wired for `RunnerComponent`, with `MyTask` as the `Code`.

The `UseDelegate<Code>` dispatch is the idiomatic way a context hosts multiple tasks. Wiring `RunnerComponent` to `UseDelegate<Table>` makes the context look each `Code` up in `Table` and run the provider found there, so distinct tasks get distinct implementations on the same context:

```rust
delegate_components! {
    App {
        RunnerComponent:
            UseDelegate<new AppRunnerComponents {
                ActionA: RunWithFooBar,
                ActionB: SpawnAndRun<ActionA>,
            }>,
    }
}
```

With this wiring, `app.run(PhantomData::<ActionA>)` runs the `RunWithFooBar` provider while `app.run(PhantomData::<ActionB>)` runs `SpawnAndRun<ActionA>`. A runner provider is a normal CGP provider written for the `Runner` provider trait; it receives `&Context` and the `PhantomData<Code>` tag and may freely call other components on the context — fetching values, performing effects, then completing.

The `Send` variant is wired as a proxy on the concrete context. The pattern is to implement `SendRunner<App, Code>` for `App` itself, forwarding `send_run` to the context's own `run`:

```rust
#[cgp_provider]
impl SendRunner<App, ActionA> for App {
    async fn send_run(context: &App, code: PhantomData<ActionA>) -> Result<(), Infallible> {
        context.run(code).await
    }
}
```

Because this impl names the concrete `App` and `ActionA`, the future produced by `context.run(code)` has a fully known type, so the compiler can verify it is `Send` and satisfy the `+ Send` bound on `send_run` — something the generic `CanRun` definition deliberately does not assert. A spawning runner provider can then require `Context: CanSendRun<InCode>`, clone the context into a `Send` future, and hand it to a spawner, all without `Send` bounds leaking into the abstract interfaces.

## Examples

A complete flow defines tasks, wires runners, proxies the `Send` variant, and spawns one task from inside another. A spawning provider requires the context to be `CanSendRun` and hands a `Send` future to a spawner:

```rust
#[cgp_new_provider(RunnerComponent)]
impl<Context, Code, InCode> Runner<Context, Code> for SpawnAndRun<InCode>
where
    Context: 'static + Send + Clone + CanSendRun<InCode>,
{
    async fn run(context: &Context, _code: PhantomData<Code>) -> Result<(), Context::Error> {
        let context = context.clone();

        spawn(async move {
            let _ = context.send_run(PhantomData).await;
        });

        Ok(())
    }
}
```

`SpawnAndRun<InCode>` runs the task `Code` by cloning the context and spawning the *inner* task `InCode` on a background thread. The spawner requires a `Send + 'static` future, which is exactly what `context.send_run(PhantomData::<InCode>)` returns — so the constraint `Context: CanSendRun<InCode>` is what makes the spawn type-check. The context wires `RunnerComponent` to a `UseDelegate` table mapping `ActionB` to `SpawnAndRun<ActionA>`, and supplies a `SendRunner` proxy for `ActionA` as shown above; calling `app.run(PhantomData::<ActionB>)` then spawns `ActionA` to run asynchronously. None of the abstract task or error types carry `Send` bounds — the bound is discharged only at the concrete `SendRunner` proxy.

## Related constructs

The `Runner` family is most often paired with [`HasRuntime`](has_runtime.md): a runner provider reaches the context's runtime through `HasRuntime` to spawn tasks or await timers, and names its runtime type through `HasRuntimeType`. It shares its shape with the [handler](handler.md) family — both are `#[cgp_component]` async traits dispatching on a `Code` tag via [`#[derive_delegate(UseDelegate<Code>)]`](../attributes/derive_delegate.md) — but a runner only runs to a `Result<(), Error>`, whereas a handler transforms an `Input` into an `Output`. Both runner traits supertrait [`HasErrorType`](has_error_type.md) for their failure type, are defined with [`#[cgp_component]`](../macros/cgp_component.md), and are wired on a context with [`delegate_components!`](../macros/delegate_components.md), commonly through a `UseDelegate` table keyed by `Code`.

## Source

`CanRun` / `Runner` and `CanSendRun` / `SendRunner` are defined together in [crates/extra/cgp-run/src/lib.rs](../../../crates/extra/cgp-run/src/lib.rs), reached from the facade as `cgp::extra::run`. The `#[cgp_component]` and `#[derive_delegate]` expansions they rely on live under [crates/macros/cgp-macro-core/src/](../../../crates/macros/cgp-macro-core/src/). The end-to-end usage — a `UseDelegate` runner table, a `SpawnAndRun` provider, and a concrete `SendRunner` proxy that spawns a `Send` future — is exercised in [crates/tests/cgp-tests/src/tests/async/spawn.rs](../../../crates/tests/cgp-tests/src/tests/async/spawn.rs).
