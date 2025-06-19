use core::marker::PhantomData;

pub trait TakeField<Tag> {
    type Value;

    type Remainder;

    fn take_field(self, _tag: PhantomData<Tag>) -> (Self::Value, Self::Remainder);
}
