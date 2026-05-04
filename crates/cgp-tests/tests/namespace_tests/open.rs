use cgp::prelude::*;

pub struct App;

#[cgp_component(FooProvider)]
pub trait Foo<T> {
    fn foo(&self, value: &T);
}

#[cgp_component(BarProvider)]
pub trait Bar<T> {
    fn bar(&self, value: &T);
}

#[cgp_impl(new DummyFoo)]
impl<T> FooProvider<T> {
    fn foo(&self, _value: &T) {}
}

#[cgp_impl(new DummyBar)]
impl<T> BarProvider<T> {
    fn bar(&self, _value: &T) {}
}

delegate_components! {
    App {
        open FooProviderComponent, BarProviderComponent;

        @FooProviderComponent.String:
            DummyFoo,
        @BarProviderComponent.{u32, u64, bool, usize, isize}:
            DummyBar,
    }
}

check_components! {
    App {
        FooProviderComponent:
            String,
        BarProviderComponent: [
            u32,
            u64,
            bool,
            usize,
            isize,
        ],
    }
}
