//! Dispatch methods whose signatures elide lifetimes, at any depth.
//!
//! The macro names each elided lifetime the way the compiler's elision rules
//! read it: every elided input lifetime is distinct, and an elided output
//! lifetime is the receiver's, or, for a by-value `self`, the one lifetime the
//! arguments use. So a borrow returned from `&self` outlives a shorter-lived
//! argument, as the trait's own signature promises. References nested inside
//! other types, such as `Option<&str>`, are named too, while a `Fn(&str)` bound
//! keeps the lifetime it binds itself.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_auto_dispatch.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_auto_dispatch;

pub struct Circle {
    pub name: String,
}

pub struct Square {
    pub name: String,
}

#[derive(CgpData)]
pub enum Shape {
    Circle(Circle),
    Square(Square),
}

snapshot_cgp_auto_dispatch! {
    #[cgp_auto_dispatch]
    pub trait CanLabel {
        fn label(&self, suffix: &str) -> &str;
    }

    expand_distinct_elided_lifetimes(output) {
        insta::assert_snapshot!(output, @"
        pub trait CanLabel {
            fn label(&self, suffix: &str) -> &str;
        }
        impl<__Variants__> CanLabel for __Variants__
        where
            MatchFirstWithValueHandlersRef<
                ComputeLabel,
            >: for<'__a__, '__a1__> Computer<
                (),
                (),
                (&'__a__ __Variants__, (&'__a1__ str)),
                Output = &'__a__ str,
            >,
            __Variants__: HasExtractor,
        {
            fn label(&self, arg_0: &str) -> &str {
                <MatchFirstWithValueHandlersRef<
                    ComputeLabel,
                > as Computer<
                    _,
                    _,
                    _,
                >>::compute(&(), ::core::marker::PhantomData::<()>, (self, (arg_0)))
            }
        }
        fn __compute_label__<'__a__, '__a1__, __Variants__: CanLabel>(
            __Variants__: &'__a__ __Variants__,
            (arg_0): (&'__a1__ str),
        ) -> &'__a__ str {
            __Variants__.label(arg_0)
        }
        impl<
            '__a__,
            '__a1__,
            __Variants__: CanLabel,
            __Context__,
            __Code__,
        > Computer<__Context__, __Code__, (&'__a__ __Variants__, (&'__a1__ str))>
        for ComputeLabel {
            type Output = &'__a__ str;
            fn compute(
                _context: &__Context__,
                _code: ::core::marker::PhantomData<__Code__>,
                (arg_0, arg_1): (&'__a__ __Variants__, (&'__a1__ str)),
            ) -> Self::Output {
                __compute_label__(arg_0, arg_1)
            }
        }
        impl<
            '__a__,
            '__a1__,
            __Variants__: CanLabel,
            __Context__,
            __Code__,
        > IsProviderFor<
            ComputerComponent,
            __Context__,
            (__Code__, (&'__a__ __Variants__, (&'__a1__ str))),
        > for ComputeLabel {}
        pub struct ComputeLabel;
        impl DelegateComponent<ComputerRefComponent> for ComputeLabel
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<ComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<ComputerRefComponent, __Context__, __Params__> for ComputeLabel
        where
            PromoteComputer<Self>: DelegateComponent<ComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                ComputerRefComponent,
            >>::Delegate: IsProviderFor<ComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerComponent> for ComputeLabel
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<TryComputerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerComponent, __Context__, __Params__> for ComputeLabel
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerComponent,
            >>::Delegate: IsProviderFor<TryComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<TryComputerRefComponent> for ComputeLabel
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<TryComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<TryComputerRefComponent, __Context__, __Params__> for ComputeLabel
        where
            PromoteComputer<Self>: DelegateComponent<TryComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                TryComputerRefComponent,
            >>::Delegate: IsProviderFor<TryComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerComponent> for ComputeLabel
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<AsyncComputerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerComponent, __Context__, __Params__> for ComputeLabel
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerComponent,
            >>::Delegate: IsProviderFor<AsyncComputerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<AsyncComputerRefComponent> for ComputeLabel
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<AsyncComputerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__> for ComputeLabel
        where
            PromoteComputer<Self>: DelegateComponent<AsyncComputerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                AsyncComputerRefComponent,
            >>::Delegate: IsProviderFor<AsyncComputerRefComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerComponent> for ComputeLabel
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<HandlerComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerComponent, __Context__, __Params__> for ComputeLabel
        where
            PromoteComputer<Self>: DelegateComponent<HandlerComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                HandlerComponent,
            >>::Delegate: IsProviderFor<HandlerComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<HandlerRefComponent> for ComputeLabel
        where
            PromoteComputer<Self>: DelegateComponent<HandlerRefComponent>,
        {
            type Delegate = <PromoteComputer<
                Self,
            > as DelegateComponent<HandlerRefComponent>>::Delegate;
        }
        impl<
            __Context__,
            __Params__: ?Sized,
        > IsProviderFor<HandlerRefComponent, __Context__, __Params__> for ComputeLabel
        where
            PromoteComputer<Self>: DelegateComponent<HandlerRefComponent>,
            <PromoteComputer<
                Self,
            > as DelegateComponent<
                HandlerRefComponent,
            >>::Delegate: IsProviderFor<HandlerRefComponent, __Context__, __Params__>,
        {}
        ")
    }
}

#[cgp_auto_dispatch]
pub trait CanFind {
    fn find(&self, key: Option<&str>) -> Option<&str>;
}

#[cgp_auto_dispatch]
pub trait CanPick {
    fn pick(self, fallback: &str) -> &str;
}

#[cgp_auto_dispatch]
pub trait CanMeasure {
    fn measure(&self, measure: &dyn Fn(&str) -> usize) -> usize;
}

impl CanLabel for Circle {
    fn label(&self, _suffix: &str) -> &str {
        &self.name
    }
}

impl CanLabel for Square {
    fn label(&self, _suffix: &str) -> &str {
        &self.name
    }
}

impl CanFind for Circle {
    fn find(&self, key: Option<&str>) -> Option<&str> {
        key.map(|_| self.name.as_str())
    }
}

impl CanFind for Square {
    fn find(&self, _key: Option<&str>) -> Option<&str> {
        None
    }
}

impl CanPick for Circle {
    fn pick(self, fallback: &str) -> &str {
        fallback
    }
}

impl CanPick for Square {
    fn pick(self, fallback: &str) -> &str {
        fallback
    }
}

impl CanMeasure for Circle {
    fn measure(&self, measure: &dyn Fn(&str) -> usize) -> usize {
        measure(&self.name)
    }
}

impl CanMeasure for Square {
    fn measure(&self, measure: &dyn Fn(&str) -> usize) -> usize {
        measure(&self.name)
    }
}

#[test]
fn test_elided_lifetimes() {
    let shape = Shape::Circle(Circle {
        name: "circle".to_owned(),
    });

    // The returned borrow comes from `shape`, so it outlives `suffix`.
    let label = {
        let suffix = String::from("!");
        shape.label(&suffix)
    };
    assert_eq!(label, "circle");

    assert_eq!(shape.find(Some("key")), Some("circle"));
    assert_eq!(shape.measure(&|name| name.len()), 6);
    assert_eq!(
        Shape::Square(Square {
            name: "square".to_owned()
        })
        .pick("fallback"),
        "fallback"
    );
}
