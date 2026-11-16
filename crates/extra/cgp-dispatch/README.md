# cgp-dispatch

[Context-Generic Programming (CGP)](https://contextgeneric.dev/) extends Rust with pluggable trait
implementations, chosen at compile time. Its handlers compose into larger computations, and
`cgp-dispatch` provides the combinators that split a value between them: they match an enum's
variants or extract a record's fields and pass each part to its own handler, or build a record from
what the handlers return. Here each variant of an enum gets its own area computation:

```rust,ignore
use cgp::extra::dispatch::MatchWithValueHandlers;
use cgp::extra::handler::{CanCompute, Computer, ComputerComponent};
use cgp::prelude::*;

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

#[cgp_new_provider]
impl<Context, Code> Computer<Context, Code, Circle> for ComputeArea {
    type Output = f64;

    fn compute(_context: &Context, _code: PhantomData<Code>, circle: Circle) -> f64 {
        3.0 * circle.radius * circle.radius
    }
}

#[cgp_provider]
impl<Context, Code> Computer<Context, Code, Rectangle> for ComputeArea {
    type Output = f64;

    fn compute(_context: &Context, _code: PhantomData<Code>, rectangle: Rectangle) -> f64 {
        rectangle.width * rectangle.height
    }
}

pub struct App;

delegate_components! { App { ComputerComponent: MatchWithValueHandlers<ComputeArea> } }

fn main() {
    let shape = Shape::Rectangle(Rectangle { width: 2.0, height: 3.0 });
    assert_eq!(App.compute(PhantomData::<()>, shape), 6.0);
}
```

The website's reference covers its main items:
[`MatchWithHandlers`](https://contextgeneric.dev/docs/reference/providers/dispatch/match_with_handlers),
[`MatchWithValueHandlers`](https://contextgeneric.dev/docs/reference/providers/dispatch/match_with_value_handlers),
and
[`ExtractFieldAndHandle`](https://contextgeneric.dev/docs/reference/providers/dispatch/extract_field_and_handle).

The `cgp` crate re-exports it as `cgp::extra::dispatch`, so a project depends on
[`cgp`](https://crates.io/crates/cgp) alone, as the example does.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers the API.