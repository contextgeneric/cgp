use core::convert::Infallible;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_check_components, snapshot_delegate_components};

use crate::preset_tests::wrapped::preset::{BoxError, ErrorHandlerPreset};

pub struct MyContext;

snapshot_delegate_components! {
    delegate_components! {
        MyContext {
            ErrorTypeProviderComponent:
                UseType<BoxError>,
            ErrorRaiserComponent:
                ErrorHandlerPreset::Provider,
        }
    }

    expand_my_context(output) {
        insta::assert_snapshot!(output, @"
        impl DelegateComponent<ErrorTypeProviderComponent> for MyContext {
            type Delegate = UseType<BoxError>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<ErrorTypeProviderComponent, __Context__, __Params__> for MyContext
        where
            UseType<
                BoxError,
            >: IsProviderFor<ErrorTypeProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<ErrorRaiserComponent> for MyContext {
            type Delegate = ErrorHandlerPreset::Provider;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<ErrorRaiserComponent, __Context__, __Params__> for MyContext
        where
            ErrorHandlerPreset::Provider: IsProviderFor<
                ErrorRaiserComponent,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

snapshot_check_components! {
    check_components! {
        MyContext {
            ErrorRaiserComponent: [
                BoxError,
                Infallible,
                std::io::Error,
            ]
        }
    }

    expand_check_my_context(output) {
        insta::assert_snapshot!(output, @"
        trait __CheckMyContext<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CheckMyContext<ErrorRaiserComponent, BoxError> for MyContext {}
        impl __CheckMyContext<ErrorRaiserComponent, Infallible> for MyContext {}
        impl __CheckMyContext<ErrorRaiserComponent, std::io::Error> for MyContext {}
        ")
    }
}
