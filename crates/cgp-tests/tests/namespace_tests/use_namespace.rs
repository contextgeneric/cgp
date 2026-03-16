use cgp::core::error::ErrorTypeProviderComponent;
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
    }
}
