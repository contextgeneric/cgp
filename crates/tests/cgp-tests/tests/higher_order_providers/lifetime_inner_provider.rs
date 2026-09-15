//! A higher-order provider for a component that carries a **lifetime**, where the
//! inner-provider bound gets no `IsProviderFor` counterpart.
//!
//! Deriving a provider's `IsProviderFor` impl normally augments a bound naming the
//! same provider trait — the inner-provider bound of a higher-order provider — with
//! its `IsProviderFor` counterpart, so a dependency missing inside the inner
//! provider still propagates outward. That rewrite reads the provider trait's
//! *first* generic argument as the context, and Rust requires lifetime arguments to
//! come first, so on a lifetime-carrying component the first argument is a lifetime
//! and the rewrite finds no context to build the counterpart from. It then leaves
//! the bound alone rather than emitting something wrong.
//!
//! The snapshot pins that: the outer `IsProviderFor` impl is produced correctly,
//! with the lifetime lifted into `Life<'a>` in the params tuple, while the
//! `Inner: ReferenceGetter<'a, Context>` bound is copied verbatim with no
//! `IsProviderFor<ReferenceGetterComponent, Context, (Life<'a>)>` beside it —
//! compare `use_provider_impl`, where the component has no lifetime and the
//! counterpart *is* added. The consequence is a weaker diagnostic rather than
//! broken code, and it is recorded under Known issues in the reference.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_provider.md (Known issues) and
//! cgp-knowledge-base/cgp/reference/types/life.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_new_provider;

#[cgp_component(ReferenceGetter)]
pub trait HasReference<'a> {
    fn get_reference(&self) -> &'a u32;
}

snapshot_cgp_new_provider! {
    #[cgp_new_provider]
    impl<'a, Context, Inner> ReferenceGetter<'a, Context> for ForwardReference<Inner>
    where
        Inner: ReferenceGetter<'a, Context>,
    {
        fn get_reference(context: &Context) -> &'a u32 {
            Inner::get_reference(context)
        }
    }

    expand_forward_reference(output) {
        insta::assert_snapshot!(output, @"
        impl<'a, Context, Inner> ReferenceGetter<'a, Context> for ForwardReference<Inner>
        where
            Inner: ReferenceGetter<'a, Context>,
        {
            fn get_reference(context: &Context) -> &'a u32 {
                Inner::get_reference(context)
            }
        }
        impl<'a, Context, Inner> IsProviderFor<ReferenceGetterComponent, Context, (Life<'a>)>
        for ForwardReference<Inner>
        where
            Inner: ReferenceGetter<'a, Context>,
        {}
        pub struct ForwardReference<Inner>(pub ::core::marker::PhantomData<Inner>);
        ")
    }
}

pub struct App<'a> {
    pub value: &'a u32,
}

#[cgp_impl(new GetReference)]
impl<'a> ReferenceGetter<'a> for App<'a> {
    fn get_reference(&self) -> &'a u32 {
        self.value
    }
}

delegate_components! {
    <'a> App<'a> {
        ReferenceGetterComponent:
            ForwardReference<GetReference>,
    }
}

#[test]
fn test_lifetime_inner_provider() {
    let value = 42;
    let app = App { value: &value };

    assert_eq!(app.get_reference(), &42);
}
