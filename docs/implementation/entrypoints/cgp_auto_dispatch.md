# `#[cgp_auto_dispatch]` — implementation

`#[cgp_auto_dispatch]` takes a trait with one impl per payload type and appends the code that makes the trait work on an enum of those payloads: a blanket impl of the trait for a fresh enum parameter that runs a value-handler matcher, plus one per-variant [`Computer`](../../reference/components/computer.md) per method. This document covers how the macro is built; for the accepted syntax and the full expansion, read the reference document [reference/macros/cgp_auto_dispatch.md](../../reference/macros/cgp_auto_dispatch.md).

## Entry point

The macro is the `cgp_auto_dispatch` function in [cgp-extra-macro-lib/src/entrypoints/cgp_auto_dispatch.rs](../../../crates/macros/cgp-extra-macro-lib/src/entrypoints/cgp_auto_dispatch.rs). It is a self-contained procedural function operating directly on `syn` types rather than a driver over a `cgp-macro-core` AST stack. It parses the annotated item as a `syn::ItemTrait`, keeps the original trait tokens verbatim, and appends generated code to them. The attribute takes no arguments.

The macro is additive: the trait itself is unchanged, so the per-payload impls the user writes still satisfy it directly. Everything the macro produces is emitted *after* the trait — the enum-level blanket impl first, then one per-variant computer per method.

## Pipeline

There is no staged AST pipeline; the entry function walks the trait's methods twice. The generated code is built by two internal helpers, each of which is worth naming because it emits one of the two kinds of output:

- **`derive_blanket_impl`** builds the single `impl <Trait> for __Variants__` that, per method, invokes a value-handler matcher over the enum. It threads a `where` clause that bounds each matcher and requires `__Variants__: HasExtractor`.
- **`derive_method_computer`** builds, per method, a free function annotated with [`#[cgp_computer]`](cgp_computer.md) whose body calls the trait method on the payload. The macro emits these by delegating to `#[cgp_computer]` rather than synthesizing the `Computer` impl itself.

Both helpers reject a trait item that is not a method, and reject a method that has no `self` receiver or carries a non-lifetime generic parameter.

## Generated items

For each method the macro emits a per-variant computer named `Compute` followed by the method name in PascalCase (`area` yields `ComputeArea`). Its body is just the trait-method call on the payload, and it is bound so it applies to every payload type implementing the trait; a `&self` method borrows the payload through a fresh lifetime `'__a__`:

```rust
// from `fn area(&self) -> f64;`
#[cgp_computer(ComputeArea)]
fn area<'__a__, __Variants__: HasArea>(__Variants__: &'__a__ __Variants__) -> f64 {
    __Variants__.area()
}
```

The enum-level blanket impl implements the trait for a fresh parameter `__Variants__` and, in each method body, dispatches through the matcher struct selected for that method's shape, invoking it with a unit context `&()` and unit code `PhantomData::<()>`:

```rust
impl<__Variants__> HasArea for __Variants__
where
    MatchWithValueHandlersRef<ComputeArea>:
        for<'__a__> Computer<(), (), &'__a__ __Variants__, Output = f64>,
    __Variants__: HasExtractor,
{
    fn area(&self) -> f64 {
        MatchWithValueHandlersRef::<ComputeArea>::compute(&(), PhantomData::<()>, self)
    }
}
```

The matcher is chosen from the value-handler family by two properties of the method: the receiver form and whether the method takes extra arguments. With no extra arguments the plain family is used — `MatchWithValueHandlersRef` for `&self`, `MatchWithValueHandlersMut` for `&mut self`, `MatchWithValueHandlers` for by-value `self`. With extra arguments the first-argument family is used instead — `MatchFirstWithValueHandlersRef`/`Mut`/(plain) — and the receiver and arguments are bundled into the matcher input as `(context, (args…))`. An `async` method selects the `AsyncComputer` form of both the `where` bound and the matcher call, and `.await`s the result.

## Behavior and corner cases

The **matcher input reflects the receiver and argument list**. A no-argument method passes the bare payload as the input; a method with arguments passes a nested tuple `(self, (arg_0, arg_1, …))`, and the arguments are rebound positionally as `arg_i` (the source patterns are discarded).

**Reference lifetimes are elaborated** so the matcher bound can be quantified. A reference receiver, reference argument, or reference return type with an elided lifetime is rewritten to carry the fresh `'__a__`, and a `for<'__a__>` quantifier is added to the matcher bound when any such lifetime is introduced. A `'static` lifetime is excluded from the quantifier.

The **enum must be extensible**: the blanket impl always carries `__Variants__: HasExtractor`, so the enum needs a `CgpData`/`CgpVariant`-style derive that supplies the extractor and field machinery. Forgetting a per-variant impl surfaces as an unsatisfied matcher bound at the point the enum's method is used, not at the trait definition.

## Known issues

The macro **rejects a trait method with non-lifetime generic parameters** with a spanned `syn::Error` ("Dispatch trait methods cannot contain non-lifetime generic parameters due to the lack of quantified constraints in Rust"). This is a deliberate limitation, not a parser gap: the generated blanket impl would need a bound quantified over the method's type parameter to guarantee every payload satisfies it for all instantiations, and Rust has no such quantified bound. The user-visible consequence is documented under Known issues in [reference/macros/cgp_auto_dispatch.md](../../reference/macros/cgp_auto_dispatch.md); a method that must be generic has to be wired with the dispatch combinators directly instead.

## Tests

The behavioral tests cover every receiver-and-argument shape the matcher selection distinguishes:

- [dispatching/auto_dispatch_shape.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_shape.rs) — a realistic `Shape` enum with a `&self` reader (`area`) and a `&mut self` mutator (`scale`).
- [dispatching/auto_dispatch_self_only.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_self_only.rs), [dispatching/auto_dispatch_self_ref_only.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_self_ref_only.rs), [dispatching/auto_dispatch_self_mut_only.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_self_mut_only.rs) — the by-value, `&self`, and `&mut self` no-argument forms, selecting the three plain matchers.
- [dispatching/auto_dispatch_multi_args.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_multi_args.rs), [dispatching/auto_dispatch_multi_args_ref.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_multi_args_ref.rs), [dispatching/auto_dispatch_multi_args_owned_self.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_multi_args_owned_self.rs) — the argument-taking forms, selecting the first-argument matcher family.
- [dispatching/auto_dispatch_self_ref_return_implicit_ref.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_self_ref_return_implicit_ref.rs), [dispatching/auto_dispatch_self_ref_return_explicit_ref.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_self_ref_return_explicit_ref.rs) — a reference return type with an elided versus an explicit lifetime, pinning the `'__a__` elaboration.
- [dispatching/auto_dispatch_multi_methods.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_multi_methods.rs) — a trait mixing `&self`, `&mut self`, and `self` methods over one enum.
- [dispatching/auto_dispatch_generics.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_generics.rs) — a generic *trait* (`CanCall<T>`) whose per-variant impls add their own bounds; distinct from a generic *method*, which is rejected.
- [dispatching/auto_dispatch_async_self_only.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_async_self_only.rs), [dispatching/auto_dispatch_async_self_ref_only.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_async_self_ref_only.rs), [dispatching/auto_dispatch_async_self_mut_only.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_async_self_mut_only.rs), [dispatching/auto_dispatch_async_multi_args.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_async_multi_args.rs), [dispatching/auto_dispatch_async_multi_args_ref.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_async_multi_args_ref.rs), [dispatching/auto_dispatch_async_multi_args_owned_self.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_async_multi_args_owned_self.rs), [dispatching/auto_dispatch_async_generics.rs](../../../crates/tests/cgp-tests/tests/dispatching/auto_dispatch_async_generics.rs) — the same shapes stacked with [`#[async_trait]`](async_trait.md), selecting the `AsyncComputer` matcher form.

There is no dedicated `snapshot_cgp_auto_dispatch!` macro; the macro's expansion is not pinned by a snapshot and is exercised only behaviorally.

## Source

- Entry point: `cgp_auto_dispatch` in [cgp-extra-macro-lib/src/entrypoints/cgp_auto_dispatch.rs](../../../crates/macros/cgp-extra-macro-lib/src/entrypoints/cgp_auto_dispatch.rs), forwarded from the proc-macro shim in [cgp-extra-macro/src/lib.rs](../../../crates/macros/cgp-extra-macro/src/lib.rs).
- The per-variant handlers are emitted through [`#[cgp_computer]`](cgp_computer.md).
- The value-handler matchers it wires: [crates/extra/cgp-dispatch/src/providers/matchers/](../../../crates/extra/cgp-dispatch/src/providers/matchers/).
