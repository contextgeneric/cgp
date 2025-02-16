use cgp_core::prelude::*;

#[cgp_type]
pub trait HasRuntimeType {
    type Runtime;
}
