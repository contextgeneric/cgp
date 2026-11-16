# cgp-async-macro

Many traits in [Context-Generic Programming (CGP)](https://contextgeneric.dev/) are async, and
`cgp-async-macro` provides the `#[async_trait]` macro for them, which rewrites each `async fn` in a
trait to return `impl Future` without boxing the future:

```rust,ignore
use cgp::prelude::*;

#[async_trait]
pub trait CanFetch {
    async fn fetch(&self, key: &str) -> String;
}

pub struct Store;

impl CanFetch for Store {
    async fn fetch(&self, key: &str) -> String {
        format!("value of {key}")
    }
}

fn main() {
    // `fetch` returns `impl Future<Output = String>`, which any executor can run.
    let _future = Store.fetch("port");
}
```

Projects use the macro through `cgp::prelude`, so they depend on
[`cgp`](https://crates.io/crates/cgp) alone, as the example does. The website's reference page for
[`#[async_trait]`](https://contextgeneric.dev/docs/reference/macros/async_trait) describes it in
full.

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
