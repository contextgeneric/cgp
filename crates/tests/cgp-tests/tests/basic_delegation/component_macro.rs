//! Canonical expansion of `#[cgp_component]` for a simple consumer trait.
//!
//! `#[cgp_component]` turns one consumer trait into the whole component bundle:
//! the consumer trait itself, the provider trait (with `Self` moved to a leading
//! `__Context__` parameter), the `…Component` marker, and the blanket impls that
//! route between them (plus the `UseContext` and `RedirectLookup` providers).
//! This is the reference snapshot for that expansion; other concepts reuse
//! `#[cgp_component]` without re-snapshotting it.
//!
//! See docs/reference/macros/cgp_component.md.

use cgp_macro_test_util::snapshot_cgp_component;

snapshot_cgp_component! {
    #[cgp_component(FooProvider)]
    pub trait CanDoFoo {
        fn foo(&self, value: u32) -> String;
    }

    expand_foo_component(output) {
        insta::assert_snapshot!(output, @"
        pub trait CanDoFoo {
            fn foo(&self, value: u32) -> String;
        }
        impl<__Context__> CanDoFoo for __Context__
        where
            __Context__: FooProvider<__Context__>,
        {
            fn foo(&self, value: u32) -> String {
                __Context__::foo(self, value)
            }
        }
        pub trait FooProvider<
            __Context__,
        >: IsProviderFor<FooProviderComponent, __Context__, ()> {
            fn foo(__context__: &__Context__, value: u32) -> String;
        }
        impl<__Provider__, __Context__> FooProvider<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<FooProviderComponent>
                + IsProviderFor<FooProviderComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                FooProviderComponent,
            >>::Delegate: FooProvider<__Context__>,
        {
            fn foo(__context__: &__Context__, value: u32) -> String {
                <__Provider__ as DelegateComponent<
                    FooProviderComponent,
                >>::Delegate::foo(__context__, value)
            }
        }
        pub struct FooProviderComponent;
        impl<__Context__> FooProvider<__Context__> for UseContext
        where
            __Context__: CanDoFoo,
        {
            fn foo(__context__: &__Context__, value: u32) -> String {
                __Context__::foo(__context__, value)
            }
        }
        impl<__Context__> IsProviderFor<FooProviderComponent, __Context__, ()> for UseContext
        where
            __Context__: CanDoFoo,
        {}
        impl<__Context__, __Components__, __Path__> FooProvider<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: FooProvider<__Context__>,
        {
            fn foo(__context__: &__Context__, value: u32) -> String {
                <__Components__ as DelegateComponent<
                    __Path__,
                >>::Delegate::foo(__context__, value)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<FooProviderComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<FooProviderComponent, __Context__, ()>
                + FooProvider<__Context__>,
        {}
        ")
    }
}
