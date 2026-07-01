//! Canonical expansion of `#[cgp_type]` for a simple abstract-type trait.
//!
//! `#[cgp_type]` is the dedicated macro for an abstract-type component: like
//! `#[cgp_component]` it emits the consumer trait, the provider trait (here the
//! provider name defaults to the type name plus `TypeProvider`), the `…Component`
//! marker, and the routing blanket impls — but it *additionally* emits the
//! `UseType<T>` blanket impl (and the `WithProvider` bridge), which is what lets
//! a context fix the abstract type by wiring the component to `UseType<Concrete>`.
//! This is the reference snapshot for that expansion; other files reuse
//! `#[cgp_type]` plainly.
//!
//! See docs/reference/macros/cgp_type.md and docs/concepts/abstract-types.md.

use cgp_macro_test_util::snapshot_cgp_type;

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
