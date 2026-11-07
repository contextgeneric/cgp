//! Builds the `cgp` crate for a target without `std`.
//!
//! The crate is `#![no_std]` and uses the macros, so a build for a target that has no `std` fails if
//! any crate in the tree needs it. The `alloc` feature turns on the providers that need an
//! allocator, so the same crate checks both paths:
//!
//! ```sh
//! cargo check -p cgp-no-std-tests --target x86_64-unknown-none
//! cargo check -p cgp-no-std-tests --features alloc --target x86_64-unknown-none
//! ```
//!
//! See cgp-knowledge-base/cgp/implementation/README.md and the `no_std` inventory in the campaign
//! records.

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

use cgp::prelude::*;

/// A component, so the build exercises the component macros.
#[cgp_component(Greeter)]
pub trait CanGreet {
    /// Greets.
    fn greet(&self) -> &'static str;
}

/// A provider for the component.
#[cgp_impl(new GreetHello)]
impl Greeter {
    fn greet(&self) -> &'static str {
        "Hello"
    }
}

/// A function-style trait, so the build exercises `#[cgp_fn]`.
#[cgp_fn]
pub fn double_count(&self, #[implicit] count: u32) -> u32 {
    count * 2
}

/// A `&str` implicit argument, which reads any `AsRef<str>` field and so needs no
/// allocator.
#[cgp_fn]
pub fn greet_name(&self, #[implicit] name: &str) -> &str {
    name
}

/// An `Option<&str>` implicit argument, which reads any `Option<U>` whose `U`
/// implements `AsRef<str>` through `OptionAsRef`, so it needs no allocator either.
#[cgp_fn]
pub fn greet_nickname(&self, #[implicit] nickname: Option<&str>) -> Option<&str> {
    nickname
}

/// A context that wires the component.
pub struct App;

delegate_components! {
    App {
        GreeterComponent: GreetHello,
    }
}

check_components! {
    App {
        GreeterComponent,
    }
}

/// A type-level string, so the build exercises the type-level primitives.
pub type NameTag = Symbol!("name");

/// A provider that needs an allocator, compiled only with the `alloc` feature.
#[cfg(feature = "alloc")]
#[cgp_impl(new GreetWithAlloc)]
impl Greeter {
    fn greet(&self) -> &'static str {
        let greeting = alloc::string::String::from("Hello");
        alloc::boxed::Box::leak(greeting.into_boxed_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_context_greets() {
        assert_eq!(App.greet(), "Hello");
    }
}
