//! `DefaultImpls2`, the two-type-parameter member of the default-lookup family.
//!
//! `DefaultNamespace`, `DefaultImpls1`, and `DefaultImpls2` differ only in how many types
//! take part in the key. Nothing in the library or the rest of this suite exercised
//! `DefaultImpls2`, so this file establishes that it is genuinely reachable rather than a
//! declared-but-unusable trait — both halves of the round trip work, and neither needed a
//! new construct.
//!
//! Registration is the ordinary `#[default_impl]` attribute: the attribute takes an
//! arbitrary namespace trait path and appends the table parameter, so naming
//! `DefaultImpls2<Component, U>` emits
//! `impl<__Components__> DefaultImpls2<Component, U, __Components__> for Key`. The key
//! written before `in` becomes the impl's `Self`, and the types written inside the path
//! become the trait's leading parameters — which is the opposite way round from what the
//! parameter names `T`/`T1`/`T2` suggest.
//!
//! Consumption is the ordinary `for … in` loop, whose generated bound is
//! `T: DefaultImpls2<Component, U, App, Delegate = Provider>`. The loop variable is the
//! *key*, so the entry must mention `T` for it to be constrained.
//!
//! See cgp-knowledge-base/cgp/reference/traits/default_namespace.md.

use cgp::core::component::DefaultImpls2;
use cgp::prelude::*;

#[cgp_component(ShowImpl)]
pub trait CanShowPair<T, U> {
    fn show_pair(&self, first: &T, second: &U) -> String;
}

/// Registers `ShowPair` as the default for the key `String` under the two-type key
/// `(ShowImplComponent, u64)`.
#[cgp_impl(new ShowPair)]
#[default_impl(String in DefaultImpls2<ShowImplComponent, u64>)]
impl ShowImpl<String, u64> {
    fn show_pair(&self, first: &String, second: &u64) -> String {
        format!("{first}={second}")
    }
}

pub struct App;

delegate_components! {
    App {
        open ShowImplComponent;

        for <T, Provider> in DefaultImpls2<ShowImplComponent, u64> {
            @ShowImplComponent.T: Provider,
        }
    }
}

check_components! {
    App {
        ShowImplComponent: (String, u64),
    }
}

#[test]
fn test_the_two_type_default_resolves() {
    let shown = App.show_pair(&"answer".to_owned(), &42);

    assert_eq!(shown, "answer=42");
}
