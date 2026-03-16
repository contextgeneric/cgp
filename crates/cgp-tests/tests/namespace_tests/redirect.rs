use cgp::prelude::*;

#[cgp_component(FooProvider)]
#[use_namespace(bar.baz)]
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
    #[use_namespace]
    App {
        // @bar.*: TestProvider,

        @bar.baz.*: TestProvider,

        // @bar.baz.FooProviderComponent: TestProvider,
    }
}

check_components! {
    App {
        FooProviderComponent,
    }
}
