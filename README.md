# Context-Generic Programming (CGP) for Rust

[![Apache 2.0 Licensed](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/contextgeneric/cgp/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/cgp.svg)](https://crates.io/crates/cgp)
[![Docs.rs](https://docs.rs/cgp/badge.svg)](https://docs.rs/cgp)
[![Tests](https://github.com/contextgeneric/cgp/actions/workflows/tests.yml/badge.svg)](https://github.com/contextgeneric/cgp/actions/workflows/tests.yml)
![Rust Stable](https://img.shields.io/badge/rustc-stable-blue.svg)
![Rust 1.89+](https://img.shields.io/badge/rustc-1.89+-blue.svg)

**A language extension for Rust, with pluggable trait implementations at compile-time.**

Still ordinary Rust: a library on the stable toolchain, with provider selection resolved at compile
time, adopted one trait at a time. Beyond interchangeable implementations, CGP adds abstract types
chosen by each context, so error and runtime types need not be separate parameters through every
layer. It also supports extensible records and variants, and composable handlers.

**[Website](https://contextgeneric.dev/) · [Quickstart](https://contextgeneric.dev/docs/quickstart) · [Tutorials](https://contextgeneric.dev/docs/tutorials/hello) · [Reference](https://contextgeneric.dev/docs/reference/) · [crates.io](https://crates.io/crates/cgp) · [API docs](https://docs.rs/cgp)**

> [!IMPORTANT]
> This repository describes **v0.8.0**, which is **not compatible** with v0.7.0, because v0.8.0 changes and removes syntax that v0.7.0 used. **All documentation in this repository describes v0.8.0 only.** For v0.7.0, refer to its published crate documentation instead.

## Install

```bash
cargo add cgp
```

Then bring everything into scope with a single import:

```rust
use cgp::prelude::*;
```

CGP requires **Rust 1.89+** and runs on the **stable** toolchain, and needs neither a nightly toolchain nor a fork. The
[quickstart](https://contextgeneric.dev/docs/quickstart) takes it from there.

## A quick look

Real operations depend on infrastructure such as a database or object storage, and the usual ways of
supplying it do not scale. Passing each dependency as an argument threads long lists through every
layer, while grouping them into a struct and adding methods locks the logic to that one concrete
type, so it cannot be reused for a test, an embedded build, or a different backend. CGP lets you
write the operation once against the dependencies a context provides.

```rust
use cgp::prelude::*;

// A context is a plain data struct.
#[derive(HasField)]
pub struct App {
    pub database: PgPool,
    pub storage_client: Client,
    pub bucket_id: String,
}

// Write the operation once. `#[implicit]` reads `database` from the context;
// `user_id` stays an ordinary argument. Any context carrying a matching
// `database` field gains `get_user` without wiring or trait impls.
#[cgp_fn]
#[async_trait]
pub async fn get_user(
    &self,
    #[implicit] database: &PgPool,
    user_id: &UserId,
) -> anyhow::Result<User> {
    // ...query `database` for the user row
}
```

`get_user` reads like an ordinary async function, yet it works on any context with a matching field, checked at compile time, so a context missing
`database` is a build error rather than a runtime failure. When a step needs *more than one* implementation, promote it to a component and let each
context pick a provider:

```rust
// One interface...
#[async_trait]
#[cgp_component(StorageObjectFetcher)]
pub trait CanFetchStorageObject {
    async fn fetch_storage_object(&self, object_id: &str) -> anyhow::Result<Vec<u8>>;
}

// ...with as many implementations as you need...
#[cgp_impl(new FetchS3Object)]
impl StorageObjectFetcher { /* fetch the object from Amazon S3 */ }

#[cgp_impl(new FetchGCloudObject)]
impl StorageObjectFetcher { /* fetch the object from Google Cloud Storage */ }

// ...and each context picks one, resolved at compile time.
delegate_components! {
    App {
        StorageObjectFetcherComponent: FetchS3Object,
        // ...
    }
}

delegate_components! {
    GCloudApp {
        StorageObjectFetcherComponent: FetchGCloudObject,
        // ...
    }
}
```

`App` fetches from Amazon S3, and `GCloudApp`, a context carrying a Google Cloud storage client
instead, fetches from Google Cloud. Neither uses `dyn` or runtime dispatch, the choice is one
line that a search finds, and code that calls `self.fetch_storage_object(..)` never changes. (Method
bodies are elided here for brevity.)

## Key features

- **One interface, many implementations.** Write interchangeable implementations of one interface and choose between them per application. Separate provider types let implementations coexist while each application makes its choice explicit.
- **Zero-cost abstraction.** Provider selection resolves at compile time and compiles down to direct calls, without a runtime container or vtable lookup, and unused providers are never instantiated. The selected code still does its own work.
- **Type-safe wiring.** Check the wiring at compile time, with `check_components!` or by using the component, so a missing dependency fails the build rather than a call, in safe Rust, without `dyn`, `Any`, or reflection.
- **Abstract over every dependency.** Write core logic that names its error type, runtime, and I/O abstractly and let each context supply the concrete choice, which keeps the core `no_std`-friendly when the chosen implementations support it.
- **Still ordinary Rust.** Adopt CGP one component at a time: providers use familiar impl syntax, implicit arguments resemble parameters, and consumer traits can still be implemented directly.

## Learn CGP

The documentation lives on the website, **[contextgeneric.dev](https://contextgeneric.dev/)**, and
each part answers a different question.

| To… | Read |
| --- | --- |
| Get something running | [Quickstart](https://contextgeneric.dev/docs/quickstart) |
| Learn by building | [Tutorials](https://contextgeneric.dev/docs/tutorials/hello): a first program, then [area calculation](https://contextgeneric.dev/docs/tutorials/area-calculation) |
| Understand why CGP works this way | [Concepts](https://contextgeneric.dev/docs/concepts/): [consumer and provider traits](https://contextgeneric.dev/docs/concepts/consumer-and-provider-traits), [impl-side dependencies](https://contextgeneric.dev/docs/concepts/impl-side-dependencies), [coherence](https://contextgeneric.dev/docs/concepts/coherence), [namespaces](https://contextgeneric.dev/docs/concepts/namespaces) |
| Look up a construct | [Reference](https://contextgeneric.dev/docs/reference/): one page per macro, attribute, derive, provider, and trait |
| Compare it with what you know | [Comparisons](https://contextgeneric.dev/docs/comparisons/): dependency injection, type classes, ML modules, dynamic dispatch, and more |
| See it in a real program | [Projects](https://contextgeneric.dev/docs/projects/) |
| Choose how much CGP a problem needs | [The modularity hierarchy](https://contextgeneric.dev/docs/concepts/modularity-hierarchy) |

## Readable errors: cargo-cgp

CGP compiles to ordinary Rust, so a wiring mistake surfaces as a compiler error about generated
types, often many errors with the real cause hard to find. **When you start with CGP, install
[`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp) alongside it.** It is CGP's error
toolchain: a drop-in for `cargo check` that rewrites those errors into a compact, root-cause-first
form, leading with the field or wiring that actually failed. `cargo cgp check` leads with the root
cause for the classes it recognizes, and the tool does not yet reshape every class.

```bash
cargo install --locked cargo-cgp   # the front-end (needs Rust 1.96 or newer)
cargo cgp setup                    # provisions the compiler + driver it needs
```

Then run `cargo cgp check` wherever you would run `cargo check`; it forces the pinned nightly that
`cargo cgp setup` installed only for its own check, so your project keeps its toolchain untouched.
(Prefer Nix? `nix run github:contextgeneric/cargo-cgp/v0.1.0 -- check` runs it without installing.)
The [installation guide](https://contextgeneric.dev/docs/cargo-cgp/installation) covers every path,
[reading the output](https://contextgeneric.dev/docs/cargo-cgp/reading-output) explains the rewritten
errors, and the [error codes](https://contextgeneric.dev/docs/cargo-cgp/error-codes) catalog every
`[CGP-Exxx]` code.

cargo-cgp is optional and adds only the reading commands `check` and `expand`: keep building,
running, and testing with ordinary cargo, since CGP is a plain library needing only stable Rust
(1.89+), and reach for `cargo cgp check` when you want a wiring error made readable. Plain
`cargo check` still works too, and is fine when you are not expecting a compile error.

## When to use CGP, and when not

CGP is a tool with a boundary, and it earns its cost only past that boundary. The rule of thumb is to
**reach for CGP when an interface needs more than one implementation and the choice belongs to the
context, and not before.** Concretely:

- **One implementation?** Use a plain trait or function. CGP would be over-engineering.
- **A small, closed set of variants?** Use an `enum` and a `match`.
- **Implementations chosen at runtime**, such as plugins, config-driven selection, or heterogeneous collections? Use `dyn Trait`; CGP resolves everything at compile time and gives up that runtime openness by design.
- **One implementation must be globally unique**, as with a single `Ord` for a map key? A coherent trait is the right tool; CGP's per-context choice is deliberately not global.

CGP is a *superset* of ordinary traits, so adopting it starts from the plainer tool rather than
replacing it. You can use it exactly where it pays and leave the rest of a codebase unchanged. The
[modularity hierarchy](https://contextgeneric.dev/docs/concepts/modularity-hierarchy) works through
the choice in more detail.

The costs are real and worth stating beside the benefits: more declarations and wiring, extra
compile-time work, verbose raw diagnostics, and concepts to learn. A delegation entry alone does not
verify its provider's dependencies, so add a check or use the component to force verification.

## Project status

CGP works today on stable Rust and already offers a broad feature set: components and providers,
abstract types, extensible records and variants, a handler and computation family, namespaces, and
modular error handling. It is also young, with the paradigm and its ecosystem still evolving, so
adopting it is a real decision rather than a foregone one.

Two properties make it low-risk to try. Because CGP is a superset of ordinary traits, it can be
introduced in one corner of a codebase and stepped back from without a rewrite; and because it
adds nothing at runtime, it does not lock a project into a framework's lifecycle. If you are evaluating it
for a team, start small, and use the worked examples on the
[website](https://contextgeneric.dev/docs/projects/) to gauge the learning curve honestly.

## Documentation

The [website](https://contextgeneric.dev/) is the main documentation: the quickstart, the tutorials,
the concepts, the construct reference, the comparisons, and the project pages. The crates' API
documentation on [docs.rs](https://docs.rs/cgp) is the place to look up an item.

> [!NOTE]
> Each crate's `docs.rs` page says what that crate provides and links the reference page for its
> constructs. The per-item documentation is still being filled in, so the website remains the place
> to start.

Contributors and coding agents have a second, deeper source: the
[CGP knowledge base](https://github.com/contextgeneric/cgp-knowledge-base), which holds the
exhaustive per-construct reference, the concept overviews, the writing guides, the compile-error
catalog, the worked examples, and the internals of the macros in this repository. It is written for
AI agents rather than for learners, so start at the website if you are learning CGP and reach for the
knowledge base when you need the exact semantics of a construct or are changing this repository. The
[`/cgp` agent skill](https://contextgeneric.dev/docs/ai/skills/) teaches an assistant to read and
write CGP; it does not remove the reviewer's need to understand the code.

Start from [AGENTS.md](AGENTS.md) for the conventions this repository follows, and
[sibling-projects.md](sibling-projects.md) for the related repositories.

## How the project is organized

CGP is a collection of small, layered crates, arranged so that low-level primitives stay independent
of the high-level facade. Most users depend only on the single **`cgp`** crate, which re-exports the
core and extra functionality through `cgp::prelude`. Pluggable error backends are opt-in as separate
crates: `cgp-error-anyhow`, `cgp-error-eyre`, and `cgp-error-std`. Browse [`crates/`](crates) for
the full layout.

## Contributing

Contributions, questions, and bug reports are welcome. Open an
[issue](https://github.com/contextgeneric/cgp/issues) or a pull request, and read the
[contribute page](https://contextgeneric.dev/docs/contribute) to get oriented first. Unless stated
otherwise, contributions are made under the terms of the Apache-2.0 license below.

### Development

Run the checks from the repository root. The toolchain is pinned in `rust-toolchain.toml`, and most
crates are `#![no_std]`, so keep new code `no_std`-compatible.

```sh
cargo +nightly fmt --all
cargo clippy --all-features --all-targets -- -D warnings
cargo clippy --no-default-features --all-targets -- -D warnings
cargo nextest run --all-features --no-fail-fast --workspace
```

The suite covers `cgp-tests` for behavior and expansion snapshots and `cgp-macro-tests` for the macro
internals. Post-codegen compile failures are UI fixtures in
[`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp).

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
