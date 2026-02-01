use core::fmt::Display;

use cgp::prelude::*;

#[cgp_auto_getter]
pub trait HasName {
    type Name: Display;

    fn name(&self) -> &Self::Name;
}
