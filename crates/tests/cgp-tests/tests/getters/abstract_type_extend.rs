//! Getters whose return type is an abstract type imported from another component
//! via `#[extend(HasScalarType)]`, so the signatures name the type as
//! `Self::Scalar`. Both the auto getter and the full getter variant are pinned.
//! The `#[cgp_type]` scaffolding is written plainly here — its expansion is owned
//! by the `abstract_types` concept.
//!
//! See docs/reference/macros/cgp_getter.md, docs/reference/macros/cgp_auto_getter.md,
//! and docs/reference/providers/use_type.md.

use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_auto_getter, snapshot_cgp_getter};

#[cgp_type]
pub trait HasScalarType {
    type Scalar: Copy;
}

snapshot_cgp_auto_getter! {
    #[cgp_auto_getter]
    #[extend(HasScalarType)]
    pub trait AutoRectangleFields {
        fn width(&self) -> Self::Scalar;

        fn height(&self) -> Self::Scalar;
    }

    expand_auto_rectangle_fields(output) {
        insta::assert_snapshot!(output, @"
        pub trait AutoRectangleFields: HasScalarType {
            fn width(&self) -> Self::Scalar;
            fn height(&self) -> Self::Scalar;
        }
        impl<__Context__> AutoRectangleFields for __Context__
        where
            __Context__: HasScalarType,
            __Context__: HasField<
                Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                Value = __Context__::Scalar,
            >,
            __Context__: HasField<
                Symbol<
                    6,
                    Chars<'h', Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>>,
                >,
                Value = __Context__::Scalar,
            >,
        {
            fn width(&self) -> __Context__::Scalar {
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
            fn height(&self) -> __Context__::Scalar {
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
    #[extend(HasScalarType)]
    pub trait HasRectangleFields {
        fn width(&self) -> Self::Scalar;

        fn height(&self) -> Self::Scalar;
    }

    expand_has_rectangle_fields(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasRectangleFields: HasScalarType {
            fn width(&self) -> Self::Scalar;
            fn height(&self) -> Self::Scalar;
        }
        impl<__Context__> HasRectangleFields for __Context__
        where
            __Context__: HasScalarType,
            __Context__: RectangleFieldsGetter<__Context__>,
        {
            fn width(&self) -> Self::Scalar {
                __Context__::width(self)
            }
            fn height(&self) -> Self::Scalar {
                __Context__::height(self)
            }
        }
        pub trait RectangleFieldsGetter<
            __Context__,
        >: IsProviderFor<RectangleFieldsGetterComponent, __Context__, ()>
        where
            __Context__: HasScalarType,
        {
            fn width(__context__: &__Context__) -> __Context__::Scalar;
            fn height(__context__: &__Context__) -> __Context__::Scalar;
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
            fn width(__context__: &__Context__) -> __Context__::Scalar {
                <__Provider__ as DelegateComponent<
                    RectangleFieldsGetterComponent,
                >>::Delegate::width(__context__)
            }
            fn height(__context__: &__Context__) -> __Context__::Scalar {
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
            fn width(__context__: &__Context__) -> __Context__::Scalar {
                __Context__::width(__context__)
            }
            fn height(__context__: &__Context__) -> __Context__::Scalar {
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
            fn width(__context__: &__Context__) -> __Context__::Scalar {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::width(__context__)
            }
            fn height(__context__: &__Context__) -> __Context__::Scalar {
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
                Value = __Context__::Scalar,
            >,
            __Context__: HasField<
                Symbol<
                    6,
                    Chars<'h', Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>>,
                >,
                Value = __Context__::Scalar,
            >,
        {
            fn width(__context__: &__Context__) -> __Context__::Scalar {
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
            fn height(__context__: &__Context__) -> __Context__::Scalar {
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
                Value = __Context__::Scalar,
            >,
            __Context__: HasField<
                Symbol<
                    6,
                    Chars<'h', Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>>,
                >,
                Value = __Context__::Scalar,
            >,
        {}
        ")
    }
}
