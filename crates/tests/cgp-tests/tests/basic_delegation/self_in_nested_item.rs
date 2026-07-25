//! `self`/`Self` rewriting inside a `#[cgp_impl]` body stops at a nested item.
//!
//! `#[cgp_impl]` rewrites the block's `self` into the context and `Self` into the
//! context type. That rewrite must not reach into an item *nested inside* a method
//! body — a local `struct`, `impl`, or `fn` introduces its own `self`/`Self`
//! scope that names that item, not the enclosing context. Here the provider method
//! defines a local `Wrapper` with its own `Display` impl whose `fmt(&self, …)` and
//! `self.0` refer to `Wrapper`; if the rewrite leaked into the nested impl, its
//! receiver would be turned into a context parameter and the `Display` impl would
//! fail to compile. The outer `self.name()` is still rewritten to the context.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_impl.md (Behavior and corner cases)
//! and cgp-knowledge-base/cgp/reference/macros/cgp_impl.md.

use core::fmt::{self, Display};

use cgp::prelude::*;

#[cgp_auto_getter]
pub trait HasName {
    fn name(&self) -> &str;
}

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> String;
}

#[cgp_impl(new GreetHello)]
impl Greeter
where
    Self: HasName,
{
    fn greet(&self) -> String {
        // A local type with its own `Display` impl. Both the `&self` receiver and
        // the `self.0` field access below belong to `Wrapper`, not the context,
        // and must survive the rewrite untouched.
        struct Wrapper(usize);

        impl Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "({} chars)", self.0)
            }
        }

        // `self.name()` — the outer receiver, rewritten to the context.
        let name = self.name();
        format!("Hello, {} {}", name, Wrapper(name.len()))
    }
}

#[derive(HasField)]
pub struct Person {
    pub name: String,
}

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

#[test]
fn test_self_in_nested_item() {
    let person = Person {
        name: "World".to_owned(),
    };
    assert_eq!(person.greet(), "Hello, World (5 chars)");
}
