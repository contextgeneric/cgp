use core::marker::PhantomData;

pub struct Field<Tag, Value> {
    pub value: Value,
    pub phantom: PhantomData<Tag>,
}

impl<Tag, Value> From<Value> for Field<Tag, Value> {
    fn from(value: Value) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}
