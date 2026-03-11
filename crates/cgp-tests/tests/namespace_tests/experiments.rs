use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::error::RaiseFrom;
use cgp::extra::handler::CanTryCompute;
use cgp::prelude::*;
use cgp_tests::{ExtendedNamespace, MyErrorComponents};

pub struct App;

delegate_components! {
    App {
        <Component: ExtendedNamespace<App>>
            Component: Component::Provider,
        Product![MyErrorComponents, ErrorTypeProviderComponent]:
            UseType<String>,
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
