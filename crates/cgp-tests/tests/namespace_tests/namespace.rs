use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::error::ReturnError;
use cgp::prelude::*;

pub struct MyComponents;

#[cgp_component(FooProvider)]
#[namespace(DefaultNamespace: @app.MyComponents.FooProviderComponent)]
pub trait Foo {
    fn foo(&self);
}

pub struct App;

delegate_components! {
    App {
        namespace default;

        @cgp.core.error.ErrorTypeProviderComponent:
            UseType<String>,
        @cgp.core.error.ErrorRaiserComponent.String:
            ReturnError,
    }
}

check_components! {
    App {
        ErrorRaiserComponent: String,
    }
}
