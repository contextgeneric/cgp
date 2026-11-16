# cgp-extra

`cgp-extra` gathers the extra features of Context-Generic Programming (CGP): the handlers that model
computations, the dispatch and monadic combinators that compose them, task runners and the runtime
abstraction, the optional-field builders, and the generic error providers. Its `alloc` feature turns
on the providers that need an allocator.

The `cgp` crate re-exports it as `cgp::extra`. Most projects therefore depend on
[`cgp`](https://crates.io/crates/cgp) alone, without adding this crate to their manifest.

To learn more, read the [`cgp` crate's documentation](https://docs.rs/cgp) and the
[CGP website](https://contextgeneric.dev/), which covers everything from a first program to the
reference for each construct.
