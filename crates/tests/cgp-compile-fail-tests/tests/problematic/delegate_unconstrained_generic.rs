//! Problematic failure: a per-entry generic list on a `delegate_components!`
//! mapping whose parameter appears only in the *provider value* and not in the
//! *key*. The macro faithfully lowers `<T> GreeterComponent: GreetWith<T>` into
//! `impl<T> DelegateComponent<GreeterComponent> for Person { type Delegate =
//! GreetWith<T>; }`, where `T` is constrained by neither the trait, the self
//! type, nor a predicate — so the compiler rejects it with E0207.
//!
//! A per-entry generic is only well-formed when it appears in the key (as in
//! `<T2> BazKey<T1, T2>: BarValue<T1>`, where `DelegateComponent<BazKey<..>>`
//! binds it). The macro does not check that every declared generic reaches the
//! key, so it accepts this nonsensical entry and emits an impl with a free
//! parameter instead of rejecting it with a spanned error at macro time.
//!
//! The correct behavior would be to reject a per-entry generic that does not
//! appear in the key. This fixture pins the current behavior; its `.stderr`
//! should improve when the defect is fixed.
//!
//! See docs/implementation/entrypoints/delegate_components.md (Known issues).

use core::marker::PhantomData;

use cgp::prelude::*;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self);
}

pub struct GreetWith<T>(pub PhantomData<T>);

#[cgp_provider]
impl<Context, T> Greeter<Context> for GreetWith<T> {
    fn greet(_context: &Context) {}
}

pub struct Person;

// `T` is declared for the entry but only used in the value `GreetWith<T>`.
delegate_components! {
    Person {
        <T> GreeterComponent: GreetWith<T>,
    }
}

fn main() {}
