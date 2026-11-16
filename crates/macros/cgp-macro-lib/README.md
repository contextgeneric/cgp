# cgp-macro-lib

`cgp-macro-lib` implements the procedural macros of Context-Generic Programming (CGP), one module
per macro. The `cgp-macro` crate exports them.

Projects use the macros through `cgp::prelude` and have no reason to depend on this crate.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers the API.