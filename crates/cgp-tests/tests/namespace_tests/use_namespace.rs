use cgp::core::component::{CgpCore, DefaultComponentsNamespace};
use cgp::core::error::ErrorTypeProviderComponent;
use cgp::prelude::*;

pub struct MyComponents;

#[cgp_component(FooProvider)]
#[use_namespace(CgpNamespace: CgpCore.MyComponents)]
pub trait CanDoFoo {
    fn foo(&self);
}

pub struct App;

delegate_components! {
    App {
        <Component: DefaultComponentsNamespace<App>>
            Component: Component::Provider,
        ErrorTypeProviderComponent:
            UseType<String>,
    }
}
