use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_component;
use insta::assert_snapshot;

snapshot_cgp_component! {
    #[ConstantGetter]
    {
        pub trait HasConstant {
            const CONSTANT: u64;
        }
    }

    test_expansion(output) {
        assert_snapshot!(output, @"
        pub trait HasConstant {
            const CONSTANT: u64;
        }
        impl<__Context__> HasConstant for __Context__
        where
            __Context__: ConstantGetter<__Context__>,
        {
            const CONSTANT: u64 = <__Context__ as ConstantGetter<__Context__>>::CONSTANT;
        }
        pub trait ConstantGetter<
            __Context__,
        >: IsProviderFor<ConstantGetterComponent, __Context__, ()> {
            const CONSTANT: u64;
        }
        impl<__Provider__, __Context__> ConstantGetter<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<ConstantGetterComponent>
                + IsProviderFor<ConstantGetterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                ConstantGetterComponent,
            >>::Delegate: ConstantGetter<__Context__>,
        {
            const CONSTANT: u64 = <<__Provider__ as DelegateComponent<
                ConstantGetterComponent,
            >>::Delegate as ConstantGetter<__Context__>>::CONSTANT;
        }
        pub struct ConstantGetterComponent;
        impl<__Context__> ConstantGetter<__Context__> for UseContext
        where
            __Context__: HasConstant,
        {
            const CONSTANT: u64 = <__Context__ as HasConstant>::CONSTANT;
        }
        impl<__Context__> IsProviderFor<ConstantGetterComponent, __Context__, ()> for UseContext
        where
            __Context__: HasConstant,
        {}
        impl<__Context__, __Components__, __Path__> ConstantGetter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: ConstantGetter<__Context__>,
        {
            const CONSTANT: u64 = <<__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate as ConstantGetter<__Context__>>::CONSTANT;
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<ConstantGetterComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<ConstantGetterComponent, __Context__, ()>
                + ConstantGetter<__Context__>,
        {}
        ")
    }
}

pub fn test_component_with_const() {
    pub struct UseConstant<const CONSTANT: u64>;

    #[cgp_provider]
    impl<Context, const CONSTANT: u64> ConstantGetter<Context> for UseConstant<CONSTANT> {
        const CONSTANT: u64 = CONSTANT;
    }

    pub struct MyContext;

    delegate_and_check_components! {
        MyContext {
            ConstantGetterComponent: UseConstant<42>,
        }
    }

    assert_eq!(<MyContext as HasConstant>::CONSTANT, 42);
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

    pub struct MyContext;

    delegate_components! {
        MyContext {
            UnitTypeProviderComponent: UseType<u64>,
            ConstantGetterComponent: UseConstant<42>,
        }
    }

    check_components! {
        MyContext {
            ConstantGetterComponent,
        }
    }

    assert_eq!(<MyContext as HasConstant>::CONSTANT, 42);
}
