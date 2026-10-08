//! A `#[helper]` method inside `#[cgp_impl]` is not a provider-trait method.
//! It is parsed out before the provider rewrite and emitted as its own blanket
//! trait, still in consumer form (`&self`). `replace_self` runs only on the
//! real provider method, which calls the helper through the context value.
//!
//! See cgp-knowledge-base-fork/cgp/implementation/entrypoints/cgp_impl.md and
//! cgp-knowledge-base-fork/cgp/reference/macros/cgp_impl.md.

use cgp_fork::prelude::*;
use cgp_fork_macro::snapshot_cgp_impl;

#[cgp_component(Greeter)]
pub trait CanGreet {
    fn greet(&self) -> &'static str;
}

snapshot_cgp_impl! {
    #[cgp_impl(new GreetShout)]
    impl Greeter {
        #[helper]
        fn shout(&self) -> &'static str {
            "HELLO"
        }

        fn greet(&self) -> &'static str {
            self.shout()
        }
    }

    expand_greet_shout(output) {
        insta::assert_snapshot!(output, @r#"
        #[allow(non_camel_case_types)]
        trait __Impl_GreetShout__ {
            fn shout(&self) -> &'static str;
        }
        impl<__Context__> __Impl_GreetShout__ for __Context__ {
            fn shout(&self) -> &'static str {
                "HELLO"
            }
        }
        impl<__Context__> Greeter<__Context__> for GreetShout {
            fn greet(__context__: &__Context__) -> &'static str {
                __context__.shout()
            }
        }
        impl<__Context__> IsProviderFor<GreeterComponent, __Context__, ()> for GreetShout {}
        pub struct GreetShout;
        "#)
    }
}

pub struct App;

delegate_components! {
    App {
        GreeterComponent: GreetShout,
    }
}

#[test]
fn test_helper_method() {
    assert_eq!(App.greet(), "HELLO");
}
