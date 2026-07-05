//! Acceptable failure: a per-entry generic list on a `delegate_components!`
//! mapping whose parameter appears only in the *provider value* and not in the
//! *key*. The macro faithfully lowers `<T> GreeterComponent: GreetWith<T>` into
//! `impl<T> DelegateComponent<GreeterComponent> for Person { type Delegate =
//! GreetWith<T>; }`, where `T` is constrained by neither the trait, the self
//! type, nor a predicate — so the compiler rejects it with E0207.
//!
//! A per-entry generic is only well-formed when it appears in the key (as in
//! `<T2> BazKey<T1, T2>: BarValue<T1>`, where `DelegateComponent<BazKey<..>>`
//! binds it). Writing one that never reaches the key is ill-formed input, and
//! the macro lowers it faithfully rather than second-guessing it — so `rustc`
//! rejects the unconstrained parameter with exactly the E0207 it would give a
//! hand-written `impl<T>` with an unused parameter. Deferring this to the
//! compiler is the intended behavior, not a macro defect.
//!
//! See docs/errors/wiring/unconstrained-generic.md.

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
