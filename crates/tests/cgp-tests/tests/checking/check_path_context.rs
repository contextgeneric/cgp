//! `check_components!` accepts a path-qualified context type such as
//! `inner::Context`, deriving the check trait name from the final path segment
//! (`__CheckContext`). This mirrors `delegate_components!`, which uses the context
//! type verbatim, so a context defined in another module can be wired and checked
//! by its path without importing it. This concept owns the macro's expansion
//! snapshot.
//!
//! See docs/reference/macros/check_components.md and
//! docs/implementation/entrypoints/check_components.md.

use cgp_macro_test_util::snapshot_check_components;

pub mod inner {
    use cgp::prelude::*;

    #[cgp_auto_getter]
    pub trait HasName {
        fn name(&self) -> &str;
    }

    #[cgp_component(Greeter)]
    pub trait CanGreet {
        fn greet(&self);
    }

    #[cgp_impl(new GreetHello)]
    impl Greeter
    where
        Self: HasName,
    {
        fn greet(&self) {
            let _ = self.name();
        }
    }

    #[derive(HasField)]
    pub struct Context {
        pub name: String,
    }

    // Plain wiring: the `delegate_components!` expansion is snapshotted in
    // `basic_delegation`, so we invoke it plainly here.
    delegate_components! {
        Context {
            GreeterComponent: GreetHello,
        }
    }
}

snapshot_check_components! {
    check_components! {
        inner::Context {
            inner::GreeterComponent,
        }
    }

    expand_check_path_context(output) {
        insta::assert_snapshot!(output, @"
        trait __CheckContext<
            __Component__,
            __Params__: ?Sized,
        >: CanUseComponent<__Component__, __Params__> {}
        impl __CheckContext<inner::GreeterComponent, ()> for inner::Context {}
        ")
    }
}
