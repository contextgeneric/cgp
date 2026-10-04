use syn::{ItemImpl, ItemTrait};

use crate::types::cgp_computer::ItemCgpComputer;

/// The intermediate representation `#[cgp_auto_dispatch]` evaluates a trait
/// into. Each entry of `computers` is what the macro would otherwise emit as a
/// `#[cgp_computer(Compute{Method})]` helper function; the entrypoint runs it
/// through the `#[cgp_computer]` pipeline instead.
pub struct EvaluatedCgpAutoDispatch {
    /// The annotated trait, emitted unchanged.
    pub item_trait: ItemTrait,
    /// The impl of the trait for any extensible enum.
    pub blanket_impl: ItemImpl,
    /// One per-variant computer per method, in method order.
    pub computers: Vec<ItemCgpComputer>,
}
