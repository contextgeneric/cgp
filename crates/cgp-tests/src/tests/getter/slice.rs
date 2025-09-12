use cgp::prelude::*;

#[test]
pub fn test_slice_getter() {
    #[cgp_getter]
    pub trait HasFoo {
        fn foo(&self) -> &[u8];
    }

    #[cgp_context]
    #[derive(HasField)]
    pub struct App {
        pub bar: Vec<u8>,
    }

    delegate_components! {
        AppComponents {
            FooGetterComponent: UseField<Symbol!("bar")>,
        }
    }

    let context = App { bar: vec![1, 2, 3] };

    assert_eq!(context.foo(), &[1, 2, 3]);
}

#[test]
pub fn test_slice_auto_getter() {
    #[cgp_auto_getter]
    pub trait HasFoo {
        fn foo(&self) -> &[u8];
    }

    #[derive(HasField)]
    pub struct App {
        pub foo: Vec<u8>,
    }

    let context = App { foo: vec![1, 2, 3] };

    assert_eq!(context.foo(), &[1, 2, 3]);
}
