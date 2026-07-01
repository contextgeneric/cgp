//! Downstream crate for cross-crate CGP coherence tests.
//!
//! Everything here consumes the CGP surface defined in `cgp-test-crate-a`,
//! demonstrating three cross-crate abilities that Rust's coherence rules would
//! otherwise make awkward:
//!
//! 1. wiring a foreign component to a foreign provider on a local context;
//! 2. defining a *local* provider for a *foreign* provider trait (orphan-safe,
//!    because the provider struct is local) and wiring a context to it;
//! 3. participating in a namespace declared upstream.
//!
//! See crates/tests/CLAUDE.md and docs/concepts/coherence.md.

use cgp::prelude::*;
use cgp_test_crate_a::{AnnounceLoudly, AnnouncerComponent, GreetHello, Greeter, GreeterComponent};

/// (1) A local context wires the foreign `Greeter` component to the foreign
/// `GreetHello` provider. `GreetHello` needs `HasName`, satisfied by the `name`
/// field through crate-a's auto getter.
#[derive(HasField)]
pub struct Person {
    pub name: String,
}

delegate_components! {
    Person {
        GreeterComponent: GreetHello,
    }
}

/// (2) A provider defined in *this* crate for the foreign `Greeter` provider
/// trait. This is allowed because `GreetGoodbye` is local, even though `Greeter`
/// is not — the coherence win that CGP's provider structs are built for.
#[cgp_impl(new GreetGoodbye)]
impl Greeter
where
    Self: cgp_test_crate_a::HasName,
{
    fn greet(&self) -> String {
        format!("Goodbye, {}!", self.name())
    }
}

#[derive(HasField)]
pub struct FormalPerson {
    pub name: String,
}

delegate_components! {
    FormalPerson {
        GreeterComponent: GreetGoodbye,
    }
}

/// (3) A local context participates in crate-a's `@app` namespace, wiring the
/// upstream `Announcer` component through the namespace path.
#[derive(HasField)]
pub struct Broadcaster {
    pub name: String,
}

delegate_components! {
    Broadcaster {
        namespace DefaultNamespace;

        @app.AnnouncerComponent: AnnounceLoudly,
    }
}

#[cfg(test)]
mod tests {
    use cgp_test_crate_a::{CanAnnounce, CanGreet};

    use super::*;

    #[test]
    fn wire_foreign_provider() {
        let person = Person {
            name: "John".to_owned(),
        };
        assert_eq!(person.greet(), "Hello, John!");
    }

    #[test]
    fn local_provider_for_foreign_component() {
        let person = FormalPerson {
            name: "John".to_owned(),
        };
        assert_eq!(person.greet(), "Goodbye, John!");
    }

    #[test]
    fn participate_in_upstream_namespace() {
        let broadcaster = Broadcaster {
            name: "John".to_owned(),
        };
        assert_eq!(broadcaster.announce(), "ANNOUNCEMENT from John!");
    }
}
