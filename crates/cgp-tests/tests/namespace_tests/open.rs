use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::error::ReturnError;
use cgp::prelude::*;

pub struct App;

delegate_components! {
    App {
        open ErrorRaiserComponent;

        ErrorTypeProviderComponent:
            UseType<String>,
        @ErrorRaiserComponent.String:
            ReturnError,
    }
}

check_components! {
    App {
        ErrorRaiserComponent:
            String,
    }
}
