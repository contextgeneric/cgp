use cgp::prelude::*;

pub struct MyApp;

#[cgp_component(FooProvider)]
pub trait Foo {
    fn foo(&self);
}

cgp_namespace! {
    MyNamespace {
        FooProviderComponent:
            @MyApp.MyFooComponent,
    }
}

cgp_namespace! {
    OtherNamespace {
        FooProviderComponent:
            @my_app.MyFooComponent,
    }
}

#[cgp_component(BarProvider)]
#[use_namespace(MyNamespace: @MyApp.MyBarComponent)]
#[use_namespace(OtherNamespace: @my_app.MyBarComponent)]
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
    #[use_namespace(MyNamespace)]
    App {
        @MyApp.MyFooComponent:
            DummyFoo,
        @MyApp.MyBarComponent:
            DummyBar,
    }
}

check_components! {
    App {
        FooProviderComponent,
        BarProviderComponent,
    }
}

pub struct OtherApp;

delegate_components! {
    #[use_namespace(OtherNamespace)]
    OtherApp {
        @my_app.MyFooComponent:
            DummyFoo,
        @my_app.MyBarComponent:
            DummyBar,
    }
}

check_components! {
    OtherApp {
        FooProviderComponent,
        BarProviderComponent,
    }
}
