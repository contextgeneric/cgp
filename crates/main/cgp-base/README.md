# cgp-base

`cgp-base` gathers the base layer of Context-Generic Programming (CGP): the type-level primitives,
the component machinery, and the procedural macros that CGP's higher layers build on. Projects reach
it through the `cgp` crate, which re-exports it as `cgp::core::base`, so most depend on
[`cgp`](https://crates.io/crates/cgp) alone and never add this crate to their manifest.

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
