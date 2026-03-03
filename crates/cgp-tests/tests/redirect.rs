use cgp::prelude::*;

pub trait HasNamespace<T> {}

pub struct UseNamespace<Path, Components>(pub PhantomData<(Path, Components)>);

pub struct RedirectLookup<Key, Components>(pub PhantomData<(Key, Components)>);

#[cgp_component(FooProvider)]
pub trait CanDoFoo {
    fn foo();
}

impl<T> HasNamespace<T> for FooProviderComponent {}

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
    <Components> UseNamespace<Nil, Components> {
        FooProviderComponent: RedirectLookup<Product![BarComponent, BazComponent, FooProviderComponent], Components>,
    }
}

pub struct BarComponent;

pub struct BazComponent;

#[cgp_impl(new TestProvider)]
impl FooProvider {
    fn foo() {}
}

pub struct App;

delegate_components! {
    // #[use_namespace]
    App {
        <Component: HasNamespace<App>> Component:
            UseNamespace<Nil, App>,

        // @BarComponent::* : TestProvider,
        // <Components> Cons<BarComponent, Components>: TestProvider,

        // @BarComponent::BazComponent::* : TestProvider,
        <Components> Cons<BarComponent, Cons<BazComponent, Components>>: TestProvider,

        // @BarComponent::BazComponent::FooProviderComponent : TestProvider,
        // Product![BarComponent, BazComponent, FooProviderComponent]: TestProvider,
    }
}

check_components! {
    App {
        FooProviderComponent,
    }
}
