//! Where a user's doc comment lands on a `#[cgp_component]`.
//!
//! The consumer trait's doc is copied onto every item the macro derives, so the
//! provider trait and the blanket impls carry it too, and the marker struct
//! carries a generated line naming the consumer trait. The two never meet on one
//! item: the marker struct has only the generated line, and the derived items have
//! only the user's doc.
//!
//! See cgp-knowledge-base/cgp/implementation/README.md#doc-comments-on-generated-items.

// The doc comment inside the invocation is consumed by the macro, which emits it
// on the generated items, so rustdoc never sees it here.
#![allow(unused_doc_comments)]

use cgp_macro_test_util::snapshot_cgp_component;

snapshot_cgp_component! {
    #[cgp_component(Greeter)]
    /// The user's doc for the consumer trait.
    pub trait CanGreet {
        /// The user's doc for the method.
        fn greet(&self) -> String;
    }

    expand_can_greet(output) {
        insta::assert_snapshot!(output, @"
        /// The user's doc for the consumer trait.
        pub trait CanGreet {
            /// The user's doc for the method.
            fn greet(&self) -> String;
        }
        /// The user's doc for the consumer trait.
        impl<__Context__> CanGreet for __Context__
        where
            __Context__: Greeter<__Context__>,
        {
            fn greet(&self) -> String {
                __Context__::greet(self)
            }
        }
        /// The user's doc for the consumer trait.
        pub trait Greeter<__Context__>: IsProviderFor<GreeterComponent, __Context__, ()> {
            /// The user's doc for the method.
            fn greet(__context__: &__Context__) -> String;
        }
        /// The user's doc for the consumer trait.
        impl<__Provider__, __Context__> Greeter<__Context__> for __Provider__
        where
            __Provider__: DelegateComponent<GreeterComponent>
                + IsProviderFor<GreeterComponent, __Context__, ()>,
            <__Provider__ as DelegateComponent<
                GreeterComponent,
            >>::Delegate: Greeter<__Context__>,
        {
            fn greet(__context__: &__Context__) -> String {
                <__Provider__ as DelegateComponent<
                    GreeterComponent,
                >>::Delegate::greet(__context__)
            }
        }
        ///Component marker for [`CanGreet`].
        pub struct GreeterComponent;
        /// The user's doc for the consumer trait.
        impl<__Context__> Greeter<__Context__> for UseContext
        where
            __Context__: CanGreet,
        {
            fn greet(__context__: &__Context__) -> String {
                __Context__::greet(__context__)
            }
        }
        impl<__Context__> IsProviderFor<GreeterComponent, __Context__, ()> for UseContext
        where
            __Context__: CanGreet,
        {}
        /// The user's doc for the consumer trait.
        impl<__Context__, __Components__, __Path__> Greeter<__Context__>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<__Path__>>::Delegate: Greeter<__Context__>,
        {
            fn greet(__context__: &__Context__) -> String {
                <__Components__ as DelegateComponent<__Path__>>::Delegate::greet(__context__)
            }
        }
        impl<
            __Context__,
            __Components__,
            __Path__,
        > IsProviderFor<GreeterComponent, __Context__, ()>
        for RedirectLookup<__Components__, __Path__>
        where
            __Components__: DelegateComponent<__Path__>,
            <__Components__ as DelegateComponent<
                __Path__,
            >>::Delegate: IsProviderFor<GreeterComponent, __Context__, ()>
                + Greeter<__Context__>,
        {}
        ")
    }
}
