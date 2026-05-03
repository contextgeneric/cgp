use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::error::ReturnError;
use cgp::prelude::*;

pub struct App;

delegate_components! {
    App {
        ErrorTypeProviderComponent:
            UseType<String>,
        ErrorRaiserComponent:
            RedirectLookup<App, PathCons<ErrorRaiserComponent, PathNil>>,
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
