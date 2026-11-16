# cgp-field

[Context-Generic Programming (CGP)](https://contextgeneric.dev/) extends Rust with pluggable trait
implementations, chosen at compile time. `cgp-field` provides CGP's field access: the traits that
read and build a type's fields by name, the tags that name them, and the structural casts between
records and between enums. `#[derive(HasField)]` gives each field a tag, and the field can then be
read by name:

```rust,ignore
use cgp::core::field::traits::HasField;
use cgp::prelude::*;

#[derive(HasField)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

fn main() {
    let person = Person { name: "Alice".to_owned(), age: 30 };
    assert_eq!(person.get_field(PhantomData::<Symbol!("age")>), &30);
}
```

Its main items are
[`HasField`](https://contextgeneric.dev/docs/reference/traits/field-access/has_field),
[`Index`](https://contextgeneric.dev/docs/reference/types/index_type),
[`MRef`](https://contextgeneric.dev/docs/reference/types/mref), and the
[casts](https://contextgeneric.dev/docs/reference/traits/casting/), each linked to its reference
page on the website.

The `cgp` crate re-exports it as `cgp::core::field`, so a project depends on
[`cgp`](https://crates.io/crates/cgp) alone, as the example does.

The [CGP website](https://contextgeneric.dev/) teaches CGP from a first program to the reference for
each construct, and the [`cgp` crate's documentation](https://docs.rs/cgp) covers its API.
