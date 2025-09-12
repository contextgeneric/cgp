#![no_std]
#![allow(mixed_script_confusables)]

pub mod prelude;

#[doc(inline)]
pub use {
    cgp_async_macro::async_trait, cgp_component as component, cgp_error as error,
    cgp_field as field, cgp_macro as macros, cgp_macro::re_export_imports, cgp_type as types,
};
