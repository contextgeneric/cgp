#![no_std]

use cgp_component::*;
use cgp_component_macro::cgp_component;

#[cgp_component {
    name: InnerComponent,
    provider: ProvideInner,
}]
pub trait HasInner {
    type Inner;

    fn inner(&self) -> &Self::Inner;
}
