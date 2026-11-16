# cgp-macro-extra-lib

`cgp-macro-extra-lib` implements the extra procedural macros of Context-Generic Programming (CGP),
`#[cgp_computer]`, `#[cgp_producer]`, and `#[cgp_auto_dispatch]`, which `cgp-macro-extra` exports.
Projects use the macros through `cgp::prelude`, so they have no reason to depend on this crate.

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
