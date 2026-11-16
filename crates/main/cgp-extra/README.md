# cgp-extra

[Context-Generic Programming (CGP)](https://contextgeneric.dev/) extends Rust with pluggable trait
implementations, chosen at compile time. `cgp-extra` gathers the features built on top: the handlers
and the dispatch and monadic combinators that compose them, task runners and the runtime
abstraction, the optional-field builders, and the generic error providers. Its `alloc` feature turns
on the providers that need an allocator. A handler pipeline, for example, runs one computer after
another:

```rust,ignore
use cgp::extra::handler::{CanCompute, Computer, ComputerComponent, PipeHandlers};
use cgp::prelude::*;

#[cgp_impl(new Double)]
impl<Code> Computer<Code, u64> {
    type Output = u64;

    fn compute(&self, _code: PhantomData<Code>, input: u64) -> u64 {
        input * 2
    }
}

#[cgp_impl(new Increment)]
impl<Code> Computer<Code, u64> {
    type Output = u64;

    fn compute(&self, _code: PhantomData<Code>, input: u64) -> u64 {
        input + 1
    }
}

pub struct App;

delegate_components! { App { ComputerComponent: PipeHandlers<Product![Double, Increment]> } }

fn main() {
    assert_eq!(App.compute(PhantomData::<()>, 20), 41);
}
```

The website's reference covers its main items:
[`CanHandle`](https://contextgeneric.dev/docs/reference/components/handler/handler),
[`PipeHandlers`](https://contextgeneric.dev/docs/reference/providers/handler/pipe_handlers), and
[`RaiseFrom`](https://contextgeneric.dev/docs/reference/providers/error/raise_from).

The `cgp` crate re-exports it as `cgp::extra`, so a project depends on
[`cgp`](https://crates.io/crates/cgp) alone, as the example does.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers the API.