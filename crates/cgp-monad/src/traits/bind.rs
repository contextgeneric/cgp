use cgp_core::field::MapType;

pub trait CanBindValue: MapType {
    fn bind<T, U>(value: Self::Map<T>, cont: impl Fn(T) -> Self::Map<U>) -> Self::Map<U>;
}
