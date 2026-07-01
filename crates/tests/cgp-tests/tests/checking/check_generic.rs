//! `check_components!` on a generic context with lifetimes and a `where` clause:
//! the leading generic list (`<'a, I>`) and the trailing `where I: Clone` are
//! carried onto the generated check trait's impls, and a check entry may use a
//! generic parameter as its component's parameter (`Component: &'a I`) and target a
//! component that is itself generic (`BarGetterAtComponent<I>`). This concept owns
//! the macro's expansion snapshot.
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
pub trait HasFooAt<I: Clone>: HasFooType {
    fn foo(&self, _tag: PhantomData<I>) -> &Self::Foo;
}

#[cgp_getter {
    name: BarGetterAtComponent<I>,
    provider: BarGetterAt,
}]
pub trait HasBarAt<I: Clone, J>: HasBarType {
    fn foo(&self, _tag: PhantomData<(I, J)>) -> &Self::Bar;
}

#[derive(HasField)]
pub struct Context {
    pub dummy: (),
}

// Incidental wiring so that `Context` can use the components being checked below;
// the `delegate_components!` expansion is snapshotted in `basic_delegation`, so we
// invoke it plainly here.
delegate_components! {
    Context {
        [
            FooTypeProviderComponent,
            BarTypeProviderComponent,
        ]:
            UseType<()>,
        [
            FooGetterAtComponent,
            <I> BarGetterAtComponent<I>,
        ]:
            UseField<Symbol!("dummy")>,
    }
}

snapshot_check_components! {
    check_components! {
        <'a, I> Context
        where
            I: Clone,
        {
            FooGetterAtComponent: &'a I,
            BarGetterAtComponent<I>: (I, &'a Index<0>),
        }
    }

    expand_check_context(output) {
        insta::assert_snapshot!(output, @"
        trait __CheckContext<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl<'a, I> __CheckContext<FooGetterAtComponent, &'a I> for Context
        where
            I: Clone,
        {}
        impl<'a, I> __CheckContext<BarGetterAtComponent<I>, (I, &'a Index<0>)> for Context
        where
            I: Clone,
        {}
        ")
    }
}
