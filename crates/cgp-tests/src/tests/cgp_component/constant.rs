use cgp::prelude::*;

pub fn test_component_with_const() {
    #[cgp_component(ConstantGetter)]
    pub trait HasConstant {
        const CONSTANT: u64;
    }

    pub struct UseConstant<const CONSTANT: u64>;

    #[cgp_provider]
    impl<Context, const CONSTANT: u64> ConstantGetter<Context> for UseConstant<CONSTANT> {
        const CONSTANT: u64 = CONSTANT;
    }

    #[cgp_context]
    pub struct MyContext;

    delegate_components! {
        MyContextComponents {
            ConstantGetterComponent: UseConstant<42>,
        }
    }

    check_components! {
        CanUseMyContext for MyContext {
            ConstantGetterComponent,
        }
    }

    assert_eq!(MyContext::CONSTANT, 42);
}

pub fn test_component_with_generic_const() {
    #[cgp_type]
    pub trait HasUnitType {
        type Unit;
    }

    #[cgp_component(ConstantGetter)]
    pub trait HasConstant: HasUnitType {
        const CONSTANT: Self::Unit;
    }

    pub struct UseConstant<const CONSTANT: u64>;

    #[cgp_provider]
    impl<Context, const CONSTANT: u64> ConstantGetter<Context> for UseConstant<CONSTANT>
    where
        Context: HasUnitType<Unit = u64>,
    {
        const CONSTANT: u64 = CONSTANT;
    }

    #[cgp_context]
    pub struct MyContext;

    delegate_components! {
        MyContextComponents {
            UnitTypeProviderComponent: UseType<u64>,
            ConstantGetterComponent: UseConstant<42>,
        }
    }

    check_components! {
        CanUseMyContext for MyContext {
            ConstantGetterComponent,
        }
    }

    assert_eq!(MyContext::CONSTANT, 42);
}
