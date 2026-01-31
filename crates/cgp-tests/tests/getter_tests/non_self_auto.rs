use cgp::prelude::*;

#[cgp_type]
pub trait HasFooType {
    type Foo;
}

#[cgp_type]
pub trait HasBarType {
    type Bar;
}

#[cgp_auto_getter]
pub trait HasFooBar: HasFooType + HasBarType {
    fn foo_bar(foo: &Self::Foo) -> &Self::Bar;
}

#[cgp_context]
pub struct App;

#[derive(HasField)]
pub struct Foo {
    pub foo_bar: u32,
}

delegate_components! {
    AppComponents {
        FooTypeProviderComponent:
            UseType<Foo>,
        BarTypeProviderComponent:
            UseType<u32>,
    }
}

#[test]
fn test_non_self_getter() {
    let foo = Foo { foo_bar: 42 };

    let bar = App::foo_bar(&foo);
    assert_eq!(bar, &42);
}
