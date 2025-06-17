use core::marker::PhantomData;

pub trait BuildField<Tag> {
    type Value;

    type Output;

    fn build_field(self, _tag: PhantomData<Tag>, value: Self::Value) -> Self::Output;
}
