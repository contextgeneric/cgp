/// The monad trait that puts a value into a step's output on either branch: lift_value for a
/// short-circuit, lift_output for a continuation's result.
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
