use crate::types::attributes::DeriveDelegateAttribute;

#[derive(Default, Clone)]
/// The `#[derive_delegate]` tables of a component.
pub struct DeriveDelegateAttributes {
    /// The tables.
    pub attributes: Vec<DeriveDelegateAttribute>,
}
