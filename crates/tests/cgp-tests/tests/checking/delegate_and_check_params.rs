//! `delegate_and_check_components!` on components that carry generic parameters:
//! `#[check_params(...)]` supplies the parameter tuples to check each entry with,
//! an array key wires several components to one provider, and a block-level
//! `#[check_params(...)]` (on the array) is checked in addition to each entry's own
//! `#[check_params(...)]`. This concept owns the macro's expansion snapshot.
//!
//! See docs/reference/macros/delegate_and_check_components.md and
//! docs/reference/traits/can_use_component.md.

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

snapshot_delegate_and_check_components! {
    delegate_and_check_components! {
        Context {
            [
                FooTypeProviderComponent,
                BarTypeProviderComponent,
            ]:
                UseType<()>,

            #[check_params(
                (Index<5>, Index<6>),
                (Index<7>, Index<8>),
            )]
            [
                #[check_params(
                    Index<0>,
                    Index<1>,
                )]
                FooGetterAtComponent,

                #[check_params(
                    (Index<0>, Index<1>),
                    (Index<1>, Index<0>),
                )]
                BarGetterAtComponent,
            ]:
                UseField<Symbol!("dummy")>,
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
        impl DelegateComponent<FooGetterAtComponent> for Context {
            type Delegate = UseField<Symbol!("dummy")>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<FooGetterAtComponent, __Context__, __Params__> for Context
        where
            UseField<
                Symbol!("dummy"),
            >: IsProviderFor<FooGetterAtComponent, __Context__, __Params__>,
        {}
        impl DelegateComponent<BarGetterAtComponent> for Context {
            type Delegate = UseField<Symbol!("dummy")>;
        }
        impl<
            __Context__,
            __Params__,
        > IsProviderFor<BarGetterAtComponent, __Context__, __Params__> for Context
        where
            UseField<
                Symbol!("dummy"),
            >: IsProviderFor<BarGetterAtComponent, __Context__, __Params__>,
        {}
        trait __CanUseContext<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CanUseContext<FooTypeProviderComponent, ()> for Context {}
        impl __CanUseContext<BarTypeProviderComponent, ()> for Context {}
        impl __CanUseContext<FooGetterAtComponent, (Index<5>, Index<6>)> for Context {}
        impl __CanUseContext<FooGetterAtComponent, (Index<7>, Index<8>)> for Context {}
        impl __CanUseContext<FooGetterAtComponent, Index<0>> for Context {}
        impl __CanUseContext<FooGetterAtComponent, Index<1>> for Context {}
        impl __CanUseContext<BarGetterAtComponent, (Index<5>, Index<6>)> for Context {}
        impl __CanUseContext<BarGetterAtComponent, (Index<7>, Index<8>)> for Context {}
        impl __CanUseContext<BarGetterAtComponent, (Index<0>, Index<1>)> for Context {}
        impl __CanUseContext<BarGetterAtComponent, (Index<1>, Index<0>)> for Context {}
        "#)
    }
}
