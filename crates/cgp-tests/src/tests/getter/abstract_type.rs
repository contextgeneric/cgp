use cgp::prelude::*;

#[test]
pub fn test_abstract_type_getter() {
    #[cgp_type]
    pub trait HasNameType {
        type Name;
    }

    #[cgp_getter]
    pub trait HasName: HasNameType {
        fn name(&self) -> &Self::Name;
    }

    #[cgp_context]
    #[derive(HasField)]
    pub struct App {
        pub name: String,
    }

    delegate_components! {
        AppComponents {
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
pub fn test_abstract_type_auto_getter() {
    #[cgp_type]
    pub trait HasNameType {
        type Name;
    }

    #[cgp_auto_getter]
    pub trait HasName: HasNameType {
        fn name(&self) -> &Self::Name;
    }

    #[cgp_context]
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
