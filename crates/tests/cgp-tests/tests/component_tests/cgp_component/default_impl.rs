use cgp::prelude::*;

#[cgp_getter]
pub trait HasName {
    fn name(&self) -> &str {
        "John"
    }
}

#[cgp_component(Greeter)]
pub trait CanGreet: HasName {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}

pub struct UseDefault;

#[cgp_impl(UseDefault)]
impl<Context> NameGetter for Context {}

#[cgp_impl(UseDefault)]
impl<Context: HasName> Greeter for Context {}

#[test]
fn test_default_method_impl() {
    pub struct App;

    delegate_components! {
        App {
            [
                NameGetterComponent,
                GreeterComponent,
            ]:
                UseDefault,
        }
    }

    assert_eq!(App.greet(), "Hello, John!");
}
