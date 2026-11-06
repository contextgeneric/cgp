//! A `#[cgp_component]` method with a default body may take a `mut` parameter or a `mut self`
//! receiver.
//!
//! The generated impls forward each argument by name, so a parameter written `mut count` is
//! forwarded as `count`, and the forwarding impls leave the `mut` off their own signatures, where
//! nothing mutates it; a `mut self` receiver is forwarded as `self` the same way. The default body
//! keeps its `mut`: an empty provider inherits the body, and a provider that writes the method may
//! declare the parameter either way. rustc's `unused_mut` lint, which the clippy runs deny, guards
//! the forwarding signatures.
//!
//! See cgp-knowledge-base/cgp/implementation/functions/derive/delegated_impls.md (Rejected
//! parameters) and cgp-knowledge-base/cgp/reference/macros/cgp_component.md.

use cgp::prelude::*;

#[cgp_component(Counter)]
pub trait CanCount {
    fn count_up(&self, mut count: u32, steps: u32) -> u32 {
        for _ in 0..steps {
            count += 1;
        }
        count
    }
}

#[cgp_component(Consumer)]
pub trait CanConsume {
    fn consume(mut self, extra: u32) -> u32
    where
        Self: Sized + Into<u32>,
    {
        let receiver = &mut self;
        let _ = receiver;
        self.into() + extra
    }
}

#[cgp_impl(new CountByDefault)]
impl Counter {}

#[cgp_impl(new ConsumeByDefault)]
impl Consumer {}

#[cgp_impl(new CountTwice)]
impl Counter {
    fn count_up(&self, mut count: u32, steps: u32) -> u32 {
        count += 2 * steps;
        count
    }
}

pub struct App;

delegate_components! {
    App {
        CounterComponent: CountByDefault,
    }
}

pub struct Tally(u32);

impl From<Tally> for u32 {
    fn from(tally: Tally) -> u32 {
        tally.0
    }
}

delegate_components! {
    Tally {
        ConsumerComponent: ConsumeByDefault,
    }
}

pub struct FastApp;

delegate_components! {
    FastApp {
        CounterComponent: CountTwice,
    }
}

#[test]
fn test_default_method_mut_parameter() {
    assert_eq!(App.count_up(2, 3), 5);
    assert_eq!(FastApp.count_up(2, 3), 8);
    assert_eq!(Tally(1).consume(2), 3);
}
