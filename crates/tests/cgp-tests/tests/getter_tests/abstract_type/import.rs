use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_auto_getter, snapshot_cgp_getter, snapshot_cgp_type};

snapshot_cgp_type! {
    #[cgp_type]
    pub trait HasScalarType {
        type Scalar: Copy;
    }

    expand_has_scalar_type(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasScalarType {
            type Scalar: Copy;
        }
        impl<__Context__> HasScalarType for __Context__
        where
            __Context__: ScalarTypeProvider<__Context__>,
        {
            type Scalar = <__Context__ as ScalarTypeProvider<__Context__>>::Scalar;
        }
        pub trait ScalarTypeProvider<
            __Context__,
        >: IsProviderFor<ScalarTypeProviderComponent, __Context__, ()> {
            type Scalar: Copy;
        }
        impl<__Provider__, __Context__> ScalarTypeProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<ScalarTypeProviderComponent>
                + IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                ScalarTypeProviderComponent,
            >>::Delegate: ScalarTypeProvider<__Context__>,
        {
            type Scalar = <<__Provider__ as DelegateComponent<
                ScalarTypeProviderComponent,
            >>::Delegate as ScalarTypeProvider<__Context__>>::Scalar;
        }
        pub struct ScalarTypeProviderComponent;
        impl<__Context__> ScalarTypeProvider<__Context__> for UseContext
        where
            __Context__: HasScalarType,
        {
            type Scalar = <__Context__ as HasScalarType>::Scalar;
        }
        impl<__Context__> IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
        for UseContext
        where
            __Context__: HasScalarType,
        {}
        impl<__Context__, __Components__, __Path__> ScalarTypeProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: ScalarTypeProvider<__Context__>,
        {
            type Scalar = <<__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate as ScalarTypeProvider<__Context__>>::Scalar;
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
                + ScalarTypeProvider<__Context__>,
        {}
        impl<Scalar, __Context__> ScalarTypeProvider<__Context__> for UseType<Scalar>
        where
            Scalar: Copy,
        {
            type Scalar = Scalar;
        }
        impl<Scalar, __Context__> IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
        for UseType<Scalar>
        where
            Scalar: Copy,
        {}
        impl<__Provider__, Scalar, __Context__> ScalarTypeProvider<__Context__>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, ScalarTypeProviderComponent, Type = Scalar>,
            Scalar: Copy,
        {
            type Scalar = Scalar;
        }
        impl<
            __Provider__,
            Scalar,
            __Context__,
        > IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, ScalarTypeProviderComponent, Type = Scalar>,
            Scalar: Copy,
        {}
        ")
    }
}

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    #[use_type(HasScalarType::Scalar)]
    pub trait AutoRectangleFields {
        fn width(&self) -> Scalar;

        fn height(&self) -> Scalar;
    }

    expand_auto_rectangle_fields(output) {
        insta::assert_snapshot!(output, @"
        pub trait AutoRectangleFields: HasScalarType {
            fn width(&self) -> <Self as HasScalarType>::Scalar;
            fn height(&self) -> <Self as HasScalarType>::Scalar;
        }
        impl<__Context__> AutoRectangleFields for __Context__
        where
            __Context__: HasScalarType,
            __Context__: HasField<
                Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                Value = <__Context__ as HasScalarType>::Scalar,
            >,
            __Context__: HasField<
                Symbol<
                    6,
                    Chars<'h', Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>>,
                >,
                Value = <__Context__ as HasScalarType>::Scalar,
            >,
        {
            fn width(&self) -> <__Context__ as HasScalarType>::Scalar {
                self.get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>,
                            >,
                        >,
                    )
                    .clone()
            }
            fn height(&self) -> <__Context__ as HasScalarType>::Scalar {
                self.get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                6,
                                Chars<
                                    'h',
                                    Chars<
                                        'e',
                                        Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    )
                    .clone()
            }
        }
        ")
    }
}

snapshot_cgp_getter! {
    #[cgp_getter(RectangleFieldsGetter)]
    #[use_type(HasScalarType::Scalar)]
    pub trait HasRectangleFields {
        fn width(&self) -> Scalar;

        fn height(&self) -> Scalar;
    }

    expand_has_rectangle_fields(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasRectangleFields: HasScalarType {
            fn width(&self) -> <Self as HasScalarType>::Scalar;
            fn height(&self) -> <Self as HasScalarType>::Scalar;
        }
        impl<__Context__> HasRectangleFields for __Context__
        where
            __Context__: HasScalarType,
            __Context__: RectangleFieldsGetter<__Context__>,
        {
            fn width(&self) -> <Self as HasScalarType>::Scalar {
                __Context__::width(self)
            }
            fn height(&self) -> <Self as HasScalarType>::Scalar {
                __Context__::height(self)
            }
        }
        pub trait RectangleFieldsGetter<
            __Context__,
        >: IsProviderFor<RectangleFieldsGetterComponent, __Context__, ()>
        where
            __Context__: HasScalarType,
        {
            fn width(__context__: &__Context__) -> <__Context__ as HasScalarType>::Scalar;
            fn height(__context__: &__Context__) -> <__Context__ as HasScalarType>::Scalar;
        }
        impl<__Provider__, __Context__> RectangleFieldsGetter<__Context__> for __Provider__
        where
            __Context__: HasScalarType,
            __Provider__: DelegateComponent<RectangleFieldsGetterComponent>
                + IsProviderFor<RectangleFieldsGetterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                RectangleFieldsGetterComponent,
            >>::Delegate: RectangleFieldsGetter<__Context__>,
        {
            fn width(__context__: &__Context__) -> <__Context__ as HasScalarType>::Scalar {
                <__Provider__ as DelegateComponent<
                    RectangleFieldsGetterComponent,
                >>::Delegate::width(__context__)
            }
            fn height(__context__: &__Context__) -> <__Context__ as HasScalarType>::Scalar {
                <__Provider__ as DelegateComponent<
                    RectangleFieldsGetterComponent,
                >>::Delegate::height(__context__)
            }
        }
        pub struct RectangleFieldsGetterComponent;
        impl<__Context__> RectangleFieldsGetter<__Context__> for UseContext
        where
            __Context__: HasScalarType,
            __Context__: HasRectangleFields,
        {
            fn width(__context__: &__Context__) -> <__Context__ as HasScalarType>::Scalar {
                __Context__::width(__context__)
            }
            fn height(__context__: &__Context__) -> <__Context__ as HasScalarType>::Scalar {
                __Context__::height(__context__)
            }
        }
        impl<__Context__> IsProviderFor<RectangleFieldsGetterComponent, __Context__, ()>
        for UseContext
        where
            __Context__: HasScalarType,
            __Context__: HasRectangleFields,
        {}
        impl<__Context__, __Components__, __Path__> RectangleFieldsGetter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasScalarType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: RectangleFieldsGetter<__Context__>,
        {
            fn width(__context__: &__Context__) -> <__Context__ as HasScalarType>::Scalar {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::width(__context__)
            }
            fn height(__context__: &__Context__) -> <__Context__ as HasScalarType>::Scalar {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::height(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<RectangleFieldsGetterComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Context__: HasScalarType,
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<RectangleFieldsGetterComponent, __Context__, ()>
                + RectangleFieldsGetter<__Context__>,
        {}
        impl<__Context__> RectangleFieldsGetter<__Context__> for UseFields
        where
            __Context__: HasScalarType,
            __Context__: HasField<
                Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                Value = <__Context__ as HasScalarType>::Scalar,
            >,
            __Context__: HasField<
                Symbol<
                    6,
                    Chars<'h', Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>>,
                >,
                Value = <__Context__ as HasScalarType>::Scalar,
            >,
        {
            fn width(__context__: &__Context__) -> <__Context__ as HasScalarType>::Scalar {
                __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>,
                            >,
                        >,
                    )
                    .clone()
            }
            fn height(__context__: &__Context__) -> <__Context__ as HasScalarType>::Scalar {
                __context__
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                6,
                                Chars<
                                    'h',
                                    Chars<
                                        'e',
                                        Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>,
                                    >,
                                >,
                            >,
                        >,
                    )
                    .clone()
            }
        }
        impl<__Context__> IsProviderFor<RectangleFieldsGetterComponent, __Context__, ()>
        for UseFields
        where
            __Context__: HasScalarType,
            __Context__: HasField<
                Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                Value = <__Context__ as HasScalarType>::Scalar,
            >,
            __Context__: HasField<
                Symbol<
                    6,
                    Chars<'h', Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>>,
                >,
                Value = <__Context__ as HasScalarType>::Scalar,
            >,
        {}
        ")
    }
}
