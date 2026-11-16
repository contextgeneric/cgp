# cgp-async-macro

[Context-Generic Programming (CGP)](https://contextgeneric.dev/) extends Rust with pluggable trait
implementations, chosen at compile time. `cgp-async-macro` provides CGP's `#[async_trait]` macro,
which rewrites each `async fn` in a trait to return `impl Future`, without boxing the future:

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

Its reference page on the website is
[`#[async_trait]`](https://contextgeneric.dev/docs/reference/macros/async_trait).

The `cgp` crate gathers its macro in `cgp::prelude`, so a project depends on
[`cgp`](https://crates.io/crates/cgp) alone, as the example does.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers its API.
