# cgp-error-extra

`cgp-error-extra` provides the generic error providers of Context-Generic Programming (CGP), which
raise an error by conversion, return it unchanged, discard a detail, or format it into a message.
The `DebugError` and `DisplayError` providers need an allocator, so they sit behind the `alloc`
feature.

The `cgp` crate re-exports it as `cgp::extra::error`. Most projects therefore depend on
[`cgp`](https://crates.io/crates/cgp) alone, without adding this crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
