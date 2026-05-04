use cgp::core::component::RedirectLookup;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::prelude::*;

cgp_namespace! {
    ExtendedNamespace: DefaultNamespace {
        @cgp.core.error.ErrorRaiserComponent:
            @app.ErrorRaiserComponent,
        @cgp.core.error.ErrorTypeProviderComponent:
            @app.ErrorTypeProviderComponent,
    }
}
