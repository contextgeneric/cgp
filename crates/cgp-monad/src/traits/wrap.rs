use cgp_core::field::MapType;

pub trait CanWrapValue: MapType {
    fn wrap<T>(value: T) -> Self::Map<T>;
}
