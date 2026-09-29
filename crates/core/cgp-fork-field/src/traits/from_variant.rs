use core::marker::PhantomData;

pub trait FromVariant<Tag> {
    type Value;

    fn from_variant(_tag: PhantomData<Tag>, value: Self::Value) -> Self;
}
