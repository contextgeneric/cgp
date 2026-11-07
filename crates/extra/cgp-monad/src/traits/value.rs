/// The monad trait naming the value beneath a step's output that the monad threads forward: the Ok
/// payload under ErrMonadic, the error under OkMonadic.
///
/// See [`ContainsValue`](https://contextgeneric.dev/docs/reference/traits/monad/contains_value).
pub trait ContainsValue<Output> {
    /// The contained value.
    type Value;
}
