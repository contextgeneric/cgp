use core::marker::PhantomData;

pub trait HasField<Key> {
    type Field;

    fn get_field(&self, key: PhantomData<Key>) -> &Self::Field;
}
