//! Standalone `check_components!`: a compile-time assertion that a context can use
//! a set of components, independent of the wiring. Exercises multiple check blocks
//! in one invocation, each renamed with `#[check_trait(...)]` so they do not clash;
//! per-entry parameter lists (`Component: [P, P]` and `Component: P`) for
//! generic-parameter components; and an array key checked against a parameter list.
//! The wiring itself is set up separately with a plain `delegate_components!`.
//! This concept owns the macro's expansion snapshot.
//!
//! See docs/reference/macros/check_components.md and
//! docs/reference/traits/can_use_component.md.

use core::marker::PhantomData;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_check_components;

#[cgp_type]
pub trait HasFooType {
    type Foo;
}

#[cgp_type]
pub trait HasBarType {
    type Bar;
}

#[cgp_getter {
    provider: FooGetterAt,
}]
pub trait HasFooAt<I>: HasFooType {
    fn foo(&self, _tag: PhantomData<I>) -> &Self::Foo;
}

#[cgp_getter {
    provider: BarGetterAt,
}]
pub trait HasBarAt<I, J>: HasBarType {
    fn foo(&self, _tag: PhantomData<(I, J)>) -> &Self::Bar;
}

#[derive(HasField)]
pub struct Context {
    pub dummy: (),
    pub extra_dummy: (),
}

// Incidental wiring so that `Context` can actually use the components being
// checked below; the `delegate_components!` expansion is snapshotted in
// `basic_delegation`, so we invoke it plainly here.
delegate_components! {
    Context {
        [
            FooTypeProviderComponent,
            BarTypeProviderComponent,
        ]:
            UseType<()>,
        [
            FooGetterAtComponent,
            BarGetterAtComponent,
        ]:
            UseField<Symbol!("dummy")>,
    }
}

snapshot_check_components! {
    check_components! {
        #[check_trait(CanUseContext)]
        Context {
            FooTypeProviderComponent,
            BarTypeProviderComponent,
            FooGetterAtComponent: [
                Index<0>,
                Index<1>,
            ],
            FooGetterAtComponent:
                Index<3>,
        }

        #[check_trait(CanUseContext2)]
        Context {
            BarGetterAtComponent: [
                (Index<0>, Index<1>),
                (Index<1>, Index<0>),
            ],
            BarGetterAtComponent:
                (Index<3>, Index<4>),
            [
                FooGetterAtComponent,
                BarGetterAtComponent,
            ]: [
                (Index<5>, Index<6>),
                (Index<7>, Index<8>),
            ]
        }
    }

    expand_check_context(output) {
        insta::assert_snapshot!(output, @r#"
        trait CanUseContext<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl CanUseContext<FooTypeProviderComponent, ()> for Context {}
        impl CanUseContext<BarTypeProviderComponent, ()> for Context {}
        impl CanUseContext<FooGetterAtComponent, Index<0>> for Context {}
        impl CanUseContext<FooGetterAtComponent, Index<1>> for Context {}
        impl CanUseContext<FooGetterAtComponent, Index<3>> for Context {}
        trait CanUseContext2<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl CanUseContext2<BarGetterAtComponent, (Index<0>, Index<1>)> for Context {}
        impl CanUseContext2<BarGetterAtComponent, (Index<1>, Index<0>)> for Context {}
        impl CanUseContext2<BarGetterAtComponent, (Index<3>, Index<4>)> for Context {}
        impl CanUseContext2<FooGetterAtComponent, (Index<5>, Index<6>)> for Context {}
        impl CanUseContext2<FooGetterAtComponent, (Index<7>, Index<8>)> for Context {}
        impl CanUseContext2<BarGetterAtComponent, (Index<5>, Index<6>)> for Context {}
        impl CanUseContext2<BarGetterAtComponent, (Index<7>, Index<8>)> for Context {}
        "#)
    }
}
