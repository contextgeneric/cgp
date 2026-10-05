//! `#[cgp_component]` on a trait with an `async` method, stacked with
//! `#[async_trait]`.
//!
//! The forwarding impls keep the `async` signature and append `.await` to each
//! delegated call, and `#[async_trait]` is forwarded onto every generated trait
//! and impl, where it rewrites the trait declarations and leaves the impls alone.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/functions/derive/delegated_impls.md and
//! cgp-knowledge-base-fork/cgp/reference/macros/async_trait.md.

use cgp_fork::prelude::*;
use cgp_fork_macro_test_util::snapshot_cgp_component;

snapshot_cgp_component! {
    #[cgp_component(Fetcher)]
    #[async_trait]
    pub trait CanFetch {
        async fn fetch(&self, id: u32) -> String;
    }

    expand_can_fetch(output) {
        insta::assert_snapshot!(output, @"
        #[async_trait]
        pub trait CanFetch {
            async fn fetch(&self, id: u32) -> String;
        }
        #[async_trait]
        impl<__Context__> CanFetch for __Context__
        where
            __Context__: Fetcher<__Context__>,
        {
            async fn fetch(&self, id: u32) -> String {
                __Context__::fetch(self, id).await
            }
        }
        #[async_trait]
        pub trait Fetcher<__Context__>: IsProviderFor<FetcherComponent, __Context__, ()> {
            async fn fetch(__context__: &__Context__, id: u32) -> String;
        }
        #[async_trait]
        impl<__Provider__, __Context__> Fetcher<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<FetcherComponent>
                + IsProviderFor<FetcherComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                FetcherComponent,
            >>::Delegate: Fetcher<__Context__>,
        {
            async fn fetch(__context__: &__Context__, id: u32) -> String {
                <__Provider__ as DelegateComponent<
                    FetcherComponent,
                >>::Delegate::fetch(__context__, id)
                    .await
            }
        }
        pub struct FetcherComponent;
        #[async_trait]
        impl<__Context__> Fetcher<__Context__> for UseContext
        where
            __Context__: CanFetch,
        {
            async fn fetch(__context__: &__Context__, id: u32) -> String {
                __Context__::fetch(__context__, id).await
            }
        }
        impl<__Context__> IsProviderFor<FetcherComponent, __Context__, ()> for UseContext
        where
            __Context__: CanFetch,
        {}
        #[async_trait]
        impl<__Context__, __Components__, __Path__> Fetcher<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: Fetcher<__Context__>,
        {
            async fn fetch(__context__: &__Context__, id: u32) -> String {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::fetch(__context__, id)
                    .await
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<FetcherComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<FetcherComponent, __Context__, ()>
                + Fetcher<__Context__>,
        {}
        ")
    }
}
