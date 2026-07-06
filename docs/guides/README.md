# CGP Guides

This directory holds the *guides* to writing Context-Generic Programming code — documents that direct the **choices** an author makes, rather than explaining what a construct is. Where the [reference](../reference/README.md) tells you what a construct means and the [concepts](../concepts/README.md) explain the ideas that tie constructs together, a guide answers the question that comes up once you already understand the pieces: *given several ways to express something, which should I use, and how do I evolve code from one form to another?*

## How guides differ from concepts, reference, and examples

The four sections answer four different questions, and a reader in a hurry can pick the one that matches their need. A [reference document](../reference/README.md) answers "what does `#[prefix]` mean and what does it expand to?" A [concept document](../concepts/README.md) answers "what is a namespace, as an idea?" An [example](../examples/README.md) answers "show me a namespace solving a real problem end to end." A **guide** answers "my `delegate_components!` table has grown unwieldy — what should I do about it, and in what order?" A guide is prescriptive: it recommends a default, names the trade-offs of the alternatives, and often walks a concrete before/after refactoring so the recommendation is grounded in real code rather than stated in the abstract.

A guide leans on the other three rather than restating them. It links to the reference for the exact syntax of each construct it recommends, to the concepts for the mechanism behind a recommendation, and to the examples for a fuller worked scenario. Keep the guide focused on the decision and the migration path; when a guide finds itself explaining what a construct *is* at length, that explanation belongs in the reference or a concept, linked from the guide.

## The catalog

The authoring rules for these documents live in [../AGENTS.md](../AGENTS.md). Each guide below names a decision you face when writing CGP and walks through how to make it; the [Summary](#summary) at the end condenses all of them into one cheat-sheet, so read it first for the recommendations and follow a link for the full before/after mapping and the rules.

- [Writing providers](writing-providers.md) — `#[cgp_impl]` in consumer-trait shape, omitting the context parameter, instead of the inside-out provider forms.
- [Declaring a provider's dependencies](declaring-dependencies.md) — `#[uses]` and `#[use_provider]` instead of hand-written `where` bounds.
- [Reading context fields](reading-context-fields.md) — `#[implicit]` arguments instead of getter traits.
- [Importing abstract types](importing-abstract-types.md) — `#[use_type]` aliases and the concrete-type equality form instead of a supertrait plus `Self::Type`.
- [Adding capability supertraits](capability-supertraits.md) — `#[extend]` instead of native `:` supertrait syntax.
- [Dispatching a component per type](dispatching-per-type.md) — the `open` statement or a namespace instead of a `UseDelegate` table.
- [Organizing wiring with namespaces and prefixes](namespaces-and-prefixes.md) — keeping a growing `delegate_components!` table short with path prefixes, namespaces, and per-type defaults, worked as a refactoring of a real application.
- [Debugging CGP compile errors](debugging.md) — tracing a wiring failure back to its cause: reading the error's shape, moving the error to the wiring site with checks, reducing to a minimal reproduction, inspecting the macro expansion, and a decoder for the errors you actually see.

## Summary

This section condenses every guide above into one quick reference. Read it for the recommendation and the reason; follow a link when you need the full before/after mapping, the corner cases, or a worked example.

**Write CGP that looks like ordinary Rust.** The explicit forms — an inside-out provider-trait `impl`, `where`-clause dependencies, `<Self as Trait>::Type` abstract types, `UseDelegate` dispatch tables — are exactly what the macros desugar to, so you keep reading them in generated code and older codebases, but you should *write* the vanilla-looking form in all new code and reach for an explicit form only when a construct genuinely cannot express the case. Each row below is one such shift:

| When you… | Prefer | Instead of |
|---|---|---|
| write a provider ([guide](writing-providers.md)) | `#[cgp_impl]` with the header `impl Trait` (omit `for Context`, keep `self`/`Self`) | raw `#[cgp_provider]`/`#[cgp_new_provider]` in inside-out shape |
| require a capability or an inner provider ([guide](declaring-dependencies.md)) | `#[uses(Trait)]` / `#[use_provider(P: Trait)]`, comma-separated in one attribute | hand-written `Self:`/`P: Trait<Self>` `where` bounds |
| read a value from the context's own field ([guide](reading-context-fields.md)) | an `#[implicit]` argument | a getter trait declared only to read it |
| name an abstract type ([guide](importing-abstract-types.md)) | `#[use_type(Trait.Type)]` + the bare alias (`Trait.Type in Context` for a foreign type, `{Type = Concrete}` to pin one) | a `: Trait` supertrait + qualified `Self::Type`, or `where Context: Trait` + `Context::Type` |
| add a non-type capability supertrait ([guide](capability-supertraits.md)) | `#[extend(Trait)]` | native `: Supertrait` inheritance syntax |
| dispatch a generic-parameter component per type ([guide](dispatching-per-type.md)) | the `open` statement or a [namespace](namespaces-and-prefixes.md) | `#[derive_delegate]` + a `UseDelegate` nested table |

**When an explicit form is still right.** Keep a hand-written `where` clause for an associated-type-equality bound on a trait you would not `#[use_type]` from (`Iterator<Item = u8>`, `From<X>`) — but move an *abstract-type* pin like `Self: HasErrorType<Error = AppError>` into the `#[use_type]` equality form. Name the context explicitly (`impl<Context> Trait for Context`) only for a lifetime or higher-ranked bound the sugar cannot carry, or reach for `#[cgp_getter]` only when a context must choose per-wiring which field a getter reads. And a construct's own local associated type stays qualified as `Self::Output` always — it is never a `#[use_type]` import.

**Keep a growing wiring table short with namespaces and prefixes** ([guide](namespaces-and-prefixes.md)). As component counts rise, group components under path prefixes with `#[prefix(@path in DefaultNamespace)]`, lift a backend's provider choices into a reusable namespace (via `#[default_impl]` or namespace body entries) so a context joins it with one `namespace N;` line, and pull a separate concern's table in with a `for … in` loop. Two rules bite: a context can only override a path its namespace routes *to* but does not itself register, and a prefixed component's `#[default_impl]` must live in the namespace's own crate.

**When a wiring fails to compile, trace it, don't stare at it** ([guide](debugging.md)). CGP wiring is lazy, so one broken link surfaces as many errors on distant lines naming generated types; read the failing *trait* (an unmet `IsProviderFor` means a dependency is missing, a failed `DelegateComponent` means the lookup has no entry), move the error to the wiring site with `check_components!`, and when a large program puzzles you, reduce it to the smallest reproduction — or snapshot the expansion — rather than reasoning about coherence on paper.
