//! A namespace body reuses the `delegate_components!` entry grammar, which
//! supports no attributes, so `cgp_namespace!` rejects any attribute it finds on
//! an entry — matching `delegate_components!` rather than silently parsing and
//! discarding it. It rejects an attribute on a `:` mapping key, on a `=>` redirect
//! key, and on a key inside a `for <..> in ..` loop.
//!
//! See cgp-knowledge-base/cgp/implementation/entrypoints/cgp_namespace.md (Tests) for these failure
//! cases, and cgp-knowledge-base/cgp/reference/macros/cgp_namespace.md for the user-facing
//! semantics.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_attribute_on_mapping_key() {
    assert_macro_rejects("cgp_namespace with an attribute on a mapping key", || {
        cgp_macro_lib::cgp_namespace(quote!(
            new MyNamespace {
                #[foo]
                [String, u64]: ShowWithDisplay,
            }
        ))
    });
}

#[test]
fn rejects_attribute_on_redirect_key() {
    assert_macro_rejects("cgp_namespace with an attribute on a redirect key", || {
        cgp_macro_lib::cgp_namespace(quote!(
            new MyNamespace {
                #[foo]
                FooProviderComponent => @MyFooComponent,
            }
        ))
    });
}

#[test]
fn rejects_attribute_on_for_loop_key() {
    // The attribute is on a key inside the `for <..> in ..` loop body. The
    // validator recurses through the statement forms, so this is rejected rather
    // than parsed and discarded.
    assert_macro_rejects("cgp_namespace with an attribute on a for-loop key", || {
        cgp_macro_lib::cgp_namespace(quote!(
            new MyNamespace {
                for <T, Provider> in SomeTable {
                    #[foo]
                    @test.ShowImplComponent.T: Provider,
                }
            }
        ))
    });
}
