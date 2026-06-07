pub trait HasFields {
    type Fields;
}

pub trait HasFieldsRef {
    type FieldsRef<'a>
    where
        Self: 'a;
}
