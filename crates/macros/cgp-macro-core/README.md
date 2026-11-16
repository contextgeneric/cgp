# cgp-macro-core

`cgp-macro-core` holds the shared core of the procedural macros of Context-Generic Programming
(CGP): the syntax trees, parsers, visitors, and code-generation helpers that the macro crates build
on. Projects use the macros through `cgp::prelude`, so they have no reason to depend on this crate.

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
