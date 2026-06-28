use crate::DelegateComponent;

pub trait IsDelegateKeyIn<Table> {}

impl<Table, Key> IsDelegateKeyIn<Table> for Key where Table: DelegateComponent<Key> {}
