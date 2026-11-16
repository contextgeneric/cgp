# cgp-macro-extra

In [Context-Generic Programming (CGP)](https://contextgeneric.dev/), each pluggable implementation
of a trait is a provider, and some providers follow a common pattern that `cgp-macro-extra` writes
for you: `#[cgp_computer]` and `#[cgp_producer]` define a handler provider from a function, and
`#[cgp_auto_dispatch]` implements a trait for an enum by dispatching to its variants, as here:

```rust,ignore
use cgp::prelude::*;

#[cgp_auto_dispatch]
pub trait HasArea {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        3.0 * self.radius * self.radius
    }
}

pub struct Square {
    pub side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

fn main() {
    assert_eq!(Shape::Square(Square { side: 2.0 }).area(), 4.0);
}
```

Projects use these macros through `cgp::prelude`, so they depend on
[`cgp`](https://crates.io/crates/cgp) alone, as the example does. The website's reference has a page
for each of its main items:
[`#[cgp_computer]`](https://contextgeneric.dev/docs/reference/macros/cgp_computer),
[`#[cgp_producer]`](https://contextgeneric.dev/docs/reference/macros/cgp_producer), and
[`#[cgp_auto_dispatch]`](https://contextgeneric.dev/docs/reference/macros/cgp_auto_dispatch).

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
