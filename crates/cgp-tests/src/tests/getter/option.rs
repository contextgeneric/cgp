use cgp::prelude::*;

#[test]
pub fn test_option_getter() {
    #[cgp_getter]
    pub trait HasFoo {
        fn foo(&self) -> Option<&String>;
    }

    #[cgp_context]
    #[derive(HasField)]
    pub struct App {
        pub bar: Option<String>,
    }

    delegate_components! {
        AppComponents {
            FooGetterComponent: UseField<Symbol!("bar")>,
        }
    }

    let context = App {
        bar: Some("foo".to_owned()),
    };

    assert_eq!(context.foo(), Some(&"foo".to_owned()));
}

#[test]
pub fn test_option_auto_getter() {
    #[cgp_auto_getter]
    pub trait HasFoo {
        fn foo(&self) -> Option<&String>;
    }

    #[cgp_context]
    #[derive(HasField)]
    pub struct App {
        pub foo: Option<String>,
    }

    let context = App {
        foo: Some("foo".to_owned()),
    };

    assert_eq!(context.foo(), Some(&"foo".to_owned()));
}
