//! `#[cgp_type]` where the abstract type carries a *self-referential* bound
//! (`type Scalar: Mul<Output = Self::Scalar> + Clone`).
//!
//! This is a distinct `#[cgp_type]` expansion variant: the bound is threaded onto
//! the provider trait's associated type and, notably, onto the generated
//! `UseType<Scalar>` and `WithProvider` impls as a `where Scalar: Mul<Output =
//! Scalar> + Clone` clause (with `Self::Scalar` rewritten to the free `Scalar`
//! parameter). The `delegate_components!` wiring and its check are incidental
//! scaffolding and use the plain macros.
//!
//! See docs/reference/macros/cgp_type.md and docs/concepts/abstract-types.md.

use core::ops::Mul;

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_type;

snapshot_cgp_type! {
    #[cgp_type]
    pub trait HasScalarType {
        type Scalar: Mul<Output = Self::Scalar> + Clone;
    }

    expand_has_scalar_type(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasScalarType {
            type Scalar: Mul<Output = Self::Scalar> + Clone;
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
            type Scalar: Mul<Output = Self::Scalar> + Clone;
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
        impl<Scalar, __Context__> ScalarTypeProvider<__Context__> for UseType<Scalar>
        where
            Scalar: Mul<Output = Scalar> + Clone,
        {
            type Scalar = Scalar;
        }
        impl<Scalar, __Context__> IsProviderFor<ScalarTypeProviderComponent, __Context__, ()>
        for UseType<Scalar>
        where
            Scalar: Mul<Output = Scalar> + Clone,
        {}
        impl<__Provider__, Scalar, __Context__> ScalarTypeProvider<__Context__>
        for WithProvider<__Provider__>
        where
            Scalar: Mul<Output = Scalar> + Clone,
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
            Scalar: Mul<Output = Scalar> + Clone,
            __Provider__: TypeProvider<__Context__, ScalarTypeProviderComponent, Type = Scalar>,
        {}
        ")
    }
}

pub struct App;

delegate_components! {
    App {
        ScalarTypeProviderComponent:
            UseType<f64>,
    }
}

check_components! {
    App {
        ScalarTypeProviderComponent,
    }
}
