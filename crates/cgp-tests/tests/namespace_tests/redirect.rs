use cgp::prelude::*;

#[cgp_component(FooProvider)]
#[namespace(@bar.baz.FooProviderComponent)]
pub trait CanDoFoo {
    fn foo();
}

pub struct BarComponent;

pub struct BazComponent;

#[cgp_impl(new TestProvider)]
impl FooProvider {
    fn foo() {}
}

pub struct App;

delegate_components! {
    App {
        namespace default;

        // @bar: TestProvider,

        @bar.baz: TestProvider,

        // @bar.baz.FooProviderComponent: TestProvider,
    }
}

check_components! {
    App {
        FooProviderComponent,
    }
}
