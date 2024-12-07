#![no_std]

extern crate alloc;

use cgp_component::{cgp_component, DelegateComponent, HasComponents};

#[cgp_component(InnerComponent, ProvideInner<Context>)]
pub trait HasInner {
    type Inner;

    fn inner(&self) -> &Self::Inner;
}
