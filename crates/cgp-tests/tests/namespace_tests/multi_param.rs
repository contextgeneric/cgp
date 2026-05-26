use cgp::prelude::*;

#[cgp_component(FooProvider)]
#[prefix(@app)]
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
        open {FooProviderComponent};

        @FooProviderComponent.String.u32:
            DummyFoo,
        <T> @FooProviderComponent.bool.T:
            DummyFoo,
    }
}

check_components! {
    AppA {
        FooProviderComponent: [
            <'a> (Life<'a>, String, u32),
            <'a> (Life<'a>, bool, String),
        ],
        FooProviderComponent:
            <'a> (Life<'a>, bool, bool),
    }
}

pub struct AppB;

delegate_components! {
    AppB {
        namespace DefaultNamespace;

        @app.FooProviderComponent.String.u64:
            DummyFoo,
        @app.FooProviderComponent.bool.<T> T:
            DummyFoo,
    }
}

check_components! {
    <'a> AppB {
        FooProviderComponent: [
            (Life<'a>, String, u64),
            (Life<'a>, bool, String),
        ],
    }
}
