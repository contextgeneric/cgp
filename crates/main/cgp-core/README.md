# cgp-core

[Context-Generic Programming (CGP)](https://contextgeneric.dev/) extends Rust with pluggable trait
implementations chosen at compile time, and `cgp-core` gathers the core it rests on: the base layer
and the procedural macros, the `#[async_trait]` macro, and the error, field, and abstract-type
components. A `#[cgp_fn]` function, for example, reads its arguments from the context's fields:

```rust,ignore
use cgp::prelude::*;

#[cgp_fn]
fn rectangle_area(&self, #[implicit] width: f64, #[implicit] height: f64) -> f64 {
    width * height
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

fn main() {
    let rectangle = Rectangle { width: 2.0, height: 3.0 };
    assert_eq!(rectangle.rectangle_area(), 6.0);
}
```

Projects reach `cgp-core` through the `cgp` crate, which re-exports it as `cgp::core`, so they
depend on [`cgp`](https://crates.io/crates/cgp) alone, as the example does. The website's reference
has a page for each of its main items:
[`#[cgp_component]`](https://contextgeneric.dev/docs/reference/macros/cgp_component),
[`HasField`](https://contextgeneric.dev/docs/reference/traits/field-access/has_field), and
[`HasErrorType`](https://contextgeneric.dev/docs/reference/components/has_error_type).

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
