# cgp-type

[Context-Generic Programming (CGP)](https://contextgeneric.dev/) lets each context choose a trait's
implementation at compile time, and it can choose a type the same way. `cgp-type` provides these
abstract types, which generic code names only through a trait while each context picks the concrete
type in its wiring:

```rust,ignore
use cgp::prelude::*;

#[cgp_type]
pub trait HasNameType {
    type Name;
}

pub struct App;

delegate_components! { App { NameTypeProviderComponent: UseType<String> } }

fn main() {
    let name: <App as HasNameType>::Name = String::from("app");
    assert_eq!(name, "app");
}
```

The website's reference covers its main items:
[`HasType`](https://contextgeneric.dev/docs/reference/components/has_type),
[`UseType`](https://contextgeneric.dev/docs/reference/providers/use_type), and
[`UseDelegatedType`](https://contextgeneric.dev/docs/reference/providers/use_delegated_type).

The `cgp` crate re-exports it as `cgp::core::types`, so a project depends on
[`cgp`](https://crates.io/crates/cgp) alone, as the example does.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers the API.