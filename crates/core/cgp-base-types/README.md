# cgp-base-types

[Context-Generic Programming (CGP)](https://contextgeneric.dev/) extends Rust with pluggable trait
implementations, chosen at compile time. `cgp-base-types` provides CGP's type-level primitives: the
symbols that name fields, the lists and sums that describe a type's fields and variants, and the
paths that namespaces route along. A `Product!` list, for example, is a chain of `Cons` cells closed
by `Nil`:

```rust,ignore
use cgp::core::field::types::{Cons, Nil};
use cgp::prelude::*;

// `Product![u8, String]` is the list `Cons<u8, Cons<String, Nil>>`.
fn as_cells(list: Product![u8, String]) -> Cons<u8, Cons<String, Nil>> {
    list
}

fn main() {
    let Cons(first, Cons(second, Nil)) = as_cells(product![1, "one".to_owned()]);
    assert_eq!((first, second.as_str()), (1, "one"));
}
```

Its main items are [`Symbol!`](https://contextgeneric.dev/docs/reference/macros/symbol),
[`Cons`](https://contextgeneric.dev/docs/reference/types/cons),
[`Nil`](https://contextgeneric.dev/docs/reference/types/nil),
[`Chars`](https://contextgeneric.dev/docs/reference/types/chars),
[`PathCons`](https://contextgeneric.dev/docs/reference/types/path_cons),
[`StaticFormat`](https://contextgeneric.dev/docs/reference/traits/formatting/static_format), and
[`ConcatPath`](https://contextgeneric.dev/docs/reference/traits/formatting/concat_path), each linked
to its reference page on the website.

The `cgp` crate re-exports these types under `cgp::core::base`, and the macros that build them under
`cgp::prelude`, so a project depends on [`cgp`](https://crates.io/crates/cgp) alone, as the example
does.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers its API.
