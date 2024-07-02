pub mod delegate_components;
pub mod derive_component;

#[cfg(test)]
mod tests;

pub use crate::delegate_components::{define_components, delegate_components};
pub use crate::derive_component::derive_component;
