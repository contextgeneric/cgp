//! `#[derive_provider]` only derives `WithProvider`, and only for a
//! `#[cgp_component]` trait that is an abstract type or a single-method getter.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_unknown_provider() {
    assert_macro_rejects(
        "derive_provider with a provider other than WithProvider",
        || {
            cgp_macro_lib::derive_provider(
                quote!(UseContext),
                quote!(
                    #[cgp_component(Greeter)]
                    pub trait CanGreet {
                        fn greet(&self);
                    }
                ),
            )
        },
    );
}

#[test]
fn rejects_trait_without_cgp_component() {
    assert_macro_rejects("derive_provider without #[cgp_component]", || {
        cgp_macro_lib::derive_provider(
            quote!(WithProvider),
            quote!(
                pub trait HasNameType {
                    type Name;
                }
            ),
        )
    });
}

#[test]
fn rejects_component_that_is_not_a_type_or_getter() {
    assert_macro_rejects("derive_provider on a method that is not a getter", || {
        cgp_macro_lib::derive_provider(
            quote!(WithProvider),
            quote!(
                #[cgp_component(Greeter)]
                pub trait CanGreet {
                    fn greet(&self);
                }
            ),
        )
    });
}
