use core::convert::Infallible;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_components;
use insta::assert_snapshot;

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
        assert_snapshot!(output, @"
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

check_components! {
    MyContext {
        ErrorRaiserComponent: [
            BoxError,
            Infallible,
            std::io::Error,
        ]
    }
}
