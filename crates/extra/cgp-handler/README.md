# cgp-handler

In [Context-Generic Programming (CGP)](https://contextgeneric.dev/), a computation can be a
component too, which each context wires like any other. `cgp-handler` provides that handler family:
components for computations synchronous or async and fallible or not, and the combinators that
chain, nest, and promote them. A provider of `Computer` computes an output from an input:

```rust,ignore
use cgp::extra::handler::{CanCompute, Computer, ComputerComponent};
use cgp::prelude::*;

#[cgp_impl(new Double)]
impl<Code> Computer<Code, u64> {
    type Output = u64;

    fn compute(&self, _code: PhantomData<Code>, input: u64) -> u64 {
        input * 2
    }
}

pub struct App;

delegate_components! { App { ComputerComponent: Double } }

fn main() {
    assert_eq!(App.compute(PhantomData::<()>, 21), 42);
}
```

Projects reach the handlers through the `cgp` crate, which re-exports them as `cgp::extra::handler`,
so they depend on [`cgp`](https://crates.io/crates/cgp) alone, as the example does. The website's
reference has a page for each of its main items:
[`CanCompute`](https://contextgeneric.dev/docs/reference/components/handler/computer),
[`CanHandle`](https://contextgeneric.dev/docs/reference/components/handler/handler),
[`PipeHandlers`](https://contextgeneric.dev/docs/reference/providers/handler/pipe_handlers), and
[`Promote`](https://contextgeneric.dev/docs/reference/providers/handler/promote).

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
