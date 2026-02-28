use cgp::prelude::*;

#[test]
pub fn test_clone_getter() {
    #[cgp_type]
    pub trait HasNameType {
        type Name;
    }

    #[cgp_getter]
    pub trait HasName: HasNameType<Name: Copy> {
        fn name(&self) -> Self::Name;
    }

    #[derive(HasField)]
    pub struct App {
        pub name: &'static str,
    }

    delegate_components! {
        App {
            NameTypeProviderComponent: UseType<&'static str>,
            NameGetterComponent: UseField<Symbol!("name")>,
        }
    }

    let context = App { name: "Alice" };

    assert_eq!(context.name(), "Alice");
}

#[test]
pub fn test_clone_auto_getter() {
    #[cgp_type]
    pub trait HasNameType {
        type Name;
    }

    #[cgp_auto_getter]
    pub trait HasName: HasNameType<Name: Copy> {
        fn name(&self) -> Self::Name;
    }

    #[derive(HasField)]
    pub struct App {
        pub name: &'static str,
    }

    delegate_components! {
        App {
            NameTypeProviderComponent: UseType<&'static str>,
        }
    }

    let context = App { name: "Alice" };

    assert_eq!(context.name(), "Alice");
}
