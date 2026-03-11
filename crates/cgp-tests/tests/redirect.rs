use cgp::prelude::*;

pub trait HasNamespace<T> {}

pub struct UseNamespace<Components>(pub PhantomData<Components>);

#[cgp_component(FooProvider)]
pub trait CanDoFoo {
    fn foo();
}

impl<T> HasNamespace<T> for FooProviderComponent {}

delegate_components! {
    <Components> UseNamespace<Components> {
        FooProviderComponent: RedirectLookup<Components, Product![BarComponent, BazComponent, FooProviderComponent]>,
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
            UseNamespace<App>,

        // @BarComponent.* : TestProvider,
        // <Components> Cons<BarComponent, Components>: TestProvider,

        // @BarComponent.BazComponent.* : TestProvider,
        // <Components> Cons<BarComponent, Cons<BazComponent, Components>>: TestProvider,

        // @BarComponent.BazComponent.FooProviderComponent : TestProvider,
        Product![BarComponent, BazComponent, FooProviderComponent]: TestProvider,

        // @*.BazComponent.*: TestProvider,
        // <ComponentsA, ComponentsB> Cons<ComponentsA, Cons<BazComponent, ComponentsB>>: TestProvider,
    }
}

check_components! {
    App {
        FooProviderComponent,
    }
}
