# cgp-dispatch

`cgp-dispatch` provides the dispatch combinators of Context-Generic Programming (CGP), which split a
value between handlers. They match an enum's variants or extract a record's fields and pass each one
to its own handler, or build a record from what the handlers return.

The `cgp` crate re-exports it as `cgp::extra::dispatch`. Most projects therefore depend on
[`cgp`](https://crates.io/crates/cgp) alone, without adding this crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
