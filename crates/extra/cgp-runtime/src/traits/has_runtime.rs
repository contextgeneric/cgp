use cgp::prelude::*;

use crate::HasRuntimeType;

#[cgp_getter]
pub trait HasRuntime: HasRuntimeType {
    fn runtime(&self) -> &Self::Runtime;
}
