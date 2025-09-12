use cgp::core::field::types::MRef;
use cgp::prelude::*;

#[test]
pub fn test_mref_getter() {
    #[cgp_getter]
    pub trait HasFoo {
        fn foo(&self) -> MRef<'_, String>;
    }

    #[cgp_context]
    #[derive(HasField)]
    pub struct App {
        pub bar: String,
    }

    delegate_components! {
        AppComponents {
            FooGetterComponent: UseField<Symbol!("bar")>,
        }
    }

    let context = App { bar: "foo".into() };

    assert_eq!(context.foo().as_ref(), "foo");
}

#[test]
pub fn test_mref_auto_getter() {
    #[cgp_auto_getter]
    pub trait HasFoo {
        fn foo(&self) -> MRef<'_, String>;
    }

    #[derive(HasField)]
    pub struct App {
        pub foo: String,
    }

    let context = App { foo: "foo".into() };

    assert_eq!(context.foo().as_ref(), "foo");
}
