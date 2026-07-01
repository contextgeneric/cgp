//! `delegate_components!` array-key form: several component keys mapping to one
//! provider in a single entry.
//!
//! An array key `[A, B]: Provider` expands to one `DelegateComponent` +
//! `IsProviderFor` impl pair per key, all pointing at the same provider.
//!
//! See docs/reference/macros/delegate_components.md.

use cgp::prelude::DelegateComponent;
use cgp_macro_test_util::snapshot_delegate_components;

pub struct FooKey;
pub struct FooValue;
pub struct BarKey;
pub struct BarValue;
pub struct BazKey;

pub struct Components;

snapshot_delegate_components! {
    delegate_components! {
        Components {
            FooKey: FooValue,
            [
                BarKey,
                BazKey,
            ]:
                BarValue,
        }
    }

    expand_components(output) {
        insta::assert_snapshot!(output, @"
        impl DelegateComponent<FooKey> for Components {
            type Delegate = FooValue;
        }
        impl<__Context__, __Params__> IsProviderFor<FooKey, __Context__, __Params__>
        for Components
        where
            FooValue: IsProviderFor<FooKey, __Context__, __Params__>,
        {}
        impl DelegateComponent<BarKey> for Components {
            type Delegate = BarValue;
        }
        impl<__Context__, __Params__> IsProviderFor<BarKey, __Context__, __Params__>
        for Components
        where
            BarValue: IsProviderFor<BarKey, __Context__, __Params__>,
        {}
        impl DelegateComponent<BazKey> for Components {
            type Delegate = BarValue;
        }
        impl<__Context__, __Params__> IsProviderFor<BazKey, __Context__, __Params__>
        for Components
        where
            BarValue: IsProviderFor<BazKey, __Context__, __Params__>,
        {}
        ")
    }
}

pub trait CheckDelegates:
    DelegateComponent<FooKey, Delegate = FooValue>
    + DelegateComponent<BarKey, Delegate = BarValue>
    + DelegateComponent<BazKey, Delegate = BarValue>
{
}

impl CheckDelegates for Components {}
