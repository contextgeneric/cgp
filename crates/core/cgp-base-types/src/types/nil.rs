/// The end marker of the product, string, and path lists: an empty, constructible list that is also
/// the shape of a unit struct.
///
/// See [`Nil`](https://contextgeneric.dev/docs/reference/types/nil).
#[derive(Eq, PartialEq, Clone, Default, Debug)]
pub struct Nil;
