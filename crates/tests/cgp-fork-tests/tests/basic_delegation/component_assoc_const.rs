//! `#[cgp_component]` on a trait carrying an associated const beside a method.
//!
//! Every forwarding impl re-exposes the const as a projection through the trait
//! on the other side: the consumer blanket impl reads it from the provider trait,
//! the provider blanket impl from the delegate's provider trait, and `UseContext`
//! from the consumer trait. The context below reads the const through wiring.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/functions/derive/delegated_impls.md and
//! cgp-knowledge-base-fork/cgp/reference/macros/cgp_component.md.

use cgp_fork::prelude::*;
use cgp_fork_macro_test_util::snapshot_cgp_component;

snapshot_cgp_component! {
    #[cgp_component(LimitProvider)]
    pub trait HasLimit {
        const LIMIT: u32;

        fn limit(&self) -> u32;
    }

    expand_has_limit(output) {
        insta::assert_snapshot!(output, @"
        pub trait HasLimit {
            const LIMIT: u32;
            fn limit(&self) -> u32;
        }
        impl<__Context__> HasLimit for __Context__
        where
            __Context__: LimitProvider<__Context__>,
        {
            const LIMIT: u32 = <__Context__ as LimitProvider<__Context__>>::LIMIT;
            fn limit(&self) -> u32 {
                __Context__::limit(self)
            }
        }
        pub trait LimitProvider<
            __Context__,
        >: IsProviderFor<LimitProviderComponent, __Context__, ()> {
            const LIMIT: u32;
            fn limit(__context__: &__Context__) -> u32;
        }
        impl<__Provider__, __Context__> LimitProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<LimitProviderComponent>
                + IsProviderFor<LimitProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                LimitProviderComponent,
            >>::Delegate: LimitProvider<__Context__>,
        {
            const LIMIT: u32 = <<__Provider__ as DelegateComponent<
                LimitProviderComponent,
            >>::Delegate as LimitProvider<__Context__>>::LIMIT;
            fn limit(__context__: &__Context__) -> u32 {
                <__Provider__ as DelegateComponent<
                    LimitProviderComponent,
                >>::Delegate::limit(__context__)
            }
        }
        pub struct LimitProviderComponent;
        impl<__Context__> LimitProvider<__Context__> for UseContext
        where
            __Context__: HasLimit,
        {
            const LIMIT: u32 = <__Context__ as HasLimit>::LIMIT;
            fn limit(__context__: &__Context__) -> u32 {
                __Context__::limit(__context__)
            }
        }
        impl<__Context__> IsProviderFor<LimitProviderComponent, __Context__, ()> for UseContext
        where
            __Context__: HasLimit,
        {}
        impl<__Context__, __Components__, __Path__> LimitProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: LimitProvider<__Context__>,
        {
            const LIMIT: u32 = <<__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate as LimitProvider<__Context__>>::LIMIT;
            fn limit(__context__: &__Context__) -> u32 {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::limit(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<LimitProviderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<LimitProviderComponent, __Context__, ()>
                + LimitProvider<__Context__>,
        {}
        ")
    }
}

#[cgp_impl(new TenLimit)]
impl LimitProvider {
    const LIMIT: u32 = 10;

    fn limit(&self) -> u32 {
        10
    }
}

pub struct App;

delegate_components! {
    App {
        LimitProviderComponent: TenLimit,
    }
}

#[test]
fn test_component_assoc_const() {
    assert_eq!(<App as HasLimit>::LIMIT, 10);
    assert_eq!(App.limit(), 10);
}
