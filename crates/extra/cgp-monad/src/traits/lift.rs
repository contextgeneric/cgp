/// Lifts a value into a monad.
///
/// See [`LiftValue`](https://contextgeneric.dev/docs/reference/traits/monad/lift_value).
pub trait LiftValue<Value, Output> {
    /// The lifted output.
    type Output;

    /// Lifts a value.
    fn lift_value(value: Value) -> Self::Output;

    /// Lifts an output.
    fn lift_output(output: Output) -> Self::Output;
}
