use cgp::prelude::*;

#[cgp_getter]
pub trait HasName {
    type Name;

    fn name(&self) -> &Self::Name;
}
