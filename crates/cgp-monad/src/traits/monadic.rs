use cgp_core::field::MapType;

pub trait Monadic<T> {
    type Value;

    type Monad: MapType<Map<Self::Value> = T>;
}
