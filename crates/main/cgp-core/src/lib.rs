#![no_std]
#![allow(mixed_script_confusables)]

pub mod prelude;

pub use prelude as macro_prelude;
#[doc(inline)]
pub use {
    cgp_async_macro::async_trait, cgp_component as component, cgp_error as error,
    cgp_field as field, cgp_macro as macros, cgp_type as types,
};
