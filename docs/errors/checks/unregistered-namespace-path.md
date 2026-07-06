# Unregistered namespace path

A context joins a namespace that *routes* a component to a path but never *binds* a provider at that path, so the lookup finds no delegate — and a check surfaces the failure as an `E0277` naming the path-keyed `DefaultNamespace` (or `DelegateComponent`) bound that has no impl.

## What triggers it

This class is a *lookup* failure rather than a *dependency* failure: no provider is found for the component at all, as opposed to a provider being found whose `where` clause is unmet. It arises when a component is registered into a [namespace](../../reference/macros/cgp_namespace.md) under a path — through [`#[prefix]`](../../reference/macros/cgp_namespace.md) — and a context joins that namespace, but nothing ever terminates the path with a concrete provider. The prefix registers the *routing* (the namespace resolves the component to a `RedirectLookup` along the path), yet the path itself stays empty because the author forgot the `#[default_impl]`, the namespace body entry, or the direct wiring that would place a provider there.

```rust
#[cgp_component(Greeter)]
#[prefix(@app in DefaultNamespace)] // routes GreeterComponent to @app.GreeterComponent
pub trait CanGreet {
    fn greet(&self) -> String;
}

#[cgp_impl(new GreetHello)]
impl Greeter {
    fn greet(&self) -> String { "Hello".to_owned() }
}

pub struct App;

// App joins the namespace, so GreeterComponent follows the @app.GreeterComponent
// redirect — but nothing (no #[default_impl], no body entry, no direct line) ever
// binds GreetHello, or anything, at that path.
delegate_components! {
    App {
        namespace DefaultNamespace;
    }
}

check_components! {
    App {
        GreeterComponent,
    }
}
```

The same shape appears whenever a redirect lands on an empty path: a context whose joined namespace does not inherit the base namespace a component was prefixed into, a `@`-path entry with a mistyped or too-short segment, or a component prefixed under one path but wired under another. In every case the redirect resolves to a table slot that holds nothing.

## The diagnostic

This is a **surfaced** class, and unusually the root cause *is the primary error* rather than a note beneath it. Forcing the wiring through a [`check_components!`](../../reference/macros/check_components.md) produces an `E0277` whose headline is that the *path* does not implement the namespace lookup trait — `PathCons<Symbol!("app"), PathCons<GreeterComponent, Nil>>: DefaultNamespace<App>` is not satisfied — with the caret on the checked `GreeterComponent` entry. The failing bound's `Self` is a `PathCons<…>` path, not a bare component marker, and that is the signature of the class: the compiler is reporting that the redirect target has no table entry.

Two notes below the headline confirm the reading and are worth recognizing. A `note: required for App to implement DelegateComponent<PathCons<…>>` names the exact missing table entry — the path for which `App` has no delegate — and a `note: required for RedirectLookup<App, PathCons<…>> to implement IsProviderFor<GreeterComponent, App>` names the [`RedirectLookup`](../../reference/providers/redirect_lookup.md) provider the redirect resolved to. A `Self` that is a `RedirectLookup` or a `PathCons` path, rather than a real provider struct, always means the failure is in the lookup, not in a provider's dependencies.

One landmark reads at first like a contradiction and is in fact diagnostic. Beneath the headline the compiler prints `help: the following other types implement trait DefaultNamespace<Components>:` and lists the component *markers* that the namespace does resolve — including the bare `GreeterComponent` marker. That the marker implements `DefaultNamespace` while the *path* `PathCons<app, GreeterComponent>` does not tells you the `#[prefix]` registration succeeded — the component is in the namespace — but the path it routes to was never filled. The problem is a missing binding at the leaf, not a missing prefix.

The diagnostic itself is ordinary `rustc` trait-solving, which is worth seeing so the shape is not mistaken for something exotic. [`E0277`](../error_codes/e0277.md) is the code for an unsatisfied trait bound, and everything here is standard obligation resolution. Proving the check's `App: CanUseComponent<GreeterComponent>` obligation drives the solver through the `RedirectLookup` provider, which implements the component's provider trait only `where Components: DelegateComponent<Path>` — so the solver must discharge `App: DelegateComponent<PathCons<…>>`, finds no impl at that path, and reports *that* bound as the unsatisfied one. The `required for …` lines are `rustc`'s ordinary obligation-tracing output for the proof, and the `help: the following other types implement trait …` list is its stock "an impl of the same trait exists" hint, pointing at the keys the namespace *does* resolve. What makes this class read as "surfaced at the top" rather than buried is only that the unmet bound is a *lookup* (`DelegateComponent`/`DefaultNamespace`) with no impl at all, so it is the first obligation to fail — unlike a [check-trait failure](check-trait-failure.md), where a provider *is* found and the solver descends into its `where` clause before reporting the leaf in a `help:` note.

## Where the root cause is

The root cause is **present and sits at the top of the output**, in the primary `E0277` line and its first `required for … DelegateComponent<PathCons<…>>` note. This is the opposite position from a [check-trait failure](check-trait-failure.md), whose concrete leaf lives in a `help:` note *below* the primary error; here the unsatisfied lookup bound *is* the primary error, and the `required for` notes build outward from it toward the check. Reading the `PathCons<…>` key of that bound tells you which path is unbound — decode it with the [`Symbol!`](../../reference/macros/symbol.md) segments, or read the `long-type-….txt` file the compiler names when the path is elided as `...`.

Exercised without a check — by calling the consumer-trait method on the context — the same broken wiring instead produces the [hidden `E0599`](../hidden/unsatisfied-dependency.md): "the method `greet` exists for struct `App`, but its trait bounds were not satisfied," naming only `App: CanGreet`/`App: Greeter<App>` and descending no further. That output is byte-for-byte the shape of the hidden unsatisfied-dependency class, because the compiler's method-probe heuristic drops the nested lookup bound exactly as it drops a nested dependency bound. So an unregistered path is hidden when reached by a call and surfaced when reached by a check, and only the check reveals which path is empty.

## Resolving it

The fix is to bind a provider at the path the component routes to. Register it from the provider's own definition with [`#[default_impl(@app.GreeterComponent in Namespace)]`](../../reference/traits/default_namespace.md), add a namespace **body** entry (`@app.GreeterComponent: GreetHello`) in the namespace's own crate, or wire the path directly on the context (`@app.GreeterComponent: GreetHello`) — whichever matches where the wiring belongs, per the [namespaces guide](../../guides/namespaces-and-prefixes.md). When the path itself is wrong — a prefix and a wiring that disagree on the path, or a joined namespace that does not inherit the base the component was prefixed into — the fix is to reconcile the two so the redirect and the binding name the same path.

A related variant resolves to a delegate that exists but is not a provider for the component: a path wired to the wrong provider produces an `E0277` where the `RedirectLookup`'s delegate — a named provider — fails to implement the component's provider trait, rather than the `DelegateComponent` lookup failing outright. The remedy there is to wire the path to a provider that actually implements the component.

## Notes for tooling

For a `cargo-cgp`-style post-processor the fact to extract is the **unbound path**: decode the `PathCons<Symbol!(…)>` key of the failing `DefaultNamespace`/`DelegateComponent` bound back to its readable `@app.GreeterComponent` form and report "no provider is wired at `@app.GreeterComponent`, which `GreeterComponent` routes to through `DefaultNamespace`." The `RedirectLookup` and `CanUseComponent` frames in the `required for` chain are scaffolding to collapse, and the `help:` list of markers that *do* resolve is worth mining for the near-contradiction — if the bare marker is in that list, the tool can add "the component is registered but its path is empty; you likely forgot a `#[default_impl]` or body entry." Because the method-call form hides the cause as `E0599`, a tool that promotes hidden errors by synthesizing a check (see the [hidden class](../hidden/unsatisfied-dependency.md)) turns this from its hidden form into the surfaced diagnostic it can then decode.

## Backing fixtures

- [acceptable/cgp_namespace/unregistered_prefix_path.rs](../../../crates/tests/cgp-compile-fail-tests/tests/acceptable/cgp_namespace/unregistered_prefix_path.rs) — a `#[prefix]`-ed `Greeter` joined through `DefaultNamespace` with no provider ever bound at `@app.GreeterComponent`; its `.stderr` pins the primary `E0277` on `PathCons<app, GreeterComponent>: DefaultNamespace<App>`, the `required for … DelegateComponent<PathCons<…>>` note that names the empty path, the `RedirectLookup … IsProviderFor` note, and the `help:` list in which the bare `GreeterComponent` marker appears.

## Related

- [Check-trait failure (surfaced)](check-trait-failure.md) — the sibling surfaced class, where a provider *is* found but its dependency is unmet; contrast the position of the cause (a `help:` note there, the primary error here) and the failing bound (a concrete `HasField` there, a `PathCons` lookup here).
- [Unsatisfied dependency (hidden)](../hidden/unsatisfied-dependency.md) — the `E0599` shape this class takes when exercised by a method call instead of a check; a missing wiring and an unmet dependency are indistinguishable in that hidden form.
- [Verbose dependency cascade](verbose-cascade.md) — when several components route through the same empty path, this diagnostic multiplies the same way.
- [`#[cgp_namespace]`](../../reference/macros/cgp_namespace.md), [`RedirectLookup`](../../reference/providers/redirect_lookup.md), [`DefaultNamespace`](../../reference/traits/default_namespace.md), and the [namespaces guide](../../guides/namespaces-and-prefixes.md) — the routing mechanics and the wiring that fills a path.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `DelegateComponent`/`Namespace` "lookup failed" entry in the decoder.
