use cgp::prelude::*;

#[cgp_component(FooProvider)]
#[use_namespace(@app.FooProviderComponent)]
pub trait Foo<'a, T, U> {
    fn foo(&self, first: &'a T, second: U);
}

#[cgp_impl(new DummyFoo)]
impl<'a, T, U> FooProvider<'a, T, U> {
    fn foo(&self, _first: &'a T, _second: U) {}
}

pub struct AppA;

delegate_components! {
    AppA {
        open FooProviderComponent;

        @FooProviderComponent.String.u32:
            DummyFoo,
    }
}

check_components! {
    <'a> AppA {
        FooProviderComponent: [
            (Life<'a>, String, u32),
        ],
    }
}

pub struct AppB;

delegate_components! {
    AppB {
        namespace default;

        @app.FooProviderComponent.String.u64:
            DummyFoo,
    }
}

check_components! {
    <'a> AppB {
        FooProviderComponent: [
            (Life<'a>, String, u64),
        ],
    }
}
