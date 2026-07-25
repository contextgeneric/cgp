//! `delegate_and_check_components!` on a delegation key that carries its own
//! generic parameters (`<I> BarGetterAtComponent<I>`): the key's generics are
//! threaded onto the derived check impl so it binds them
//! (`impl<I> __CanUse...<BarGetterAtComponent<I>, ...> for Context {}`) rather than
//! referencing them unbound. Exercises both a `#[check_params(...)]` value that
//! itself mentions the key generic and an array key mixing a generic and a
//! non-generic component. This concept owns the macro's expansion snapshot.
//!
//! See cgp-knowledge-base/cgp/reference/macros/delegate_and_check_components.md and
//! cgp-knowledge-base/cgp/reference/traits/can_use_component.md.

use core::marker::PhantomData;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_delegate_and_check_components;

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
    name: BarGetterAtComponent<I>,
    provider: BarGetterAt,
}]
pub trait HasBarAt<I, J>: HasBarType {
    fn bar(&self, _tag: PhantomData<(I, J)>) -> &Self::Bar;
}

#[derive(HasField)]
pub struct Context {
    pub dummy: (),
}

snapshot_delegate_and_check_components! {
    delegate_and_check_components! {
        Context {
            [
                FooTypeProviderComponent,
                BarTypeProviderComponent,
            ]:
                UseType<()>,

            #[check_params(
                (I, Index<0>),
            )]
            <I> BarGetterAtComponent<I>: UseField<Symbol!("dummy")>,
        }
    }

    expand_context(output) {
        insta::assert_snapshot!(output, @r#"
        impl DelegateComponent<FooTypeProviderComponent> for Context {
            type Delegate = UseType<()>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<FooTypeProviderComponent, __Context__, __Params__> for Context
        where
            UseType<()>: IsProviderFor<FooTypeProviderComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<BarTypeProviderComponent> for Context {
            type Delegate = UseType<()>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<BarTypeProviderComponent, __Context__, __Params__> for Context
        where
            UseType<()>: IsProviderFor<BarTypeProviderComponent, __Context__, __Params__>,
        {}
        impl<I> DelegateComponent<BarGetterAtComponent<I>> for Context {
            type Delegate = UseField<Symbol!("dummy")>;
        }
        impl<
            I,
            __Context__,
            __Params__,
        > IsProviderFor<BarGetterAtComponent<I>, __Context__, __Params__> for Context
        where
            UseField<
                Symbol!("dummy"),
            >: IsProviderFor<BarGetterAtComponent<I>, __Context__, __Params__>,
        {}
        trait __CanUseContext<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CanUseContext<FooTypeProviderComponent, ()> for Context {}
        impl __CanUseContext<BarTypeProviderComponent, ()> for Context {}
        impl<I> __CanUseContext<BarGetterAtComponent<I>, (I, Index<0>)> for Context {}
        "#)
    }
}
