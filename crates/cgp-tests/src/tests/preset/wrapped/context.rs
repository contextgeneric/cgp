use core::convert::Infallible;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;

use crate::tests::preset::wrapped::preset::{BoxError, ErrorHandlerPreset};

#[cgp_context]
pub struct MyContext;

delegate_components! {
    MyContextComponents {
        ErrorTypeProviderComponent:
            UseType<BoxError>,
        ErrorRaiserComponent:
            ErrorHandlerPreset::Provider,
    }
}

check_components! {
    CanUseMyContext for MyContext {
        ErrorRaiserComponent: [
            BoxError,
            Infallible,
            std::io::Error,
        ]
    }
}
