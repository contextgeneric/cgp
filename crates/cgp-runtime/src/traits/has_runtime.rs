use cgp_core::prelude::*;

use crate::HasRuntimeType;

#[cgp_getter]
pub trait HasRuntime: HasRuntimeType {
    fn runtime(&self) -> &Self::Runtime;
}
