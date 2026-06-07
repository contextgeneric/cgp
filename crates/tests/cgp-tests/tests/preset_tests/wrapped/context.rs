use core::convert::Infallible;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;

use crate::preset_tests::wrapped::preset::{BoxError, ErrorHandlerPreset};

pub struct MyContext;

delegate_components! {
    MyContext {
        ErrorTypeProviderComponent:
            UseType<BoxError>,
        ErrorRaiserComponent:
            ErrorHandlerPreset::Provider,
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
