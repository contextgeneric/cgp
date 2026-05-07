use cgp::prelude::*;

#[cgp_component(FooProvider)]
pub trait Foo {
    fn foo(&self);
}

cgp_namespace! {
    MyNamespace {
        FooProviderComponent:
            @MyFooComponent,
    }
}

#[cgp_component(BarProvider)]
#[namespace(MyNamespace: @MyBarComponent)]
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

        @MyFooComponent:
            DummyFoo,
        @MyBarComponent:
            DummyBar,
    }
}

check_components! {
    App {
        FooProviderComponent,
        BarProviderComponent,
    }
}
