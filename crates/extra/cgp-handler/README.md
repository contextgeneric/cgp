# cgp-handler

[Context-Generic Programming (CGP)](https://contextgeneric.dev/) extends Rust with pluggable trait
implementations, chosen at compile time. `cgp-handler` provides CGP's handler family: the components
that model a computation, synchronous or async and fallible or not, and the combinators that chain,
nest, and promote them. A provider of `Computer` computes an output from an input:

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

Its main items are
[`CanCompute`](https://contextgeneric.dev/docs/reference/components/handler/computer),
[`CanHandle`](https://contextgeneric.dev/docs/reference/components/handler/handler),
[`PipeHandlers`](https://contextgeneric.dev/docs/reference/providers/handler/pipe_handlers), and
[`Promote`](https://contextgeneric.dev/docs/reference/providers/handler/promote), each linked to its
reference page on the website.

The `cgp` crate re-exports it as `cgp::extra::handler`, so a project depends on
[`cgp`](https://crates.io/crates/cgp) alone, as the example does.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers its API.
