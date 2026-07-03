//! `trybuild` driver for the CGP compile-fail suite.
//!
//! Each fixture under `tests/acceptable/<construct>/` and
//! `tests/problematic/<construct>/` is compiled as its own crate and its
//! diagnostics compared against the sibling `.stderr` file. See the crate
//! [README](../README.md) for what belongs in each category and how to
//! regenerate the `.stderr` snapshots.
//!
//! A single `TestCases` runs both globs so `trybuild` invokes `cargo` once;
//! splitting the categories across two `#[test]` functions would race on the
//! shared build directory. The `**` in each glob descends into the per-construct
//! subdirectories, so a new fixture is picked up with no change here.

#[test]
fn compile_fail() {
    let t = trybuild::TestCases::new();

    // Failures CGP intentionally delegates to the Rust compiler — CGP is
    // working as designed and `rustc` is the right place for the check. These
    // are documented under each owning macro's `## Failure modes` section.
    t.compile_fail("tests/acceptable/**/*.rs");

    // Failures that are a CGP defect: input a macro should have rejected, or an
    // expansion that emits invalid Rust. Each is cross-linked to a `## Known
    // issues` entry in the owning macro's implementation document.
    t.compile_fail("tests/problematic/**/*.rs");
}
