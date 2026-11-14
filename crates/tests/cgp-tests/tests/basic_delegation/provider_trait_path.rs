//! A provider header that names its provider trait by path names the component key by the
//! same path, so the module needs no import of the key.
//!
//! `#[cgp_impl]`, `#[cgp_provider]`, and `#[cgp_new_provider]` derive the key from the
//! header's trait path, replacing its last segment with `{Trait}Component` and dropping the
//! trait's generic arguments. No module below imports `GreeterComponent` or
//! `ComputerComponent`: each header reaches the key through a path starting with `self::`,
//! `super::`, `crate::`, a crate name, or `::`.
//!
//! See cgp-knowledge-base/cgp/reference/macros/cgp_impl.md.

use cgp::prelude::*;
use cgp_macro_test_util::snapshot_cgp_impl;

pub mod components {
    use cgp::prelude::*;

    #[cgp_component(Greeter)]
    pub trait CanGreet {
        fn greet(&self) -> String;
    }
}

snapshot_cgp_impl! {
    #[cgp_impl(new GreetSelfPath)]
    impl self::components::Greeter {
        fn greet(&self) -> String {
            String::from("self")
        }
    }

    expand_greet_self_path(output) {
        insta::assert_snapshot!(output, @r#"
        impl<__Context__> self::components::Greeter<__Context__> for GreetSelfPath {
            fn greet(__context__: &__Context__) -> String {
                String::from("self")
            }
        }
        impl<__Context__> IsProviderFor<self::components::GreeterComponent, __Context__, ()>
        for GreetSelfPath {}
        ///Provider `GreetSelfPath` for [`Greeter`](self::components::Greeter).
        pub struct GreetSelfPath;
        "#)
    }
}

pub mod super_path {
    use cgp::prelude::*;

    #[cgp_impl(new GreetSuperPath)]
    impl super::components::Greeter {
        fn greet(&self) -> String {
            String::from("super")
        }
    }
}

pub mod crate_path {
    use cgp::prelude::*;

    #[cgp_impl(new GreetCratePath)]
    impl crate::basic_delegation::provider_trait_path::components::Greeter {
        fn greet(&self) -> String {
            String::from("crate")
        }
    }
}

pub mod raw_provider_path {
    use cgp::prelude::*;

    pub struct GreetRawPath;

    #[cgp_provider]
    impl<Context> super::components::Greeter<Context> for GreetRawPath {
        fn greet(_context: &Context) -> String {
            String::from("raw")
        }
    }

    #[cgp_new_provider]
    impl<Context> super::components::Greeter<Context> for GreetNewRawPath {
        fn greet(_context: &Context) -> String {
            String::from("new raw")
        }
    }
}

// A dependency's provider trait, named by path with its generic arguments, once through
// the crate name and once from the root with a leading `::`.
snapshot_cgp_impl! {
    #[cgp_impl(new DoubleByPath)]
    impl<Code> cgp::extra::handler::Computer<Code, u64> {
        type Output = u64;

        fn compute(&self, _code: PhantomData<Code>, input: u64) -> u64 {
            input * 2
        }
    }

    expand_double_by_path(output) {
        insta::assert_snapshot!(output, @"
        impl<__Context__, Code> cgp::extra::handler::Computer<__Context__, Code, u64>
        for DoubleByPath {
            type Output = u64;
            fn compute(__context__: &__Context__, _code: PhantomData<Code>, input: u64) -> u64 {
                input * 2
            }
        }
        impl<
            __Context__,
            Code,
        > IsProviderFor<cgp::extra::handler::ComputerComponent, __Context__, (Code, u64)>
        for DoubleByPath {}
        ///Provider `DoubleByPath` for [`Computer`](cgp::extra::handler::Computer).
        pub struct DoubleByPath;
        ")
    }
}

#[cgp_impl(new TripleByRootPath)]
impl<Code> ::cgp::extra::handler::Computer<Code, u64> {
    type Output = u64;

    fn compute(&self, _code: PhantomData<Code>, input: u64) -> u64 {
        input * 3
    }
}

macro_rules! greeting_app {
    ($app:ident, $provider:ty) => {
        pub struct $app;

        delegate_components! {
            $app {
                components::GreeterComponent: $provider,
            }
        }
    };
}

greeting_app!(SelfPathApp, GreetSelfPath);
greeting_app!(SuperPathApp, super_path::GreetSuperPath);
greeting_app!(CratePathApp, crate_path::GreetCratePath);
greeting_app!(RawPathApp, raw_provider_path::GreetRawPath);
greeting_app!(NewRawPathApp, raw_provider_path::GreetNewRawPath);

pub struct DoubleApp;

delegate_components! {
    DoubleApp {
        cgp::extra::handler::ComputerComponent: DoubleByPath,
    }
}

pub struct TripleApp;

delegate_components! {
    TripleApp {
        cgp::extra::handler::ComputerComponent: TripleByRootPath,
    }
}

#[test]
fn test_provider_trait_path() {
    use cgp::extra::handler::CanCompute;
    use components::CanGreet;

    assert_eq!(SelfPathApp.greet(), "self");
    assert_eq!(SuperPathApp.greet(), "super");
    assert_eq!(CratePathApp.greet(), "crate");
    assert_eq!(RawPathApp.greet(), "raw");
    assert_eq!(NewRawPathApp.greet(), "new raw");
    assert_eq!(DoubleApp.compute(PhantomData::<()>, 21), 42);
    assert_eq!(TripleApp.compute(PhantomData::<()>, 14), 42);
}
