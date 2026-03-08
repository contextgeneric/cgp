#[cfg(test)]
pub mod tests;

use cgp::core::component::CoreComponents;
use cgp::core::error::{ErrorComponents, ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;

pub trait ExtendedNamespace<T> {
    type Path;
}

pub struct ExtendedNamespaceComponents;

pub struct MyErrorComponents;

impl<Component, T, Path> ExtendedNamespace<T> for Component
where
    Component: DefaultNamespace<T, Path = Path>
        + DefaultNamespace<ExtendedNamespaceComponents, Path = Path>,
{
    type Path = Path;
}

impl<T> ExtendedNamespace<T> for ErrorRaiserComponent {
    type Path = Product![MyErrorComponents, ErrorRaiserComponent];
}

impl<T> ExtendedNamespace<T>
    for Product![CoreComponents, ErrorComponents, ErrorTypeProviderComponent]
{
    type Path = Product![MyErrorComponents, ErrorTypeProviderComponent];
}
