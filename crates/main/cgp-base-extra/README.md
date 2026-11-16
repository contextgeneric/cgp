# cgp-base-extra

`cgp-base-extra` adds the abstract types of Context-Generic Programming (CGP) to CGP's base layer,
the foundation CGP's error components are built on. It is an internal layer of the `cgp` crate,
which re-exports everything it holds under `cgp::core`, so most projects depend on
[`cgp`](https://crates.io/crates/cgp) alone and never add this crate to their manifest.

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
