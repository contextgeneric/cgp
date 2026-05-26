use syn::{ItemImpl, Type};

pub struct ItemCgpProvider {
    pub component_type: Option<Type>,
    pub item_impl: ItemImpl,
}
