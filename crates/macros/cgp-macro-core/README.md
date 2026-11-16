# cgp-macro-core

`cgp-macro-core` holds the shared core of the procedural macros of Context-Generic Programming
(CGP): the syntax trees, parsers, visitors, and code-generation helpers that the macro crates build
on.

Projects use the macros through `cgp::prelude` and have no reason to depend on this crate.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers the API.