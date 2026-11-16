# cgp-field-extra

[Context-Generic Programming (CGP)](https://contextgeneric.dev/) extends Rust with pluggable trait
implementations, chosen at compile time. Its records can be built field by field, and
`cgp-field-extra` provides the optional-field layer for that: it builds a value from fields set in
any order, then finishes once every field is set, or fills the missing ones from their defaults. A
record can be widened into a larger one this way:

```rust,ignore
use cgp::extra::field::impls::CanBuildWithDefault;
use cgp::prelude::*;

#[derive(Debug, PartialEq, CgpData)]
pub struct Point2d {
    pub x: u64,
    pub y: u64,
}

#[derive(Debug, PartialEq, CgpData)]
pub struct Point3d {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

fn main() {
    let point = Point3d::build_with_default(Point2d { x: 1, y: 2 });
    assert_eq!(point, Point3d { x: 1, y: 2, z: 0 });
}
```

The website's reference covers its main items:
[`HasOptionalBuilder`](https://contextgeneric.dev/docs/reference/traits/optional/has_optional_builder),
[`ToOptional`](https://contextgeneric.dev/docs/reference/traits/optional/to_optional),
[`SetOptional`](https://contextgeneric.dev/docs/reference/traits/optional/set_optional), and
[`FinalizeOptional`](https://contextgeneric.dev/docs/reference/traits/optional/finalize_optional).

The `cgp` crate re-exports it as `cgp::extra::field`, so a project depends on
[`cgp`](https://crates.io/crates/cgp) alone, as the example does.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers the API.