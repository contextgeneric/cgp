//! A `#[cgp_impl]` provider may take an owned, mutable receiver `mut self`.
//!
//! The receiver rewrite has to place `mut` on the generated parameter *binding*
//! (`mut __context__: Context`), not on the type: `mut` binds the parameter,
//! while `__context__: mut Context` is not a valid type. This file exercises the
//! owned-mutable receiver so the rewrite of every receiver shape — `&self`,
//! `&mut self`, `self`, and `mut self` — stays covered.
//!
//! See docs/implementation/entrypoints/cgp_impl.md and
//! docs/reference/macros/cgp_impl.md.

use cgp::prelude::*;

#[cgp_component(Consumer)]
pub trait CanConsume {
    fn consume(self) -> String;
}

#[cgp_impl(new ResetAndConsume)]
impl Consumer
where
    Self: Sized + Default,
{
    fn consume(mut self) -> String {
        // `core::mem::take` needs `&mut self`, so the receiver must be `mut`.
        drop(core::mem::take(&mut self));
        "consumed".to_owned()
    }
}

#[derive(HasField, Default)]
pub struct Person {
    pub name: String,
}

delegate_components! {
    Person {
        ConsumerComponent: ResetAndConsume,
    }
}

#[test]
fn test_mut_owned_receiver() {
    let person = Person {
        name: "World".to_owned(),
    };
    assert_eq!(person.consume(), "consumed");
}
