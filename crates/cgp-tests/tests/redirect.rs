use cgp::prelude::*;

#[cgp_component(FooProvider)]
pub trait CanDoFoo {
    fn foo();
}

pub struct UseNamespace<Components>(pub PhantomData<Components>);

pub struct RedirectLookup<Key, Components>(pub PhantomData<(Key, Components)>);

#[cgp_impl(RedirectLookup<Key, Components>)]
#[use_provider(Components::Delegate: FooProvider)]
impl<Key, Components> FooProvider
where
    Components: DelegateComponent<Key>,
{
    fn foo() {
        Components::Delegate::foo();
    }
}

delegate_components! {
    <Components> UseNamespace<Components> {
        FooProviderComponent: RedirectLookup<BarComponent, Components>,
    }
}

pub struct BarComponent;

#[cgp_impl(new TestProvider)]
impl FooProvider {
    fn foo() {}
}

pub struct App;

delegate_components! {
    App {
        FooProviderComponent: UseNamespace<App>,
        BarComponent: TestProvider,
    }
}

delegate_components! {
    new InnerComponents {
        BarComponent: TestProvider,
    }
}

check_components! {
    App {
        FooProviderComponent,
    }
}
