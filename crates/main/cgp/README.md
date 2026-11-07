# `cgp`: Context-Generic Programming in Rust

**A language extension for Rust, with pluggable trait implementations at compile-time.**

The `cgp` crate is the facade over the collection of crates that provide CGP, and it re-exports
everything through `cgp::prelude`. In ordinary Rust a trait has one implementation per type; CGP lets
one trait have many interchangeable implementations and lets each *context* choose which one it uses,
through a small wiring table the compiler resolves statically, so provider selection adds nothing at
runtime.

## Learn CGP

The documentation lives on the website, [contextgeneric.dev](https://contextgeneric.dev/):

- [Quickstart](https://contextgeneric.dev/docs/quickstart): install it and run a first program.
- [Tutorials](https://contextgeneric.dev/docs/tutorials/hello): learn by building.
- [Concepts](https://contextgeneric.dev/docs/concepts/): why CGP works the way it does.
- [Reference](https://contextgeneric.dev/docs/reference/): one page per construct.

The API documentation here covers the items themselves; the website explains how to use them. For the
exhaustive per-construct semantics, the
[CGP knowledge base](https://github.com/contextgeneric/cgp-knowledge-base) is written for agents and
records the internals.

## Install

```bash
cargo add cgp
```

```rust
use cgp::prelude::*;
```

CGP requires Rust 1.89+ and runs on the stable toolchain. When you start with CGP, install
[`cargo-cgp`](https://github.com/contextgeneric/cargo-cgp) alongside it: `cargo cgp check` leads with
the root cause for the classes it recognizes, and the tool does not yet reshape every class.

## License

Licensed under the [Apache License, Version 2.0](https://github.com/contextgeneric/cgp/blob/main/LICENSE).
