//! A `#[blanket_trait(Ctx)]` carrying an associated constant. The constant's
//! default expression is forwarded into the blanket impl like a method body,
//! and the attribute argument renames the impl's context parameter from the
//! default `__Context__` to `Ctx`.
//!
//! Snapshot variant: blanket trait with an associated constant and a custom
//! context identifier.
//! See cgp-knowledge-base-fork/cgp/reference/macros/blanket_trait.md.

use cgp_fork_macro::snapshot_blanket_trait;

pub trait HasName {
    fn name(&self) -> &str;
}

snapshot_blanket_trait! {
    #[blanket_trait(Ctx)]
    pub trait HasGreeting: HasName {
        const GREETING: &'static str = "Hello";

        fn greet(&self) -> String {
            format!("{}, {}!", Self::GREETING, self.name())
        }
    }

    expand_has_greeting(output) {
        insta::assert_snapshot!(output, @r#"
        pub trait HasGreeting: HasName {
            const GREETING: &'static str = "Hello";
            fn greet(&self) -> String {
                format!("{}, {}!", Self::GREETING, self.name())
            }
        }
        impl<Ctx> HasGreeting for Ctx
        where
            Ctx: HasName,
        {
            const GREETING: &'static str = "Hello";
            fn greet(&self) -> String {
                format!("{}, {}!", Self::GREETING, self.name())
            }
        }
        "#)
    }
}

pub struct Context;

impl HasName for Context {
    fn name(&self) -> &str {
        "Alice"
    }
}

#[test]
fn test_blanket_trait_const() {
    assert_eq!(Context::GREETING, "Hello");
    assert_eq!(Context.greet(), "Hello, Alice!");
}
