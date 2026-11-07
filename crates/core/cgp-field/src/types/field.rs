use core::fmt::Debug;
use core::marker::PhantomData;

/// A value paired with its type-level name tag, so a struct's fields and an enum's variants can be
/// described one named entry at a time.
///
/// See [`Field`](https://contextgeneric.dev/docs/reference/types/field).
pub struct Field<Tag, Value> {
    /// The field's value.
    pub value: Value,
    /// The field's type-level tag.
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

impl<Tag, Value> Eq for Field<Tag, Value> where Value: Eq {}
