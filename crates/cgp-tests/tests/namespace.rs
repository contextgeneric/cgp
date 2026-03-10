use cgp::core::component::RedirectLookup;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, LookupGenerics};
use cgp::extra::error::RaiseFrom;
use cgp::extra::handler::CanTryCompute;
use cgp::prelude::*;
use cgp_tests::{ExtendedNamespace, MyErrorComponents};

pub struct App;

impl<Component, Path> DelegateComponent<Component> for App
where
    Component: ExtendedNamespace<App, Path = Path>,
    App: DelegateComponent<Path>,
{
    // type Delegate = Delegate;
    type Delegate = RedirectLookup<Path, App>;
}

impl<Component, Path, Context, Params> IsProviderFor<Component, Context, Params> for App
where
    Component: ExtendedNamespace<App, Path = Path>,
    RedirectLookup<Path, App>: IsProviderFor<Component, Context, Params>,
    App: DelegateComponent<Path>,
{
}

delegate_components! {
    App {
        // <Component: ExtendedNamespace<App>>
        //     Component:
        //         RedirectLookup<Component::Path, App>,
        // ErrorTypeProviderComponent:
        //     UseType<String>,
        // ErrorRaiserComponent:
        //     RaiseFrom,
        Product![MyErrorComponents, ErrorTypeProviderComponent]:
            UseType<String>,
        Product![MyErrorComponents, ErrorRaiserComponent]:
            LookupGenerics<Product![MyErrorComponents, ErrorRaiserComponent], App>,
        Product![MyErrorComponents, ErrorRaiserComponent, &'static str]:
            RaiseFrom,
        TryComputerComponent:
            Foo,
    }
}

#[cgp_computer]
fn foo(x: u64) -> Result<u64, String> {
    Ok(x * 2)
}

pub trait CheckApp: HasErrorType + CanRaiseError<&'static str> + CanTryCompute<(), u64> {}

impl CheckApp for App {}
