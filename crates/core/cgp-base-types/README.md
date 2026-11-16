# cgp-base-types

[Context-Generic Programming (CGP)](https://contextgeneric.dev/) selects trait implementations at
compile time through types, so the names it works with are types too: a field's name is a type-level
string, and a list of fields is a type-level list. `cgp-base-types` provides those primitives: the
symbols, the lists and sums that describe a type's fields and variants, and the paths that
namespaces route along. A `Product!` list, for example, is a chain of `Cons` cells closed by `Nil`:

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

Projects reach these types through the `cgp` crate, which re-exports them under `cgp::core::base`
and the macros that build them under `cgp::prelude`, so they depend on
[`cgp`](https://crates.io/crates/cgp) alone, as the example does. The website's reference has a page
for each of its main items: [`Symbol!`](https://contextgeneric.dev/docs/reference/macros/symbol),
[`Cons`](https://contextgeneric.dev/docs/reference/types/cons),
[`Nil`](https://contextgeneric.dev/docs/reference/types/nil),
[`Chars`](https://contextgeneric.dev/docs/reference/types/chars),
[`PathCons`](https://contextgeneric.dev/docs/reference/types/path_cons),
[`StaticFormat`](https://contextgeneric.dev/docs/reference/traits/formatting/static_format), and
[`ConcatPath`](https://contextgeneric.dev/docs/reference/traits/formatting/concat_path).

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
