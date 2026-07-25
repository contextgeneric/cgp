# `cgp` — Context-Generic Programming in Rust

## Overview

Context-Generic Programming (CGP) is a language extension for Rust, with pluggable trait implementations at compile-time. In ordinary Rust a trait has one implementation per type; CGP lets one trait have many interchangeable implementations and lets each *context* choose which one it uses, through a small wiring table the compiler resolves statically — so the flexibility costs nothing at runtime. The `cgp` crate is the facade over a collection of micro crates that provide it, and it re-exports everything through `cgp::prelude`.

To learn more, see the website [contextgeneric.dev](https://contextgeneric.dev/) and the book [Context-Generic Programming Patterns](https://patterns.contextgeneric.dev/).

<div class="warning">

The `cgp` constructs are still mostly undocumented within Rustdoc. The best way to learn CGP today is the book [Context-Generic Programming Patterns](https://patterns.contextgeneric.dev/); for the exhaustive per-construct semantics, see the [CGP knowledge base](https://github.com/contextgeneric/cgp-knowledge-base).

</div>
