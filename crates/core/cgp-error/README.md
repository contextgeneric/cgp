# cgp-error

`cgp-error` provides the error components of Context-Generic Programming (CGP): an error type that
each context chooses, and the components that raise errors into it and wrap them with detail.

The `cgp` crate re-exports it as `cgp::core::error`, and its traits `HasErrorType`, `CanRaiseError`,
and `CanWrapError` under `cgp::prelude`. Most projects therefore depend on
[`cgp`](https://crates.io/crates/cgp) alone, without adding this crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
