use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::error::ReturnError;
use cgp::prelude::*;

pub struct MyComponents;

#[cgp_component(FooProvider)]
#[prefix(@app.MyComponents.FooProviderComponent in DefaultNamespace)]
pub trait Foo {
    fn foo(&self);
}

pub struct App;

delegate_components! {
    App {
        namespace DefaultNamespace;

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
