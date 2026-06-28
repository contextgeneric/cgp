use std::ops::Mul;

use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_fn, snapshot_cgp_type};

snapshot_cgp_type! {
    #[cgp_type]
    pub trait HasScalarType {
        type Scalar;
    }

    expand_has_scalar_type(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasScalarType {
            type Scalar;
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
            type Scalar;
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
        impl<Scalar, __Context__> ScalarTypeProvider<__Context__> for UseType<Scalar> {
            type Scalar = Scalar;
        }
        impl<Scalar, __Context__> IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
        for UseType<Scalar> {}
        impl<__Provider__, Scalar, __Context__> ScalarTypeProvider<__Context__>
        for WithProvider<__Provider__>
        where
            __Provider__: TypeProvider<__Context__, ScalarTypeProviderComponent, Type = Scalar>,
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
        {}
        ")
    }
}

snapshot_cgp_fn! {
    #[cgp_fn]
    #[extend(HasScalarType)]
    pub fn rectangle_area(
        &self,
        #[implicit] width: Self::Scalar,
        #[implicit] height: Self::Scalar,
    ) -> Self::Scalar
    where
        Self::Scalar: Mul<Output = Self::Scalar> + Copy,
    {
        width * height
    }

    expand_rectangle_area(output) {
        insta::assert_snapshot!(output, @"
        pub trait RectangleArea: HasScalarType {
            fn rectangle_area(&self) -> Self::Scalar;
        }
        impl<__Context__> RectangleArea for __Context__
        where
            Self::Scalar: Mul<Output = Self::Scalar> + Copy,
            Self: HasScalarType,
            Self: HasField<
                    Symbol<5, Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>>,
                    Value = Self::Scalar,
                >
                + HasField<
                    Symbol<
                        6,
                        Chars<
                            'h',
                            Chars<'e', Chars<'i', Chars<'g', Chars<'h', Chars<'t', Nil>>>>>,
                        >,
                    >,
                    Value = Self::Scalar,
                >,
        {
            fn rectangle_area(&self) -> Self::Scalar {
                let width: Self::Scalar = self
                    .get_field(
                        ::core::marker::PhantomData::<
                            Symbol<
                                5,
                                Chars<'w', Chars<'i', Chars<'d', Chars<'t', Chars<'h', Nil>>>>>,
                            >,
                        >,
                    )
                    .clone();
                let height: Self::Scalar = self
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
                    .clone();
                width * height
            }
        }
        ")
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl HasScalarType for Rectangle {
    type Scalar = f64;
}

pub trait CheckRectangle: RectangleArea {}
impl CheckRectangle for Rectangle {}
