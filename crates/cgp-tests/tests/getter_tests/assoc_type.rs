use cgp::prelude::*;

#[cgp_auto_getter]
pub trait HasName {
    type Name;

    fn name(&self) -> &Self::Name;
}
