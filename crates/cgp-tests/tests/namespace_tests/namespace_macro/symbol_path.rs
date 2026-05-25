use cgp::prelude::*;

#[cgp_component(FooProvider)]
pub trait Foo {
    fn foo(&self);
}

cgp_namespace! {
    new MyNamespace {
        FooProviderComponent =>
            @my_app.MyFooComponent,
    }
}

#[cgp_component(BarProvider)]
#[prefix(@my_app.MyBarComponent in MyNamespace)]
pub trait Bar {
    fn bar(&self);
}

pub struct MyFooComponent;

pub struct MyBarComponent;

#[cgp_impl(new DummyFoo)]
impl FooProvider {
    fn foo(&self) {}
}

#[cgp_impl(new DummyBar)]
impl BarProvider {
    fn bar(&self) {}
}

pub struct App;

delegate_components! {
    App {
        namespace MyNamespace;

        @my_app.MyFooComponent:
            DummyFoo,
        @my_app.MyBarComponent:
            DummyBar,
    }
}

check_components! {
    App {
        FooProviderComponent,
        BarProviderComponent,
    }
}
