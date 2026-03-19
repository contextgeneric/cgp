use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::extra::error::RaiseFrom;
use cgp::extra::handler::CanTryCompute;
use cgp::prelude::*;
use cgp_tests::namespaces::ExtendedNamespace;

pub struct App;

delegate_components! {
    #[use_namespace(ExtendedNamespace)]
    App {
        @app.ErrorTypeProviderComponent:
            UseType<String>,
        @app.{
            ErrorRaiserComponent.{&'static str, String},
            ErrorWrapperComponent.*,
        }:
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
