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

## The raw diagnostic

This section describes what plain `cargo check` prints — the fallback when `cargo-cgp` is not on hand; [How cargo-cgp presents it](#how-cargo-cgp-presents-it) below covers the readable form. This is a **surfaced** class, and unusually the root cause *is the primary error* rather than a note beneath it. Forcing the wiring through a [`check_components!`](../../reference/macros/check_components.md) produces an `E0277` whose headline is that the *path* does not implement the namespace lookup trait — `PathCons<Symbol!("app"), PathCons<GreeterComponent, Nil>>: DefaultNamespace<App>` is not satisfied — with the caret on the checked `GreeterComponent` entry. The failing bound's `Self` is a `PathCons<…>` path, not a bare component marker, and that is the signature of the class: the compiler is reporting that the redirect target has no table entry.

Two notes below the headline confirm the reading and are worth recognizing. A `note: required for App to implement DelegateComponent<PathCons<…>>` names the exact missing table entry — the path for which `App` has no delegate — and a `note: required for RedirectLookup<App, PathCons<…>> to implement IsProviderFor<GreeterComponent, App>` names the [`RedirectLookup`](../../reference/providers/redirect_lookup.md) provider the redirect resolved to. A `Self` that is a `RedirectLookup` or a `PathCons` path, rather than a real provider struct, always means the failure is in the lookup, not in a provider's dependencies.

One landmark reads at first like a contradiction and is in fact diagnostic. Beneath the headline the compiler prints `help: the following other types implement trait DefaultNamespace<Components>:` and lists the component *markers* that the namespace does resolve — including the bare `GreeterComponent` marker. That the marker implements `DefaultNamespace` while the *path* `PathCons<app, GreeterComponent>` does not tells you the `#[prefix]` registration succeeded — the component is in the namespace — but the path it routes to was never filled. The problem is a missing binding at the leaf, not a missing prefix.

The diagnostic itself is ordinary `rustc` trait-solving, which is worth seeing so the shape is not mistaken for something exotic. [`E0277`](../error_codes/e0277.md) is the code for an unsatisfied trait bound, and everything here is standard obligation resolution. Proving the check's `App: CanUseComponent<GreeterComponent>` obligation drives the solver through the `RedirectLookup` provider, which implements the component's provider trait only `where Components: DelegateComponent<Path>` — so the solver must discharge `App: DelegateComponent<PathCons<…>>`, finds no impl at that path, and reports *that* bound as the unsatisfied one. The `required for …` lines are `rustc`'s ordinary obligation-tracing output for the proof, and the `help: the following other types implement trait …` list is its stock "an impl of the same trait exists" hint, pointing at the keys the namespace *does* resolve. What makes this class read as "surfaced at the top" rather than buried is only that the unmet bound is a *lookup* (`DelegateComponent`/`DefaultNamespace`) with no impl at all, so it is the first obligation to fail — unlike a [check-trait failure](check-trait-failure.md), where a provider *is* found and the solver descends into its `where` clause before reporting the leaf in a `help:` note.

## Where the root cause is

The root cause is **present and sits at the top of the output**, in the primary `E0277` line and its first `required for … DelegateComponent<PathCons<…>>` note. This is the opposite position from a [check-trait failure](check-trait-failure.md), whose concrete leaf lives in a `help:` note *below* the primary error; here the unsatisfied lookup bound *is* the primary error, and the `required for` notes build outward from it toward the check. Reading the `PathCons<…>` key of that bound tells you which path is unbound — decode it with the [`Symbol!`](../../reference/macros/symbol.md) segments, or read the `long-type-….txt` file the compiler names when the path is elided as `...`.

Exercised without a check — by calling the consumer-trait method on the context — the same broken wiring instead produces the [hidden `E0599`](../hidden/unsatisfied-dependency.md): "the method `greet` exists for struct `App`, but its trait bounds were not satisfied," naming only `App: CanGreet`/`App: Greeter<App>` and descending no further. That output is byte-for-byte the shape of the hidden unsatisfied-dependency class, because the compiler's method-probe heuristic drops the nested lookup bound exactly as it drops a nested dependency bound. So an unregistered path is hidden when reached by a call and surfaced when reached by a check, and only the check reveals which path is empty.

## How cargo-cgp presents it

`cargo-cgp` recasts the raw lookup failure from the machinery's point of view to the programmer's: it keeps the `E0277` code but replaces the `PathCons<…>: DefaultNamespace<App>` headline with `[CGP-E001] the consumer trait \`CanGreet\` is not implemented for context \`App\``, then states the empty path in a `root cause: [CGP-E107] context \`App\` does not contain any delegate entry for \`@app.GreeterComponent\`` note. The tree between them is short — `[CGP-E101]` consumer-trait hop → `[CGP-E104]` redirect-lookup hop naming the path → the `[CGP-E107]` leaf — so the `RedirectLookup` and `DelegateComponent` scaffolding of the raw form collapses into one readable "redirect lands on an empty slot" chain. The `PathCons<Symbol!(…)>` spine is resugared to its surface `@app.GreeterComponent` notation throughout, which is the single biggest readability win over the raw output, where the path is an abbreviated `PathCons<Symbol<3, Chars<..>>, _>`.

The shape scales with how the redirect fails. A deep prefix renders its whole path (`@app.finance.types.QuantityTypeProviderComponent`); a chain of redirects that never terminates prints one `[CGP-E104]` hop per link before the `[CGP-E107]` leaf (`@start.GreeterComponent` → `@middle` → `@end`); and an `open` per-type dispatch missing a key names the type in the path (`@ItemEncoderComponent.Vec<u8>`). In every case the leaf is `[CGP-E107]` — the context wires no provider at the path the component routes to. The codes are defined in the [cargo-cgp error-code catalog](https://github.com/contextgeneric/cargo-cgp/blob/main/docs/error-code.md).

## Resolving it

The fix is to bind a provider at the path the component routes to. Register it from the provider's own definition with [`#[default_impl(@app.GreeterComponent in Namespace)]`](../../reference/traits/default_namespace.md), add a namespace **body** entry (`@app.GreeterComponent: GreetHello`) in the namespace's own crate, or wire the path directly on the context (`@app.GreeterComponent: GreetHello`) — whichever matches where the wiring belongs, per the [namespaces guide](../../guides/namespaces-and-prefixes.md). When the path itself is wrong — a prefix and a wiring that disagree on the path, or a joined namespace that does not inherit the base the component was prefixed into — the fix is to reconcile the two so the redirect and the binding name the same path.

A related variant resolves to a delegate that exists but is not a provider for the component: a path wired to the wrong provider produces an `E0277` where the `RedirectLookup`'s delegate — a named provider — fails to implement the component's provider trait, rather than the `DelegateComponent` lookup failing outright. The remedy there is to wire the path to a provider that actually implements the component.

## Notes for tooling

`cargo-cgp` already extracts the unbound path — the `[CGP-E107]` leaf decodes the `PathCons` key back to `@app.GreeterComponent` and reports that the context wires nothing there. What remains for a tool is the **fix hint** the raw `help:` list makes available but `[CGP-E107]` does not spell out: because the bare component marker still resolves through the namespace while the path does not, the registration succeeded and only the leaf binding is missing — so the actionable advice "you likely forgot a `#[default_impl]` or body entry" could be added rather than left for the reader to infer.

## Backing fixtures

- [`acceptable/resolution/unregistered_prefix_path.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/resolution/unregistered_prefix_path.rs) — a `#[prefix]`-ed `Greeter` joined through `DefaultNamespace` with no provider ever bound at `@app.GreeterComponent`; its `.rust.stderr` pins the primary `E0277` on `PathCons<…>: DefaultNamespace<App>`, the `required for … DelegateComponent<PathCons<…>>` note that names the empty path, the `RedirectLookup … IsProviderFor` note, and the `help:` list in which the bare `GreeterComponent` marker appears, while its `.cgp.stderr` recasts this as `[CGP-E001]` over a `root cause: [CGP-E107] … @app.GreeterComponent` tree with a `[CGP-E104]` redirect hop.
- [`acceptable/wiring/namespace-paths/qualified_prefix_path.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/namespace-paths/qualified_prefix_path.rs) — a deep prefix whose `[CGP-E107]` leaf renders the full `@app.finance.types.QuantityTypeProviderComponent` path, pinning that the resugaring reproduces multi-segment paths intact.
- [`acceptable/wiring/namespace-paths/multi_redirect_missing.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/namespace-paths/multi_redirect_missing.rs) — a chain of redirects that never terminates, so the `.cgp.stderr` tree stacks one `[CGP-E104]` hop per link (`@start.GreeterComponent` → `@middle` → `@end`) before the `[CGP-E107]` leaf at `@end`.
- [`acceptable/wiring/namespace-paths/open_missing_type_key.rs`](https://github.com/contextgeneric/cargo-cgp/blob/main/tests/ui/acceptable/wiring/namespace-paths/open_missing_type_key.rs) — an `open` per-type dispatch missing a type key, where the `[CGP-E107]` leaf names the type in the path (`@ItemEncoderComponent.Vec<u8>`) and the consumer headline is the generic `CanEncodeItem<Vec<u8>>`.

## Related

- [Check-trait failure (surfaced)](check-trait-failure.md) — the sibling surfaced class, where a provider *is* found but its dependency is unmet; contrast the position of the cause (a `help:` note there, the primary error here) and the failing bound (a concrete `HasField` there, a `PathCons` lookup here).
- [Unsatisfied dependency (hidden)](../hidden/unsatisfied-dependency.md) — the `E0599` shape this class takes when exercised by a method call instead of a check; a missing wiring and an unmet dependency are indistinguishable in that hidden form.
- [Verbose dependency cascade](verbose-cascade.md) — when several components route through the same empty path, this diagnostic multiplies the same way.
- [`#[cgp_namespace]`](../../reference/macros/cgp_namespace.md), [`RedirectLookup`](../../reference/providers/redirect_lookup.md), [`DefaultNamespace`](../../reference/traits/default_namespace.md), and the [namespaces guide](../../guides/namespaces-and-prefixes.md) — the routing mechanics and the wiring that fills a path.
- [Debugging CGP compile errors](../../guides/debugging.md) — the `DelegateComponent`/`Namespace` "lookup failed" entry in the decoder.
