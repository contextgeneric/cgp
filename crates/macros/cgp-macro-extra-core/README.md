# cgp-macro-extra-core

`cgp-macro-extra-core` holds the shared core of the extra procedural macros of Context-Generic
Programming (CGP): the syntax trees, parsers, and visitors behind `#[cgp_computer]`,
`#[cgp_producer]`, and `#[cgp_auto_dispatch]`. Projects use the macros through `cgp::prelude`, so
they have no reason to depend on this crate.

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
