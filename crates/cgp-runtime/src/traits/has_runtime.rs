use cgp_core::prelude::*;

use crate::HasRuntimeType;

#[cgp_getter {
    provider: RuntimeGetter,
}]
pub trait HasRuntime: HasRuntimeType {
    fn runtime(&self) -> &Self::Runtime;
}
