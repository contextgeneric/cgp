use core::marker::PhantomData;

pub trait HasField<Key> {
    type Field;

    fn field(key: PhantomData<Key>) -> Self::Field;
}
