//! Every `delegate_components!` body form in one block.
//!
//! The body's forms are independent choices — three mapping operators, three key
//! forms, and the leading statements — and nothing stops a table from using all of
//! them at once. Every other test in the suite stays inside one family, so this is
//! the only place the composition itself is pinned: an `open` statement, a `->`
//! forwarding entry into an aggregate provider, a bracketed list key sharing one
//! provider, and `@`-path keys using both grouping forms and a per-segment
//! generic, all against one context.
//!
//! The two grouping forms are the pair most easily confused, so both appear on the
//! same component: `{Circle, Ellipse}` groups whole path tails and ends the path,
//! while `[Square, Triangle]` groups alternatives for one segment. They fan out to
//! one impl pair per alternative either way.
//!
//! The `delegate_components!` snapshot is the golden output this file owns; the
//! components, the providers, and the `Bundle` aggregate are incidental
//! scaffolding.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/delegate_components.md and
//! cgp-knowledge-base/cgp/reference/macros/delegate_components.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_components;

// Incidental: one generic component to dispatch per shape, and three plain ones
// wired through the other forms.
#[cgp_component(AreaCalculator)]
pub trait CanCalculateArea<Shape> {
    fn area(&self, shape: &Shape) -> f64;
}

#[cgp_component(BarProvider)]
pub trait Bar {
    fn bar(&self);
}

#[cgp_component(BazProvider)]
pub trait Baz {
    fn baz(&self);
}

#[cgp_component(QuuxProvider)]
pub trait Quux {
    fn quux(&self);
}

pub struct Rectangle;
pub struct Circle;
pub struct Ellipse;
pub struct Square;
pub struct Triangle;

// Incidental: per-shape and plain providers.
#[cgp_impl(new ShapeArea)]
impl<Shape> AreaCalculator<Shape> {
    fn area(&self, _shape: &Shape) -> f64 {
        0.0
    }
}

#[cgp_impl(new RefArea)]
impl<Shape> AreaCalculator<Shape> {
    fn area(&self, _shape: &Shape) -> f64 {
        0.0
    }
}

#[cgp_impl(new DummyBar)]
impl BarProvider {
    fn bar(&self) {}
}

#[cgp_impl(new DummyBaz)]
impl BazProvider {
    fn baz(&self) {}
}

#[cgp_impl(DummyBaz)]
impl QuuxProvider {
    fn quux(&self) {}
}

// Incidental: the aggregate provider the `->` entry forwards into.
delegate_components! {
    new Bundle {
        BarProviderComponent: DummyBar,
    }
}

pub struct App;

snapshot_delegate_components! {
    delegate_components! {
        App {
            // Statement, which must lead the block.
            open AreaCalculatorComponent;

            // `->`: take whatever `Bundle` wires for this key.
            BarProviderComponent ->
                Bundle,

            // List key: one provider for two components.
            [BazProviderComponent, QuuxProviderComponent]:
                DummyBaz,

            // Path key with a braced group — whole tails, ends the path.
            @AreaCalculatorComponent.{Rectangle, Circle}:
                ShapeArea,

            // Path key with a bracketed group — alternatives for one segment.
            @AreaCalculatorComponent.[Square, Triangle]:
                ShapeArea,

            // Path key whose segment carries its own generics.
            @AreaCalculatorComponent.<'a, T> &'a T:
                RefArea,
        }
    }

    expand_combined_forms_app(output) {
        insta::assert_snapshot!(output, @"
        impl DelegateComponent<AreaCalculatorComponent> for App {
            type Delegate = RedirectLookup<App, PathCons<AreaCalculatorComponent, Nil>>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<AreaCalculatorComponent, __Context__, __Params__> for App
        where
            RedirectLookup<
                App,
                PathCons<AreaCalculatorComponent, Nil>,
            >: IsProviderFor<AreaCalculatorComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<BarProviderComponent> for App
        where
            Bundle: DelegateComponent<BarProviderComponent>,
        {
            type Delegate = <Bundle as DelegateComponent<BarProviderComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<BarProviderComponent, __Context__, __Params__> for App
        where
            Bundle: DelegateComponent<BarProviderComponent>,
            <Bundle as DelegateComponent<
                BarProviderComponent,
            >>::Delegate: IsProviderFor<BarProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<BazProviderComponent> for App {
            type Delegate = DummyBaz;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<BazProviderComponent, __Context__, __Params__> for App
        where
            DummyBaz: IsProviderFor<BazProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<QuuxProviderComponent> for App {
            type Delegate = DummyBaz;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<QuuxProviderComponent, __Context__, __Params__> for App
        where
            DummyBaz: IsProviderFor<QuuxProviderComponent, __Context__, __Params__>,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<AreaCalculatorComponent, PathCons<Rectangle, __Wildcard__>>>
        for App {
            type Delegate = ShapeArea;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<AreaCalculatorComponent, PathCons<Rectangle, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            ShapeArea: IsProviderFor<
                PathCons<AreaCalculatorComponent, PathCons<Rectangle, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<AreaCalculatorComponent, PathCons<Circle, __Wildcard__>>>
        for App {
            type Delegate = ShapeArea;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<AreaCalculatorComponent, PathCons<Circle, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            ShapeArea: IsProviderFor<
                PathCons<AreaCalculatorComponent, PathCons<Circle, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<AreaCalculatorComponent, PathCons<Square, __Wildcard__>>>
        for App {
            type Delegate = ShapeArea;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<AreaCalculatorComponent, PathCons<Square, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            ShapeArea: IsProviderFor<
                PathCons<AreaCalculatorComponent, PathCons<Square, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            __Wildcard__,
        > DelegateComponent<PathCons<AreaCalculatorComponent, PathCons<Triangle, __Wildcard__>>>
        for App {
            type Delegate = ShapeArea;
        }
        impl<
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<AreaCalculatorComponent, PathCons<Triangle, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            ShapeArea: IsProviderFor<
                PathCons<AreaCalculatorComponent, PathCons<Triangle, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        impl<
            'a,
            T,
            __Wildcard__,
        > DelegateComponent<PathCons<AreaCalculatorComponent, PathCons<&'a T, __Wildcard__>>>
        for App {
            type Delegate = RefArea;
        }
        impl<
            'a,
            T,
            __Wildcard__,
            __Context__,
            __Params__,
        > IsProviderFor<
            PathCons<AreaCalculatorComponent, PathCons<&'a T, __Wildcard__>>,
            __Context__,
            __Params__,
        > for App
        where
            RefArea: IsProviderFor<
                PathCons<AreaCalculatorComponent, PathCons<&'a T, __Wildcard__>>,
                __Context__,
                __Params__,
            >,
        {}
        ")
    }
}

check_components! {
    App {
        AreaCalculatorComponent: [
            Rectangle,
            Circle,
            Square,
            Triangle,
            <'a> &'a Ellipse,
        ],
        BarProviderComponent,
        BazProviderComponent,
        QuuxProviderComponent,
    }
}
