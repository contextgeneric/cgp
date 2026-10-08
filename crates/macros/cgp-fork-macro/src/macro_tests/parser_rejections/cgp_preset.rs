//! `cgp_preset!` rejects a body that is not a name, optional parents, and a
//! table. The table reuses the `delegate_components!` entry grammar, so a
//! shorthand written without its value fails here too.

use quote::quote;

use super::assert_macro_rejects;

#[test]
fn rejects_preset_without_body() {
    assert_macro_rejects("cgp_preset without a table body", || {
        crate::macro_lib::cgp_preset(quote!(PersonPreset))
    });
}

#[test]
fn rejects_preset_type_shorthand_without_value() {
    assert_macro_rejects("cgp_preset type shorthand without `=`", || {
        crate::macro_lib::cgp_preset(quote!(
            PersonPreset {
                type Name,
            }
        ))
    });
}
