//! `self` rewriting inside a `#[cgp_impl]` body distinguishes a value receiver
//! from a `self::` module path.
//!
//! `#[cgp_impl]` rewrites `self` into the context, and inside a macro invocation
//! it must do so at the token level because `VisitMut` cannot see through a
//! `macro!( … )`. The rewrite has to tell the two meanings of `self` apart: a
//! bare `self` value (here `self.name()`) becomes the context, while a `self::`
//! module path (here `self::greeting::SUFFIX`) is the current module and must be
//! left untouched. A value `self` is never followed by `::`, so the trailing
//! `::` is what disambiguates the path form.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_impl.md and
//! cgp-knowledge-base/cgp/reference/macros/cgp_impl.md.

use cgp::prelude::*;

mod greeting {
    pub const SUFFIX: &str = "!";
}

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
        // `self.name()` — value receiver, rewritten to the context.
        // `self::greeting::SUFFIX` — module path, left intact.
        format!("Hello, {}{}", self.name(), self::greeting::SUFFIX)
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
fn test_self_in_macro() {
    let person = Person {
        name: "World".to_owned(),
    };
    assert_eq!(person.greet(), "Hello, World!");
}
