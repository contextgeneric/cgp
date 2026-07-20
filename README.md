# Context-Generic Programming (CGP)

[![Apache 2.0 Licensed](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/contextgeneric/cgp/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/cgp.svg)](https://crates.io/crates/cgp)
[![Tests](https://github.com/contextgeneric/cgp/actions/workflows/tests.yml/badge.svg)](https://github.com/contextgeneric/cgp/actions/workflows/tests.yml)
![Rust Stable](https://img.shields.io/badge/rustc-stable-blue.svg)
![Rust 1.89+](https://img.shields.io/badge/rustc-1.89+-blue.svg)

**A language extension for Rust, with pluggable trait implementations at compile-time.**

Context-Generic Programming (CGP) is a library on stable Rust that lifts the language's one-implementation-per-type limit off your traits: it lets one interface have many interchangeable implementations and lets each *context* — an application, a test, a deployment — plug in the one it uses. The choice is written in one readable place and resolved entirely during compilation, so it compiles down to direct calls: there is no runtime container, no reflection, and nothing left in the binary for an implementation a context does not use.

> [!IMPORTANT]
> The `main` branch tracks the upcoming **v0.8.0** release (published as `0.8.0-alpha` pre-release crates). The current stable release on crates.io is **v0.7.0**, which is **not compatible** with this branch — v0.8.0 changes and removes syntax that v0.7.0 used. **All documentation in this repository describes v0.8.0 only.** For v0.7.0, refer to its published crate documentation instead.

**[Website](https://contextgeneric.dev/) · [crates.io](https://crates.io/crates/cgp)**

## Key features

CGP's headline promises are few and deliberately concrete.

- **One interface, many implementations.** Write many interchangeable implementations of the same interface and choose between them per context — including the overlapping and orphan implementations Rust normally forbids, made safe because every choice is explicit and local.
- **Zero-cost abstraction.** Everything is resolved at compile time and compiles down to direct calls, so the flexibility costs nothing at runtime and unused implementations never reach the binary.
- **Type-safe wiring.** All wiring is checked at compile time, so a missing dependency is a build error rather than a runtime failure — in safe Rust, with no `dyn`, `Any`, or reflection.
- **Abstract over every dependency.** Write core logic that names its error type, runtime, and I/O abstractly and let each context supply the concrete choice, which keeps the core `no_std`-friendly — from embedded systems to WebAssembly.
- **Still ordinary Rust.** CGP is a superset of ordinary traits that you adopt one piece at a time: providers read like normal impls and implicit arguments like normal parameters, so a codebase can use it in one corner and stay otherwise vanilla.

## A quick look

Real operations depend on infrastructure — a database, object storage — and the usual ways of supplying it do not scale. Passing each dependency as an argument threads long lists through every layer, while grouping them into a struct and adding methods locks the logic to that one concrete type, so it cannot be reused for a test, an embedded build, or a different backend. CGP lets you write the operation once against the capabilities a context provides.

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
// `database` field gains `get_user` — no wiring, no trait impls to write.
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

`get_user` reads like an ordinary async function, yet it works on any context with a matching field — checked at compile time, so a context missing `database` is a build error rather than a runtime failure. When a step needs *more than one* implementation, promote it to a component and let each context pick a provider:

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

`App` fetches from Amazon S3, and `GCloudApp` — a context carrying a Google Cloud storage client instead — fetches from Google Cloud. Neither pays for `dyn` or runtime dispatch, the choice is a single greppable line, and code that calls `self.fetch_storage_object(..)` never changes. (Method bodies are elided here for brevity.)

## Installation

Add the latest stable release with Cargo:

```bash
cargo add cgp
```

then bring everything into scope with a single import:

```rust
use cgp::prelude::*;
```

CGP requires **Rust 1.89+** and runs on the **stable** toolchain — no nightly, no fork.

> [!NOTE]
> `cargo add cgp` installs the current stable release, **v0.7.0**. The examples in this repository target **v0.8.0**; to follow them today, depend on the pre-release from `main`:
> ```toml
> cgp = { git = "https://github.com/contextgeneric/cgp" }
> ```
> Once v0.8.0 is released, `cargo add cgp` will install it.

### Install cargo-cgp for readable errors

CGP compiles to ordinary Rust, so a wiring mistake surfaces as a compiler error about generated types — often a wall of them with the real cause buried. **When you start with CGP, install [`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp) alongside it.** cargo-cgp is CGP's error toolchain: a drop-in for `cargo check` that rewrites those errors into a compact, root-cause-first form, leading with the field or wiring that actually failed. It is the single biggest quality-of-life improvement for writing CGP, so treat it as part of your getting-started setup.

```bash
cargo install cargo-cgp     # the front-end (installs on any toolchain)
cargo cgp setup             # provisions the compiler + driver it needs
```

Then run `cargo cgp check` wherever you would run `cargo check`; it forces the pinned nightly that `cargo cgp setup` installed only for its own check, so your project keeps its toolchain untouched. (Prefer Nix? `nix run github:contextgeneric/cargo-cgp/v0.1.0-alpha -- check` runs it without installing.) cargo-cgp is an early pre-release that already makes the common wiring errors readable and is steadily covering more; see its [README](https://github.com/contextgeneric/cargo-cgp) for the full guide.

cargo-cgp is optional and adds only a diagnostic `check`: keep building, running, and testing with ordinary cargo — CGP is a plain library needing only stable Rust (1.89+) — and reach for `cargo cgp check` when you want a wiring error made readable. Plain `cargo check` still works too, and is fine when you are not expecting a compile error.

## When to use CGP — and when not

CGP is a tool with a boundary, and it earns its cost only past that boundary. The rule of thumb is to **reach for CGP when a capability needs more than one implementation and the choice belongs to the context — not before.** Concretely:

- **One implementation?** Use a plain trait or function. CGP would be over-engineering.
- **A small, closed set of variants?** Use an `enum` and a `match`.
- **Implementations chosen at runtime** — plugins, config-driven selection, heterogeneous collections? Use `dyn Trait`; CGP resolves everything at compile time and gives up that runtime openness by design.
- **One implementation must be globally unique** — a single `Ord` for a map key, say? A coherent trait is the right tool; CGP's per-context choice is deliberately not global.

CGP is a *superset* of ordinary traits, so adopting it is a climb from the plainer tool rather than a rejection of it. You can use it exactly where it pays and leave the rest of a codebase unchanged.

## Documentation

Learn CGP at the project website, **[contextgeneric.dev](https://contextgeneric.dev/)**, which hosts the guides, worked examples, and project blog.

> [!NOTE]
> The crates' API docs on `docs.rs` are still sparse. The website is the place to start while the inline documentation is filled in.

## Project status

CGP works today on stable Rust and already offers a broad feature set: components and providers, abstract types, extensible records and variants, a handler and computation family, namespaces, and modular error handling. It is also young — the paradigm and its ecosystem are still evolving, and v0.8.0 is a pre-release — so adopting it is a real decision rather than a foregone one.

Two properties make it low-risk to try. Because CGP is a superset of ordinary traits, it can be introduced in one corner of a codebase and stepped back from without a rewrite; and because it imposes no runtime, it does not lock a project into a framework's lifecycle. If you are evaluating it for a team, start small, and lean on the worked examples on the [website](https://contextgeneric.dev/) to gauge the learning curve honestly.

## How the project is organized

CGP is a collection of small, layered crates, arranged so that low-level primitives stay independent of the high-level facade. Most users depend only on the single **`cgp`** crate, which re-exports the core and extra functionality through `cgp::prelude`. Pluggable error backends are opt-in as separate crates — `cgp-error-anyhow`, `cgp-error-eyre`, and `cgp-error-std`. Browse [`crates/`](crates) for the full layout.

## Contributing

Contributions, questions, and bug reports are welcome. Open an [issue](https://github.com/contextgeneric/cgp/issues) or a pull request, and see the [website](https://contextgeneric.dev/) to get oriented first. Unless stated otherwise, contributions are made under the terms of the Apache-2.0 license below.

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
