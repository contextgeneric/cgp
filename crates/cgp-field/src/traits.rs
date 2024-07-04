use core::marker::PhantomData;
use core::ops::Deref;

pub trait HasField<Key> {
    type Field;

    fn get_field(&self, key: PhantomData<Key>) -> &Self::Field;
}

impl<Context, Key, Target, Field> HasField<Key> for Context
where
    Context: Deref<Target = Target>,
    Target: HasField<Key, Field = Field> + 'static,
{
    type Field = Field;

    fn get_field(&self, key: PhantomData<Key>) -> &Self::Field {
        self.deref().get_field(key)
    }
}
