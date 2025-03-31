use core::fmt::Debug;
use core::marker::PhantomData;

pub struct ω<Tag, Value> {
    pub value: Value,
    pub phantom: PhantomData<Tag>,
}

pub use ω as Field;

impl<Tag, Value> From<Value> for Field<Tag, Value> {
    fn from(value: Value) -> Self {
        Self {
            value,
            phantom: PhantomData,
        }
    }
}

impl<Tag, Value> Debug for Field<Tag, Value>
where
    Value: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl<Tag, Value> PartialEq for Field<Tag, Value>
where
    Value: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl<Tag, Value> Eq for Field<Tag, Value>
where
    Value: Eq,
{
    fn assert_receiver_is_total_eq(&self) {
        self.value.assert_receiver_is_total_eq()
    }
}
