#[cfg(test)]
pub mod tests;

use cgp::core::component::{CoreComponents, RedirectLookup};
use cgp::core::error::{ErrorComponents, ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;

pub trait ExtendedNamespace<T> {
    type Provider;
}

pub struct ExtendedNamespaceComponents;

pub struct MyErrorComponents;

impl<Component, Components, Provider> ExtendedNamespace<Components> for Component
where
    Component: DefaultNamespace<Components, Provider = Provider>
        + DefaultNamespace<ExtendedNamespaceComponents>,
{
    type Provider = Provider;
}

impl<Components> ExtendedNamespace<Components> for ErrorRaiserComponent {
    type Provider = RedirectLookup<Components, Product![MyErrorComponents, ErrorRaiserComponent]>;
}

impl<Components> ExtendedNamespace<Components>
    for Product![CoreComponents, ErrorComponents, ErrorTypeProviderComponent]
{
    type Provider =
        RedirectLookup<Components, Product![MyErrorComponents, ErrorTypeProviderComponent]>;
}
