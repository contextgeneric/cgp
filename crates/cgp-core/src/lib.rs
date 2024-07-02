#![no_std]

pub mod prelude;

pub use cgp_async::{async_trait, Async};
pub use {cgp_component as component, cgp_error as error, cgp_inner as inner, cgp_run as run};
