use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::error::ReturnError;
use cgp::prelude::*;

pub struct MyComponents;

#[cgp_component(FooProvider)]
#[use_namespace(DefaultNamespace: app.MyComponents)]
pub trait CanDoFoo {
    fn foo(&self);
}

pub struct App;

delegate_components! {
    #[use_namespace]
    App {
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
