//! An `async` method with a default body under `#[async_trait]`.
//!
//! The attribute rewrites the method to return `impl Future`, and its default body
//! becomes that future: `async move { … }` around the body, with each by-value
//! parameter rebound inside it. The future therefore owns the parameters and drops
//! them when it completes, as an `async fn` does, and a `mut` parameter is mutable
//! inside. A component's default body reaches its provider trait the same way, so
//! an empty provider inherits it.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/async_trait.md.

use core::sync::atomic::{AtomicBool, Ordering};

use cgp::prelude::*;
use futures::executor::block_on;

pub struct Guard<'a>(&'a AtomicBool);

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

#[async_trait]
pub trait CanCount {
    async fn add(&self, mut value: u32, extra: u32) -> u32 {
        value += extra;
        value
    }

    async fn hold(&self, _guard: Guard<'_>) {}
}

pub struct Counter;

impl CanCount for Counter {}

#[cgp_component(Greeter)]
#[async_trait]
pub trait CanGreet {
    async fn greet(&self, name: &str) -> String {
        format!("Hello, {name}!")
    }
}

#[cgp_impl(new GreetByDefault)]
impl Greeter {}

pub struct App;

delegate_components! {
    App {
        GreeterComponent: GreetByDefault,
    }
}

#[test]
fn test_default_body_runs_as_future() {
    assert_eq!(block_on(Counter.add(2, 3)), 5);
    assert_eq!(block_on(App.greet("World")), "Hello, World!");
}

#[test]
fn test_default_body_owns_its_parameters() {
    let dropped = AtomicBool::new(false);

    let future = Counter.hold(Guard(&dropped));
    assert!(!dropped.load(Ordering::SeqCst));

    block_on(future);
    assert!(dropped.load(Ordering::SeqCst));
}
