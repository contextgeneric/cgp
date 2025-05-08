use cgp::prelude::*;

pub fn test_basic_delegate_and_check_components() {
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
    pub struct MyContext {
        pub name: String,
    }

    delegate_and_check_components! {
        CanUseMyContext for MyContext;
        MyContextComponents {
            NameTypeProviderComponent: UseType<String>,
            NameGetterComponent: UseField<symbol!("name")>,
        }
    }
}

pub fn test_generic_delegate_and_check_components() {
    #[cgp_type]
    pub trait HasNameType {
        type Name;
    }

    #[cgp_getter]
    pub trait HasName: HasNameType {
        fn name(&self) -> &Self::Name;
    }

    #[cgp_context(MyContextComponents<T>)]
    #[derive(HasField)]
    pub struct MyContext<T> {
        pub name: T,
    }

    delegate_and_check_components! {
        <T>
        CanUseMyContext for MyContext<T>;
        MyContextComponents<T> {
            NameTypeProviderComponent: UseType<T>,
            NameGetterComponent: UseField<symbol!("name")>,
        }
    }
}
