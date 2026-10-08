use crate::core::field::traits::{HasFields, HasFieldsRef};

pub trait ToFields: HasFields {
    fn to_fields(self) -> Self::Fields;
}

pub trait ToFieldsRef: HasFieldsRef {
    fn to_fields_ref<'a>(&'a self) -> Self::FieldsRef<'a>
    where
        Self: 'a;
}
