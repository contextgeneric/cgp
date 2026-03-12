use cgp::core::component::{CgpCore, RedirectLookup};
use cgp::core::error::{ErrorComponents, ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;

pub trait ExtendedNamespace<T> {
    type Provider;
}

pub struct ExtendedNamespaceComponents;

pub struct MyErrorComponents;

impl<Component, Components, Provider> ExtendedNamespace<Components> for Component
where
    Component:
        CgpNamespace<Components, Provider = Provider> + CgpNamespace<ExtendedNamespaceComponents>,
{
    type Provider = Provider;
}

impl<Components> ExtendedNamespace<Components> for ErrorRaiserComponent {
    type Provider = RedirectLookup<
        Components,
        PathCons<MyErrorComponents, PathCons<ErrorRaiserComponent, PathNil>>,
    >;
}

impl<Components> ExtendedNamespace<Components>
    for PathCons<CgpCore, PathCons<ErrorComponents, PathCons<ErrorTypeProviderComponent, PathNil>>>
{
    type Provider = RedirectLookup<
        Components,
        PathCons<MyErrorComponents, PathCons<ErrorTypeProviderComponent, PathNil>>,
    >;
}
