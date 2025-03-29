use crate::HasFields;

pub trait FromFields: HasFields {
    fn from_fields(fields: Self::Fields) -> Self;
}
