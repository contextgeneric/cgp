pub trait Refl {
    type Type;
}

impl<T> Refl for T {
    type Type = T;
}
