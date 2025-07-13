pub struct Pure<T>(pub T);

pub trait ContainsValue<T> {
    type Value;
}

pub trait MonadicTrans<T> {
    type M;
}

pub trait MonadicBind<T, Next>: ContainsValue<T> {
    type Output;

    fn bind(wrapped: T, cont: impl Fn(Self::Value) -> Next) -> Self::Output;
}
