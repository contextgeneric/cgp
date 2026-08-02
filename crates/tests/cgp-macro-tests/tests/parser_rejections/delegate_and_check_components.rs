//! `delegate_and_check_components!` rejects the check attributes it cannot make
//! sense of: a `#[skip_check]` merged with a `#[check_params]` across a list key
//! and its element, two check attributes on one key, `#[skip_check]` given
//! arguments, an unrecognized per-entry attribute, and a table attribute that is
//! not `#[check_trait]`.
//!
//! The merge case is the one worth pinning: a list key's attribute is combined
//! with each element's rather than overridden by it, so the conflict only exists
//! once the two are merged and cannot be caught by looking at either alone.
//!
//! See cgp-knowledge-base/cgp/reference/macros/delegate_and_check_components.md
//! for the merge rules these enforce.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_skip_check_merged_with_check_params() {
    assert_macro_rejects(
        "delegate_and_check_components merging #[skip_check] with #[check_params]",
        || {
            cgp_macro_lib::delegate_and_check_components(quote!(
                MyApp {
                    #[skip_check]
                    [
                        #[check_params(Rectangle)]
                        AreaCalculatorComponent,
                    ]: ShapeProvider,
                }
            ))
        },
    );
}

#[test]
fn rejects_two_check_attributes_on_one_key() {
    assert_macro_rejects(
        "delegate_and_check_components with two check attributes on one key",
        || {
            cgp_macro_lib::delegate_and_check_components(quote!(
                MyApp {
                    #[check_params(Rectangle)]
                    #[check_params(Circle)]
                    AreaCalculatorComponent: ShapeProvider,
                }
            ))
        },
    );
}

#[test]
fn rejects_skip_check_with_arguments() {
    assert_macro_rejects(
        "delegate_and_check_components with #[skip_check(..)] taking arguments",
        || {
            cgp_macro_lib::delegate_and_check_components(quote!(
                MyApp {
                    #[skip_check(Rectangle)]
                    AreaCalculatorComponent: ShapeProvider,
                }
            ))
        },
    );
}

#[test]
fn rejects_unknown_entry_attribute() {
    assert_macro_rejects(
        "delegate_and_check_components with an unrecognized entry attribute",
        || {
            cgp_macro_lib::delegate_and_check_components(quote!(
                MyApp {
                    #[check_provider(RectangleArea)]
                    AreaCalculatorComponent: ShapeProvider,
                }
            ))
        },
    );
}

#[test]
fn rejects_unknown_table_attribute() {
    // The table accepts only `#[check_trait]`; `#[check_providers]` belongs to
    // the standalone `check_components!` and is refused here rather than ignored.
    assert_macro_rejects(
        "delegate_and_check_components with #[check_providers] on the table",
        || {
            cgp_macro_lib::delegate_and_check_components(quote!(
                #[check_providers(RectangleArea)]
                MyApp {
                    AreaCalculatorComponent: ShapeProvider,
                }
            ))
        },
    );
}
