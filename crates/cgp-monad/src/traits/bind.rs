pub trait MonadicLift<Value, Output> {
    type Output;

    fn lift_value(value: Value) -> Self::Output;

    fn lift_output(output: Output) -> Self::Output;
}
