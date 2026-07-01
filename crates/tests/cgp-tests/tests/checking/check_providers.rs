//! `check_components!` with `#[check_providers(...)]`: instead of checking that the
//! context can use each component through its wiring, this form checks that each
//! *listed provider* is a valid provider of each component for the context. The
//! generated check trait therefore supertraits `IsProviderFor<_, Context, _>` and
//! is implemented `for` each provider (here two `UseField` variants), rather than
//! `for` the context. This concept owns the macro's expansion snapshot.
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

// Incidental wiring: the getter providers depend on `Context: HasFooType` /
// `HasBarType`, so the abstract types must be wired before the providers can be
// checked. The `delegate_components!` expansion is snapshotted in
// `basic_delegation`, so we invoke it plainly here.
delegate_components! {
    Context {
        [
            FooTypeProviderComponent,
            BarTypeProviderComponent,
        ]:
            UseType<()>,
    }
}

snapshot_check_components! {
    check_components! {
        #[check_trait(CanUseDummyField)]
        #[check_providers(
            UseField<Symbol!("dummy")>,
            UseField<Symbol!("extra_dummy")>,
        )]
        Context {
            FooGetterAtComponent: [
                Index<0>,
                Index<1>,
            ],
            FooGetterAtComponent:
                Index<3>,
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
        trait CanUseDummyField<
            __Component__,
            __Params__: ?Sized,
        >: IsProviderFor<__Component__, Context, __Params__> {}
        impl CanUseDummyField<FooGetterAtComponent, Index<0>> for UseField<Symbol!("dummy")> {}
        impl CanUseDummyField<FooGetterAtComponent, Index<0>>
        for UseField<Symbol!("extra_dummy")> {}
        impl CanUseDummyField<FooGetterAtComponent, Index<1>> for UseField<Symbol!("dummy")> {}
        impl CanUseDummyField<FooGetterAtComponent, Index<1>>
        for UseField<Symbol!("extra_dummy")> {}
        impl CanUseDummyField<FooGetterAtComponent, Index<3>> for UseField<Symbol!("dummy")> {}
        impl CanUseDummyField<FooGetterAtComponent, Index<3>>
        for UseField<Symbol!("extra_dummy")> {}
        impl CanUseDummyField<BarGetterAtComponent, (Index<0>, Index<1>)>
        for UseField<Symbol!("dummy")> {}
        impl CanUseDummyField<BarGetterAtComponent, (Index<0>, Index<1>)>
        for UseField<Symbol!("extra_dummy")> {}
        impl CanUseDummyField<BarGetterAtComponent, (Index<1>, Index<0>)>
        for UseField<Symbol!("dummy")> {}
        impl CanUseDummyField<BarGetterAtComponent, (Index<1>, Index<0>)>
        for UseField<Symbol!("extra_dummy")> {}
        impl CanUseDummyField<BarGetterAtComponent, (Index<3>, Index<4>)>
        for UseField<Symbol!("dummy")> {}
        impl CanUseDummyField<BarGetterAtComponent, (Index<3>, Index<4>)>
        for UseField<Symbol!("extra_dummy")> {}
        impl CanUseDummyField<FooGetterAtComponent, (Index<5>, Index<6>)>
        for UseField<Symbol!("dummy")> {}
        impl CanUseDummyField<FooGetterAtComponent, (Index<5>, Index<6>)>
        for UseField<Symbol!("extra_dummy")> {}
        impl CanUseDummyField<FooGetterAtComponent, (Index<7>, Index<8>)>
        for UseField<Symbol!("dummy")> {}
        impl CanUseDummyField<FooGetterAtComponent, (Index<7>, Index<8>)>
        for UseField<Symbol!("extra_dummy")> {}
        impl CanUseDummyField<BarGetterAtComponent, (Index<5>, Index<6>)>
        for UseField<Symbol!("dummy")> {}
        impl CanUseDummyField<BarGetterAtComponent, (Index<5>, Index<6>)>
        for UseField<Symbol!("extra_dummy")> {}
        impl CanUseDummyField<BarGetterAtComponent, (Index<7>, Index<8>)>
        for UseField<Symbol!("dummy")> {}
        impl CanUseDummyField<BarGetterAtComponent, (Index<7>, Index<8>)>
        for UseField<Symbol!("extra_dummy")> {}
        "#)
    }
}
