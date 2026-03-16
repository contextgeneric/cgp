use cgp::core::component::RedirectLookup;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;

pub trait ExtendedNamespace<T> {
    type Provider;
}

pub struct ExtendedNamespaceComponents;

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
        PathCons<Symbol!("app"), PathCons<ErrorRaiserComponent, PathNil>>,
    >;
}

impl<Components> ExtendedNamespace<Components>
    for PathCons<
        Symbol!("cgp"),
        PathCons<
            Symbol!("core"),
            PathCons<Symbol!("error"), PathCons<ErrorTypeProviderComponent, PathNil>>,
        >,
    >
{
    type Provider = RedirectLookup<
        Components,
        PathCons<Symbol!("app"), PathCons<ErrorTypeProviderComponent, PathNil>>,
    >;
}
