//! `#[cgp_component]` on a trait with a single type parameter and no lifetime.
//!
//! The type parameter is appended after `__Context__` in the provider trait, put
//! into the `IsProviderFor` params tuple as a bare `(Shape)`, and appended to the
//! `RedirectLookup` lookup path via `ConcatPath<PathCons<Shape, Nil>>`. This is the
//! type-parameter-only variant of the macro expansion, distinct from the combined
//! lifetime-and-type case in `component_lifetime`. The provider, `delegate_components!`,
//! and `check_components!` wiring below is written plainly (owned by the
//! `basic_delegation` and `checking` concepts).
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_component.md (Snapshots) for this
//! type-parameter variant, and cgp-knowledge-base/cgp/reference/macros/cgp_component.md for the
//! user-facing semantics.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_component;

snapshot_cgp_component! {
    #[cgp_component(AreaCalculator)]
    pub trait CanCalculateArea<Shape> {
        fn calculate_area(&self, shape: &Shape) -> f64;
    }

    expand_area_calculator(output) {
        insta::assert_snapshot!(output, @"
        pub trait CanCalculateArea<Shape> {
            fn calculate_area(&self, shape: &Shape) -> f64;
        }
        impl<__Context__, Shape> CanCalculateArea<Shape> for __Context__
        where
            __Context__: AreaCalculator<__Context__, Shape>,
        {
            fn calculate_area(&self, shape: &Shape) -> f64 {
                __Context__::calculate_area(self, shape)
            }
        }
        pub trait AreaCalculator<
            __Context__,
            Shape,
        >: IsProviderFor<AreaCalculatorComponent, __Context__, (Shape)> {
            fn calculate_area(__context__: &__Context__, shape: &Shape) -> f64;
        }
        impl<__Provider__, __Context__, Shape> AreaCalculator<__Context__, Shape>
        for __Provider__
        where
            __Provider__: DelegateComponent<AreaCalculatorComponent>
                + IsProviderFor<AreaCalculatorComponent, __Context__, (Shape)>,
            <__Provider__ as DelegateComponent<
                AreaCalculatorComponent,
            >>::Delegate: AreaCalculator<__Context__, Shape>,
        {
            fn calculate_area(__context__: &__Context__, shape: &Shape) -> f64 {
                <__Provider__ as DelegateComponent<
                    AreaCalculatorComponent,
                >>::Delegate::calculate_area(__context__, shape)
            }
        }
        pub struct AreaCalculatorComponent;
        impl<__Context__, Shape> AreaCalculator<__Context__, Shape> for UseContext
        where
            __Context__: CanCalculateArea<Shape>,
        {
            fn calculate_area(__context__: &__Context__, shape: &Shape) -> f64 {
                __Context__::calculate_area(__context__, shape)
            }
        }
        impl<__Context__, Shape> IsProviderFor<AreaCalculatorComponent, __Context__, (Shape)>
        for UseContext
        where
            __Context__: CanCalculateArea<Shape>,
        {}
        impl<__Context__, Shape, __Components__, __Path__> AreaCalculator<__Context__, Shape>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<Shape, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<Shape, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<Shape, Nil>>>::Output,
            >>::Delegate: AreaCalculator<__Context__, Shape>,
        {
            fn calculate_area(__context__: &__Context__, shape: &Shape) -> f64 {
                <__Components__ as DelegateComponent<
                    <__Path__ as ConcatPath<PathCons<Shape, Nil>>>::Output,
                >>::Delegate::calculate_area(__context__, shape)
            }
        }
        impl<
            __Context__,
            Shape,
            __Components__,
            __Path__,
        > IsProviderFor<AreaCalculatorComponent, __Context__, (Shape)>
        for RedirectLookup<__Components__, __Path__>
        where
            __Path__: ConcatPath<PathCons<Shape, Nil>>,
            __Components__: DelegateComponent<
                <__Path__ as ConcatPath<PathCons<Shape, Nil>>>::Output,
            >,
            <__Components__ as DelegateComponent<
                <__Path__ as ConcatPath<PathCons<Shape, Nil>>>::Output,
            >>::Delegate: IsProviderFor<AreaCalculatorComponent, __Context__, (Shape)>
                + AreaCalculator<__Context__, Shape>,
        {}
        ")
    }
}

pub trait HasArea {
    fn area(&self) -> f64;
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// A single provider handles every shape that knows its own area.
#[cgp_new_provider]
impl<Context, Shape: HasArea> AreaCalculator<Context, Shape> for ComputeArea {
    fn calculate_area(_context: &Context, shape: &Shape) -> f64 {
        shape.area()
    }
}

pub struct MyContext;

delegate_components! {
    MyContext {
        AreaCalculatorComponent: ComputeArea,
    }
}

check_components! {
    MyContext {
        AreaCalculatorComponent: Rectangle,
    }
}

#[test]
fn test_component_with_type_param() {
    let rect = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    assert_eq!(MyContext.calculate_area(&rect), 12.0);
}
