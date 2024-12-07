#![no_std]

extern crate alloc;

use cgp_component::{derive_component, DelegateComponent, HasComponents};

#[derive_component(InnerComponent, ProvideInner<Context>)]
pub trait HasInner {
    type Inner;

    fn inner(&self) -> &Self::Inner;
}
