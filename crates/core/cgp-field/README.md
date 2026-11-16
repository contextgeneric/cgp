# cgp-field

A [Context-Generic Programming (CGP)](https://contextgeneric.dev/) provider reads the values it
needs from its context's fields by name, so it fits any context that has those fields. `cgp-field`
provides that field access: the traits that read and build a type's fields by name, the tags that
name them, and the structural casts between records and between enums. `#[derive(HasField)]` gives
each field a tag, and the field can then be read by name:

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

Projects reach this field access through the `cgp` crate, which re-exports it as `cgp::core::field`,
so they depend on [`cgp`](https://crates.io/crates/cgp) alone, as the example does. The website's
reference has a page for each of its main items:
[`HasField`](https://contextgeneric.dev/docs/reference/traits/field-access/has_field),
[`Index`](https://contextgeneric.dev/docs/reference/types/index_type),
[`MRef`](https://contextgeneric.dev/docs/reference/types/mref), and the
[casts](https://contextgeneric.dev/docs/reference/traits/casting/).

To learn CGP itself, the [CGP website](https://contextgeneric.dev/) starts from a first program, and
the [`cgp` crate's documentation](https://docs.rs/cgp) covers the whole API.
