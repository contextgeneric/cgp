use std::convert::Infallible;
use std::ops::Mul;

use cgp::core::error::ErrorTypeProviderComponent;
use cgp::prelude::*;
use cgp_macro_test_util::{snapshot_cgp_type, snapshot_delegate_and_check_components};

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

#[cgp_component(AreaCalculator)]
#[use_type(HasScalarType::Scalar, HasErrorType::Error)]
pub trait CanCalculateArea {
    fn area(&self) -> Result<Scalar, Error>;
}

#[cgp_impl(new RectangleArea)]
#[use_type(HasScalarType::Scalar, HasErrorType::Error)]
impl AreaCalculator
where
    Scalar: Mul<Output = Scalar> + Copy,
{
    fn area(&self, #[implicit] width: Scalar, #[implicit] height: Scalar) -> Result<Scalar, Error> {
        Ok(width * height)
    }
}

#[derive(HasField)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

snapshot_delegate_and_check_components! {
    delegate_and_check_components! {
        Rectangle {
            ErrorTypeProviderComponent:
                UseType<Infallible>,
            ScalarTypeProviderComponent:
                UseType<f64>,
            AreaCalculatorComponent:
                RectangleArea,
        }
    }

    expand_rectangle(output) {
        insta::assert_snapshot!(output, @"
        impl DelegateComponent<ErrorTypeProviderComponent> for Rectangle {
            type Delegate = UseType<Infallible>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<ErrorTypeProviderComponent, __Context__, __Params__> for Rectangle
        where
            UseType<
                Infallible,
            >: IsProviderFor<ErrorTypeProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<ScalarTypeProviderComponent> for Rectangle {
            type Delegate = UseType<f64>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<ScalarTypeProviderComponent, __Context__, __Params__> for Rectangle
        where
            UseType<f64>: IsProviderFor<ScalarTypeProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AreaCalculatorComponent> for Rectangle {
            type Delegate = RectangleArea;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<AreaCalculatorComponent, __Context__, __Params__> for Rectangle
        where
            RectangleArea: IsProviderFor<AreaCalculatorComponent, __Context__, __Params__>,
        {}
        trait __CanUseRectangle<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CanUseRectangle<ErrorTypeProviderComponent, ()> for Rectangle {}
        impl __CanUseRectangle<ScalarTypeProviderComponent, ()> for Rectangle {}
        impl __CanUseRectangle<AreaCalculatorComponent, ()> for Rectangle {}
        ")
    }
}
