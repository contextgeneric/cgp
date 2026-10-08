//! `#[cgp_auto_impl]` moves a trait method body onto the blanket impl and lowers
//! the supertrait into a bound on `__Context__`. The trait itself keeps the
//! method signature with no body, so the blanket impl is the provider.
//!
//! This is the canonical expansion snapshot for `#[cgp_auto_impl]`.
//! See cgp-knowledge-base-fork/cgp/reference/macros/cgp_auto_impl.md.

use cgp_fork_macro::snapshot_cgp_auto_impl;

pub trait HasName {
    fn name(&self) -> &'static str;
}

snapshot_cgp_auto_impl! {
    #[cgp_auto_impl]
    pub trait CanGreet: HasName {
        fn greet(&self) -> String {
            format!("Hello, {}!", self.name())
        }
    }

    expand_can_greet(output) {
        insta::assert_snapshot!(output, @r#"
        pub trait CanGreet: HasName {
            fn greet(&self) -> String;
        }
        impl<__Context__> CanGreet for __Context__
        where
            __Context__: HasName,
        {
            fn greet(&self) -> String {
                format!("Hello, {}!", self.name())
            }
        }
        "#)
    }
}

pub struct Person;

impl HasName for Person {
    fn name(&self) -> &'static str {
        "Ada"
    }
}

#[test]
fn test_auto_impl_greet() {
    assert_eq!(Person.greet(), "Hello, Ada!");
}
