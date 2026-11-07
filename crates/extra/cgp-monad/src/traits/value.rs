/// Names the value a monad contains.
///
/// See [`ContainsValue`](https://contextgeneric.dev/docs/reference/traits/monad/contains_value).
pub trait ContainsValue<Output> {
    /// The contained value.
    type Value;
}
