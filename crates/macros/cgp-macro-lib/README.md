# cgp-macro-lib

`cgp-macro-lib` implements the procedural macros of Context-Generic Programming (CGP), one module
per macro, and the `cgp-macro` crate exports them. Projects use the macros through `cgp::prelude`,
so they have no reason to depend on this crate.

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
