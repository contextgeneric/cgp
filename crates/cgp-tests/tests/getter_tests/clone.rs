use cgp::prelude::*;

#[test]
pub fn test_clone_getter() {
    #[cgp_type]
    pub trait HasNameType {
        type Name;
    }

    #[cgp_getter]
    pub trait HasName: HasNameType<Name: Clone> {
        fn name(&self) -> Self::Name;
    }

    #[derive(HasField)]
    pub struct App {
        pub name: String,
    }

    delegate_components! {
        App {
            NameTypeProviderComponent: UseType<String>,
            NameGetterComponent: UseField<Symbol!("name")>,
        }
    }

    let context = App {
        name: "Alice".to_owned(),
    };

    assert_eq!(context.name(), "Alice");
}

#[test]
pub fn test_clone_auto_getter() {
    #[cgp_type]
    pub trait HasNameType {
        type Name;
    }

    #[cgp_auto_getter]
    pub trait HasName: HasNameType<Name: Clone> {
        fn name(&self) -> Self::Name;
    }

    #[derive(HasField)]
    pub struct App {
        pub name: String,
    }

    delegate_components! {
        App {
            NameTypeProviderComponent: UseType<String>,
        }
    }

    let context = App {
        name: "Alice".to_owned(),
    };

    assert_eq!(context.name(), "Alice");
}
