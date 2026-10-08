use crate::core::field::traits::HasFields;

pub trait FromFields: HasFields {
    fn from_fields(fields: Self::Fields) -> Self;
}
