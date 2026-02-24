#![allow(dead_code)]

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

    #[derive(HasField)]
    pub struct MyContext {
        pub name: String,
    }

    delegate_and_check_components! {
        CanUseMyContext for MyContext;
        MyContext {
            NameTypeProviderComponent: UseType<String>,
            NameGetterComponent: UseField<Symbol!("name")>,
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

    #[derive(HasField)]
    pub struct MyContext<T> {
        pub name: T,
    }

    delegate_and_check_components! {
        <T>
        CanUseMyContext for MyContext<T>;
        MyContext<T> {
            NameTypeProviderComponent: UseType<T>,
            NameGetterComponent: UseField<Symbol!("name")>,
        }
    }
}
