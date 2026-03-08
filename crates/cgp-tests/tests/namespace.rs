use cgp::core::component::{CoreComponents, RedirectLookup};
use cgp::core::error::{ErrorComponents, ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::traits::AppendProduct;
use cgp::extra::error::RaiseFrom;
use cgp::extra::handler::CanTryCompute;
use cgp::prelude::*;
use cgp_tests::ExtendedNamespace;

pub struct App;

delegate_components! {
    App {
        <Path, Component: ExtendedNamespace<App, Path: AppendProduct<Component, Output = Path>>>
            Component:
                RedirectLookup<Path, App>,
        Product![CoreComponents, ErrorComponents, ErrorTypeProviderComponent]:
            UseType<String>,
        Product![CoreComponents, ErrorComponents, ErrorRaiserComponent]:
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
