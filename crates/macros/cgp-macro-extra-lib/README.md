# cgp-macro-extra-lib

`cgp-macro-extra-lib` implements the extra procedural macros of Context-Generic Programming (CGP)
that `cgp-macro-extra` exports: `#[cgp_computer]`, `#[cgp_producer]`, and `#[cgp_auto_dispatch]`.

Projects use the macros through `cgp::prelude` and have no reason to depend on this crate.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers the API.