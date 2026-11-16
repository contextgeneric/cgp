# cgp-field-extra

`cgp-field-extra` provides the optional-field layer of Context-Generic Programming (CGP), which
builds a value from fields set in any order. The builder finishes once every field is set, or fills
the missing ones from their defaults.

The `cgp` crate re-exports it as `cgp::extra::field`. Most projects therefore depend on
[`cgp`](https://crates.io/crates/cgp) alone, without adding this crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
