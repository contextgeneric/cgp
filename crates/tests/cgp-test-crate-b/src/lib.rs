//! Downstream crate for cross-crate CGP coherence tests.
//!
//! Everything here consumes the CGP surface defined in `cgp-test-crate-a`,
//! demonstrating four cross-crate abilities that Rust's coherence rules would
//! otherwise make awkward:
//!
//! 1. wiring a foreign component to a foreign provider on a local context;
//! 2. defining a *local* provider for a *foreign* provider trait (orphan-safe,
//!    because the provider struct is local) and wiring a context to it;
//! 3. participating in a namespace declared upstream;
//! 4. registering a *local* component into an upstream namespace with
//!    `#[default_impl]` (orphan-safe because the crate owns the component key).
//!
//! See crates/tests/AGENTS.md and docs/concepts/coherence.md.

use cgp::prelude::*;
use cgp_test_crate_a::{
    AnnounceLoudly, AnnouncerComponent, AppNamespace, GreetHello, Greeter, GreeterComponent,
    HasName,
};

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

/// (4) A *local* component and provider registered into crate-a's *foreign*
/// `AppNamespace` with `#[default_impl]`. This is orphan-safe because the key —
/// the local `FarewellComponent` — is owned by this crate, even though
/// `AppNamespace` is not: registering a per-component default needs the crate to
/// own either the namespace or the component key. (A `#[prefix]`-ed component,
/// whose key is a foreign `PathCons<..>` path rather than a local marker, could
/// only be registered from the crate that owns the namespace.) `Leaver` then joins
/// `AppNamespace` and resolves the farewell through it, with no direct wiring.
#[cgp_component(Farewell)]
pub trait CanFarewell {
    fn farewell(&self) -> String;
}

#[cgp_impl(new GoodbyeFarewell)]
#[default_impl(FarewellComponent in AppNamespace)]
impl Farewell
where
    Self: HasName,
{
    fn farewell(&self) -> String {
        format!("Goodbye, {}!", self.name())
    }
}

#[derive(HasField)]
pub struct Leaver {
    pub name: String,
}

delegate_components! {
    Leaver {
        namespace AppNamespace;
    }
}

#[cfg(test)]
mod tests {
    use cgp_test_crate_a::{CanAnnounce, CanGreet};

    use super::*;

    #[test]
    fn default_impl_into_upstream_namespace() {
        let leaver = Leaver {
            name: "John".to_owned(),
        };
        assert_eq!(leaver.farewell(), "Goodbye, John!");
    }

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
