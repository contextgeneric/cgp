mod basic_const {
    use cgp::prelude::*;
    use cgp_macro_test_util::{snapshot_cgp_provider, snapshot_delegate_and_check_components};

    #[cgp_component(ConstantGetter)]
    pub trait HasConstant {
        const CONSTANT: u64;
    }

    pub struct UseConstant<const CONSTANT: u64>;

    snapshot_cgp_provider! {
        #[cgp_provider]
        impl<Context, const CONSTANT: u64> ConstantGetter<Context> for UseConstant<CONSTANT> {
            const CONSTANT: u64 = CONSTANT;
        }

        expand_use_constant(output) {
            insta::assert_snapshot!(output, @"
            impl<Context, const CONSTANT: u64> ConstantGetter<Context> for UseConstant<CONSTANT> {
                const CONSTANT: u64 = CONSTANT;
            }
            impl<Context, const CONSTANT: u64> IsProviderFor<ConstantGetterComponent, Context, ()>
            for UseConstant<CONSTANT> {}
            ")
        }
    }

    pub struct MyContext;

    snapshot_delegate_and_check_components! {
        delegate_and_check_components! {
            MyContext {
                ConstantGetterComponent: UseConstant<42>,
            }
        }

        expand_my_context(output) {
            insta::assert_snapshot!(output, @"
            impl DelegateComponent<ConstantGetterComponent> for MyContext {
                type Delegate = UseConstant<42>;
            }
            impl<
                __Context__,
                __Params__,
            > IsProviderFor<ConstantGetterComponent, __Context__, __Params__> for MyContext
            where
                UseConstant<42>: IsProviderFor<ConstantGetterComponent, __Context__, __Params__>,
            {}
            trait __CanUseMyContext<
                __Component__,
                __Params__: ?Sized,
            >: CanUseComponent<__Component__, __Params__> {}
            impl __CanUseMyContext<ConstantGetterComponent, ()> for MyContext {}
            ")
        }
    }
}

pub fn test_component_with_const() {
    use basic_const::{HasConstant, MyContext};

    assert_eq!(<MyContext as HasConstant>::CONSTANT, 42);
}

mod generic_const {
    use cgp::prelude::*;
    use cgp_macro_test_util::{
        snapshot_cgp_provider, snapshot_cgp_type, snapshot_check_components,
    };

    snapshot_cgp_type! {
        #[cgp_type]
        pub trait HasUnitType {
            type Unit;
        }

        expand_has_unit_type(output) {
            insta::assert_snapshot!(output, @"
            pub trait HasUnitType {
                type Unit;
            }
            impl<__Context__> HasUnitType for __Context__
            where
                __Context__: UnitTypeProvider<__Context__>,
            {
                type Unit = <__Context__ as UnitTypeProvider<__Context__>>::Unit;
            }
            pub trait UnitTypeProvider<
                __Context__,
            >: IsProviderFor<UnitTypeProviderComponent, __Context__, ()> {
                type Unit;
            }
            impl<__Provider__, __Context__> UnitTypeProvider<__Context__> for __Provider__
            where
                __Provider__: DelegateComponent<UnitTypeProviderComponent>
                    + IsProviderFor<UnitTypeProviderComponent, __Context__, ()>,
                <__Provider__ as DelegateComponent<
                    UnitTypeProviderComponent,
                >>::Delegate: UnitTypeProvider<__Context__>,
            {
                type Unit = <<__Provider__ as DelegateComponent<
                    UnitTypeProviderComponent,
                >>::Delegate as UnitTypeProvider<__Context__>>::Unit;
            }
            pub struct UnitTypeProviderComponent;
            impl<__Context__> UnitTypeProvider<__Context__> for UseContext
            where
                __Context__: HasUnitType,
            {
                type Unit = <__Context__ as HasUnitType>::Unit;
            }
            impl<__Context__> IsProviderFor<UnitTypeProviderComponent, __Context__, ()>
            for UseContext
            where
                __Context__: HasUnitType,
            {}
            impl<__Context__, __Components__, __Path__> UnitTypeProvider<__Context__>
            for RedirectLookup<__Components__, __Path__>
            where
                __Components__: DelegateComponent<__Path__>,
                <__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate: UnitTypeProvider<__Context__>,
            {
                type Unit = <<__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate as UnitTypeProvider<__Context__>>::Unit;
            }
            impl<
                __Context__,
                __Components__,
                __Path__,
            > IsProviderFor<UnitTypeProviderComponent, __Context__, ()>
            for RedirectLookup<__Components__, __Path__>
            where
                __Components__: DelegateComponent<__Path__>,
                <__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate: IsProviderFor<UnitTypeProviderComponent, __Context__, ()>
                    + UnitTypeProvider<__Context__>,
            {}
            impl<Unit, __Context__> UnitTypeProvider<__Context__> for UseType<Unit> {
                type Unit = Unit;
            }
            impl<Unit, __Context__> IsProviderFor<UnitTypeProviderComponent, __Context__, ()>
            for UseType<Unit> {}
            impl<__Provider__, Unit, __Context__> UnitTypeProvider<__Context__>
            for WithProvider<__Provider__>
            where
                __Provider__: TypeProvider<__Context__, UnitTypeProviderComponent, Type = Unit>,
            {
                type Unit = Unit;
            }
            impl<
                __Provider__,
                Unit,
                __Context__,
            > IsProviderFor<UnitTypeProviderComponent, __Context__, ()>
            for WithProvider<__Provider__>
            where
                __Provider__: TypeProvider<__Context__, UnitTypeProviderComponent, Type = Unit>,
            {}
            ")
        }
    }

    #[cgp_component(ConstantGetter)]
    pub trait HasConstant: HasUnitType {
        const CONSTANT: Self::Unit;
    }

    pub struct UseConstant<const CONSTANT: u64>;

    snapshot_cgp_provider! {
        #[cgp_provider]
        impl<Context, const CONSTANT: u64> ConstantGetter<Context> for UseConstant<CONSTANT>
        where
            Context: HasUnitType<Unit = u64>,
        {
            const CONSTANT: u64 = CONSTANT;
        }

        expand_use_constant(output) {
            insta::assert_snapshot!(output, @"
            impl<Context, const CONSTANT: u64> ConstantGetter<Context> for UseConstant<CONSTANT>
            where
                Context: HasUnitType<Unit = u64>,
            {
                const CONSTANT: u64 = CONSTANT;
            }
            impl<Context, const CONSTANT: u64> IsProviderFor<ConstantGetterComponent, Context, ()>
            for UseConstant<CONSTANT>
            where
                Context: HasUnitType<Unit = u64>,
            {}
            ")
        }
    }

    pub struct MyContext;

    delegate_components! {
        MyContext {
            UnitTypeProviderComponent: UseType<u64>,
            ConstantGetterComponent: UseConstant<42>,
        }
    }

    snapshot_check_components! {
        check_components! {
            MyContext {
                ConstantGetterComponent,
            }
        }

        expand_check_my_context(output) {
            insta::assert_snapshot!(output, @"
            trait __CheckMyContext<
                __Component__,
                __Params__: ?Sized,
            >: CanUseComponent<__Component__, __Params__> {}
            impl __CheckMyContext<ConstantGetterComponent, ()> for MyContext {}
            ")
        }
    }
}

pub fn test_component_with_generic_const() {
    use generic_const::{HasConstant, MyContext};

    assert_eq!(<MyContext as HasConstant>::CONSTANT, 42);
}
