# cgp-field

`cgp-field` provides the field access of Context-Generic Programming (CGP): the traits that read and
build a type's fields by name, the tags that name them, and the structural casts between records and
between enums.

The `cgp` crate re-exports it as `cgp::core::field`, and the derives that implement its traits under
`cgp::prelude`. Most projects therefore depend on [`cgp`](https://crates.io/crates/cgp) alone,
without adding this crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
