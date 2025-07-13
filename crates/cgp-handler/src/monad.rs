pub struct Pure<T>(pub T);

pub trait ContainsValue<T> {
    type Value;
}

pub trait MonadicTrans<T> {
    type M;
}

pub trait CanWrap<T>: ContainsValue<T> {
    fn wrap(value: Self::Value) -> T;
}

pub trait Functorial<T, Next>: ContainsValue<T> {
    type Output;

    fn map(wrapped: T, cont: impl Fn(Self::Value) -> Next) -> Self::Output;
}

pub trait MonadicBind<T, Next>: ContainsValue<T> {
    type Output;

    fn bind(wrapped: T, cont: impl Fn(Self::Value) -> Next) -> Self::Output;
}
