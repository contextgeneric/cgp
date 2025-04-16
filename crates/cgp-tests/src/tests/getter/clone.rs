use cgp::prelude::*;

#[test]
pub fn test_clone_getter() {
    #[cgp_type]
    pub trait HasNameType {
        type Name;
    }

    #[cgp_getter {
        provider: NameGetter,
    }]
    pub trait HasName: HasNameType<Name: Clone> {
        fn name(&self) -> Self::Name;
    }

    #[cgp_context(AppComponents)]
    #[derive(HasField)]
    pub struct App {
        pub name: String,
    }

    delegate_components! {
        AppComponents {
            NameTypeProviderComponent: UseType<String>,
            NameGetterComponent: UseField<symbol!("name")>,
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

    #[cgp_context(AppComponents)]
    #[derive(HasField)]
    pub struct App {
        pub name: String,
    }

    delegate_components! {
        AppComponents {
            NameTypeProviderComponent: UseType<String>,
        }
    }

    let context = App {
        name: "Alice".to_owned(),
    };

    assert_eq!(context.name(), "Alice");
}
