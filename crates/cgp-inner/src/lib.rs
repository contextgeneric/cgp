#![no_std]

extern crate alloc;

use cgp_component::{DelegateComponent, HasComponents};
use cgp_component_macro::cgp_component;

#[cgp_component {
    name: InnerComponent,
    provider: ProvideInner,
}]
pub trait HasInner {
    type Inner;

    fn inner(&self) -> &Self::Inner;
}
