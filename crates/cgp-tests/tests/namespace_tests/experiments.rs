use cgp::core::component::{DelegateComponent1, UseDefault};
use cgp::core::error::{ErrorRaiserComponent, ErrorRaiserComponents, ErrorTypeProviderComponent};
use cgp::extra::error::RaiseFrom;
use cgp::extra::handler::CanTryCompute;
use cgp::prelude::*;
use cgp_tests::namespaces::{ExtendedNamespace, MyErrorComponents};

pub struct App;

delegate_components! {
    App {
        <Component: ExtendedNamespace<App>>
            Component: Component::Provider,
        @MyErrorComponents.ErrorTypeProviderComponent:
            UseType<String>,
        @MyErrorComponents.ErrorRaiserComponent.&'static str:
            RaiseFrom,
        @MyErrorComponents.ErrorRaiserComponent.MyError:
            RaiseFrom,
        TryComputerComponent:
            Foo,
    }
}

#[cgp_computer]
fn foo(x: u64) -> Result<u64, String> {
    Ok(x * 2)
}

pub trait CheckApp:
    HasErrorType + CanRaiseError<&'static str> + CanRaiseError<MyError> + CanTryCompute<(), u64>
{
}

impl CheckApp for App {}

pub struct MyError;

impl From<MyError> for String {
    fn from(_: MyError) -> Self {
        "MyError".to_string()
    }
}

delegate_components! {
    ErrorRaiserComponents<UseDefault> {
        MyError:
            RaiseFrom,
    }
}

impl DelegateComponent1<ErrorRaiserComponent, MyError> for UseDefault {
    type Delegate = RaiseFrom;
}

pub struct MyDefault;

impl DelegateComponent1<ErrorRaiserComponent, String> for MyDefault {
    type Delegate = RaiseFrom;
}

// delegate_components! {
//     ErrorRaiserComponents<MyDefault> {
//         &'static str:
//             RaiseFrom,
//     }
// }
