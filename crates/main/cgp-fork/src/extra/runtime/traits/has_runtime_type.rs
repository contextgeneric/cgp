use crate::core::prelude::*;

#[cgp_type]
pub trait HasRuntimeType {
    type Runtime;
}

pub type RuntimeOf<Context> = <Context as HasRuntimeType>::Runtime;
