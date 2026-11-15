//! The provider trait's generated doc line on a consumer trait that carries
//! `#[doc(hidden)]` and no doc text.
//!
//! `#[doc(hidden)]` adds no text, so the generated line naming the component key
//! opens the provider trait's doc without an empty line before it.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_component.md.

use cgp_macro_test_util::snapshot_cgp_component;

snapshot_cgp_component! {
    #[cgp_component(Hider)]
    #[doc(hidden)]
    pub trait CanHide {
        fn hide(&self);
    }

    expand_can_hide(output) {
        insta::assert_snapshot!(output, @"
        #[doc(hidden)]
        pub trait CanHide {
            fn hide(&self);
        }
        #[doc(hidden)]
        impl<__Context__> CanHide for __Context__
        where
            __Context__: Hider<__Context__>,
        {
            fn hide(&self) {
                __Context__::hide(self)
            }
        }
        #[doc(hidden)]
        ///Provider trait for [`CanHide`]. A provider imports this trait's component key, [`HiderComponent`], beside it.
        pub trait Hider<__Context__>: IsProviderFor<HiderComponent, __Context__, ()> {
            fn hide(__context__: &__Context__);
        }
        #[doc(hidden)]
        impl<__Provider__, __Context__> Hider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<HiderComponent>
                + IsProviderFor<HiderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<HiderComponent>>::Delegate: Hider<__Context__>,
        {
            fn hide(__context__: &__Context__) {
                <__Provider__ as DelegateComponent<HiderComponent>>::Delegate::hide(__context__)
            }
        }
        ///Component marker for [`CanHide`].
        pub struct HiderComponent;
        #[doc(hidden)]
        impl<__Context__> Hider<__Context__> for UseContext
        where
            __Context__: CanHide,
        {
            fn hide(__context__: &__Context__) {
                __Context__::hide(__context__)
            }
        }
        impl<__Context__> IsProviderFor<HiderComponent, __Context__, ()> for UseContext
        where
            __Context__: CanHide,
        {}
        #[doc(hidden)]
        impl<__Context__, __Components__, __Path__> Hider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: Hider<__Context__>,
        {
            fn hide(__context__: &__Context__) {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::hide(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<HiderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<HiderComponent, __Context__, ()> + Hider<__Context__>,
        {}
        ");
    }
}
