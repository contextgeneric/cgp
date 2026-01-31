use cgp::prelude::*;

#[cgp_auto_getter]
pub trait HasFoo<Foo> {
    fn foo(&self, _tag: PhantomData<Foo>) -> &Foo;
}

#[derive(HasField)]
pub struct App {
    pub foo: u32,
}

#[test]
fn test_generic_auto_getter() {
    let app = App { foo: 42 };

    assert_eq!(app.foo(PhantomData::<u32>), &42);
}
