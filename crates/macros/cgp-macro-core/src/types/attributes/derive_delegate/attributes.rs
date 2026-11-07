use crate::types::attributes::DeriveDelegateAttribute;

/// The `#[derive_delegate]` tables of a component.
#[derive(Default, Clone)]
pub struct DeriveDelegateAttributes {
    /// The tables.
    pub attributes: Vec<DeriveDelegateAttribute>,
}
